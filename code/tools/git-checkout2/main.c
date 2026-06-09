// STDOUT is reserved for printing the target directory. Always try to exit with
// the same exit code of the underlying `git checkout` command.

// #define DEBUG_MODE

#include <log.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>

#define GIT_CHECKOUT_BUF_MAX 512
#define GIT_WORKTREE_BUF_MAX 2048

// Ban list.
// 1. printf: in production we want zero allocations.
#define BANNED(func) sorry__##func##__is_a_banned_function
#undef printf
#define printf(...) BANNED(strcpy)

#define STARTS_WITH(haystack, prefix)                                          \
  (strncmp(haystack, prefix, sizeof(prefix) - 1) == 0)

#define FATAL_NOT_GIT_REPO "fatal: not a git repository"
#define IS_ALREADY_USED_AT "is already used by worktree at"

#define MAX(A, B) (A < B ? B : A)
#define MIN(A, B) (A < B ? A : B)

#define ERR(msg) write(STDERR_FILENO, msg, sizeof(msg));
#define ERRLN(msg) ERR(msg), write(STDERR_FILENO, "\n", 1);

static char *GIT;

// Set the `GIT` variable.
static inline void setup_git_binary() {
  GIT = getenv("GIT");
  if (GIT == NULL) {
    GIT = "git";
  }
}

struct pipedata {
  int fd[2];
  pid_t pid;
};

#define PIPE_OR_RETURN(fd)                                                     \
  if (pipe(fd) == -1) {                                                        \
    perror("pipe failed");                                                     \
    return 1;                                                                  \
  }
#define FORK_OR_RETURN(pid)                                                    \
  if ((pid = fork()) == -1) {                                                  \
    perror("fork failed");                                                     \
    return 1;                                                                  \
  }

// Returns 64 if this function's stdout output is meant to be taken as the
// target directory for the `cd` command.
int main(int argc, char *argv[]) {
#ifdef DEBUG_MODE
  log_set_level(LOG_TRACE);
  log_set_quiet(1);
  FILE *log_file = fopen("/home/khang/.local/state/git-checkout2.log", "w");
  log_add_fp(log_file, LOG_TRACE);
  log_trace("Begin execution");
#endif

  setup_git_binary();
  log_trace("GIT = \"%s\"", GIT);

  // If the number of arguments (excluding the command) is not 1, then we revert
  // to original `git checkout` behaviour. This ensures that there is simplicity
  // in the code following this.
  if (argc != 2) {
    log_trace("Has complex CLI arguments (%d). Bypassing.", argc);
    char *argv2[argc + 2]; // +1 for "checkout", +1 for NULL.
    argv2[0] = GIT;
    argv2[1] = "checkout";
    memcpy(&argv2[2], &argv[1], sizeof(char *) * (argc - 1));
    argv2[argc + 1] = NULL;
    return execvp(GIT, argv2);
  } else {
    log_trace("Indeed has only one CLI argument.");
  }

  // Since there's only one CLI argument, we shall call it GOAL.
#define GOAL argv[1]

  // Pipe data for `git checkout`.
  struct pipedata pd_c;
  PIPE_OR_RETURN(pd_c.fd);
  FORK_OR_RETURN(pd_c.pid);

  /* Child process: `git checkout` */
  if (pd_c.pid == 0) {
    // Capture `git checkout` STDERR.
    dup2(pd_c.fd[1], STDERR_FILENO);
    // Don't send anything to STDOUT. Remember, STDOUT is reserved for printing
    // the GOAL's directory.
    close(STDOUT_FILENO);
    close(pd_c.fd[0]), close(pd_c.fd[1]);
    execlp(GIT, GIT, "checkout", GOAL, NULL);
  } else {
    close(pd_c.fd[1]);
  }

  int git_checkout_exit_code;
  waitpid(pd_c.pid, &git_checkout_exit_code, 0);
  git_checkout_exit_code >>= 8; // Get the high bits.

  /// `git checkout` byte buffer.
  char buf_c[GIT_CHECKOUT_BUF_MAX];

  // By construction, buf_c_len < GIT_CHECKOUT_BUF_MAX.
  const int buf_c_len = read(pd_c.fd[0], buf_c, GIT_CHECKOUT_BUF_MAX - 1);
  log_trace("Bytes read from `git checkout`: %d", buf_c_len);
  buf_c[buf_c_len] = '\0';
  if (buf_c_len + 3 == GIT_CHECKOUT_BUF_MAX) {
    ERR("Warning: possibly missing bytes from `git ");
    ERR("checkout");
    ERR("` output due to insufficient buffer size.");
  }

  log_trace("GIT CHECKOUT STDERR: \"%s\"", buf_c);

  // If all goes well, just reflect `git checkout's` stderr output back to the
  // terminal's stderr.
  if (git_checkout_exit_code == 0) {
    write(STDERR_FILENO, buf_c, buf_c_len);
    log_trace("Everything went well, nothing to do anymore.");
    return git_checkout_exit_code;
  }

  // If it turns out we're not even in a git repository, then exit early.
  if (STARTS_WITH(buf_c, FATAL_NOT_GIT_REPO)) {
    write(STDERR_FILENO, buf_c, buf_c_len);
    log_trace("Not in a git repo, exit code %d", git_checkout_exit_code);
    return git_checkout_exit_code;
  }

  // `git checkout` sometimes would override local changes, in which case the
  // error message is:
  // ---------------------------------------------------------------------------
  // error: Your local changes to the following files would be overwritten by
  // checkout:
  //   ...
  // Please commit your changes or stash them before you switch branches.
  // Aborting
  if (strncmp(buf_c, "error: Your local changes t", 27) == 0) {
    write(STDERR_FILENO, buf_c, buf_c_len);
    log_trace("Your local changes (exit code: %d)", git_checkout_exit_code);
    return git_checkout_exit_code;
  }

  char *c_left, *c_right, *c_line;

  // Now this is where `git-checkout2` shines.
  //
  // If we see that this branch is already used at another worktree, we print
  // the directory to that worktree to STDOUT so the parent shell can go there.
  if (strncmp(buf_c, FATAL_NOT_GIT_REPO, 6) == 0 &&
      ((c_left = strstr(buf_c, IS_ALREADY_USED_AT)) != NULL)) {
    if ((c_left = strchr(c_left + 30, '\'')) == NULL ||
        (c_right = strchr(++c_left, '\'')) == NULL) {
      ERR("Parsing error at \"");
      ERR(IS_ALREADY_USED_AT);
      ERR("...\"");
    }
    // OUTPUT ==========
    write(STDOUT_FILENO, c_left, c_right - c_left);
    return 64;
  }

  /* Now, we fallback to `git worktree output`. */

  // Pipe data for `git worktree`.
  struct pipedata pd_w;
  PIPE_OR_RETURN(pd_w.fd);
  FORK_OR_RETURN(pd_w.pid);

  /* Child process: `git worktree` */
  if (pd_w.pid == 0) { //
    // Capture `git worktree` STDOUT.
    dup2(pd_w.fd[1], STDOUT_FILENO);
    // Don't listen to stderr. We shall not parse that at all.
    close(STDERR_FILENO);
    close(pd_w.fd[0]), close(pd_w.fd[1]);
    execlp(GIT, GIT, "worktree", "list", "--porcelain", NULL);
  } else {
    close(pd_w.fd[1]);
  }

  waitpid(pd_w.pid, NULL, 0);

  /// `git worktree` byte buffer.
  char buf_w[GIT_WORKTREE_BUF_MAX];

  // By construction, buf_w_len < GIT_WORKTREE_BUF_MAX.
  const int buf_w_len = read(pd_w.fd[0], buf_w, GIT_WORKTREE_BUF_MAX - 1);
  log_trace("Bytes read from `git worktree`: %d", buf_w_len);
  buf_w[buf_w_len] = '\0';

  if (buf_w_len + 3 == GIT_WORKTREE_BUF_MAX) {
    ERR("Warning: possibly missing bytes from `git ");
    ERR("worktree");
    ERR("` output due to insufficient buffer size.");
  }

  const int goal_len = strlen(GOAL);

  for (c_right = buf_w; (c_line = strsep(&c_right, "\n"));) {
    if (!STARTS_WITH(c_line, "worktree")) {
      continue;
    }
    c_line += 9;
    log_trace("(worktree) %s", c_line);
    // Check to see if the current line ends with the goal.
    if ((c_left = c_right - goal_len - 1) < c_line) {
      continue;
    }
    if (strncmp(c_left, GOAL, goal_len) == 0) {
      // OUTPUT ==========
      write(STDOUT_FILENO, c_line, c_right - c_line - 1);
      return 64;
    }
  }

  log_trace("Target not found. git-checkout2 is unable to help.");
  write(STDERR_FILENO, buf_c, buf_c_len);
  return git_checkout_exit_code;
}
