macro_rules! git {
    ($($arg:expr),*) => {{
        let mut cmd = std::process::Command::new("git");
        cmd$(.arg($arg))* ;
        cmd
    }};
}

mod app;
mod config;
mod data;
mod git;
mod prelude;
mod shell;

use prelude::*;

/// Gets the git branch, and if we're currently in detached HEAD state, it will
/// print HEAD.
fn get_git_branch() -> Result<String, ()> {
    let output = git!("rev-parse", "--abbrev-ref", "--symbolic-full-name", "HEAD")
        .output()
        .map_err(|_| eprintln!("Failed to execute shell command to get git branch."))?;
    let mut output = String::from_utf8(output.stdout)
        .map_err(|_| eprintln!("Failed to parsed git branch"))?;
    output.truncate(output.as_str().trim_end().len());
    Ok(output)
}

fn get_git_branches() -> Result<String, ()> {
    let output = git!("branch", "--format=%(refname:short)")
        .output()
        .map_err(|_| eprintln!("Failed to execute shell command to get git branches."))?;
    let mut output = String::from_utf8(output.stdout)
        .map_err(|_| eprintln!("Failed to parsed git branches"))?;
    output.truncate(output.as_str().trim_end().len());
    Ok(output)
}

fn get_git_worktrees() -> Result<String, ()> {
    let output = git!("worktree", "list", "--porcelain").output().map_err(|_| {
        eprintln!("Failed to execute shell command to get git worktrees.")
    })?;
    String::from_utf8(output.stdout)
        .map_err(|_| eprintln!("Failed to parsed git worktrees"))
}

fn try_main(goal: &str) -> Result<ExitCode, ()> {
    let mut git_branch_output: Result<String, ()> = Err(());
    let mut git_branches_output: Result<String, ()> = Err(());
    let mut git_worktree_output: Result<String, ()> = Err(());
    let mut cwd: Result<PathBuf, ()> = Err(());
    let mut raw_config: String = String::new();

    rayon::scope(|scope| {
        scope.spawn(|_| git_branch_output = get_git_branch());
        scope.spawn(|_| git_branches_output = get_git_branches());
        scope.spawn(|_| git_worktree_output = get_git_worktrees());
        scope.spawn(|_| {
            cwd = std::env::current_dir()
                .map_err(|_| eprintln!("Unable to get current dir"))
        });
        scope.spawn(|_| raw_config = Config::read());
    });
    let cwd = cwd?;
    let git_branch_output = git_branch_output?;
    let git_branches_output = git_branches_output?;
    let git_worktree_output = git_worktree_output?;

    let git_branches = git_branches_output.lines().map(Branch::new).collect::<Vec<_>>();
    let mut config = Config::parse(&raw_config).unwrap();

    log::info!("cwd = {cwd:?}");
    log::info!("branch: {git_branch_output:?}");
    log::info!("branches: {git_branches:?}");
    log::info!("worktrees:\n---\n{git_worktree_output}\n---");

    log::info!("config: {config:?}");

    let worktrees = git::Bundle::parse_all(git_worktree_output.as_str());
    for (i, wt) in worktrees.iter().enumerate() {
        log::info!("[{i}] {wt:?}")
    }

    let mut read_buf = String::new();
    for branch in &git_branches {
        if let None = config.get(branch) {
            println!("Branch {branch} is not mapped to any worktree.");
            for (idx, bundle) in worktrees.iter().enumerate() {
                println!("[{idx}] {}", bundle.worktree.as_str())
            }
            print!("Pick one > ");
            _ = io::stdout().flush();
            read_buf.clear();
            io::stdin().read_line(&mut read_buf).unwrap();
            let choice = read_buf.trim().parse::<usize>().unwrap();
            let choice = &worktrees[choice];
            config.set(*branch, choice.worktree);
        }
    }

    Ok(ExitCode::SUCCESS)
}

fn main() -> std::process::ExitCode {
    log::init(Some(log::LevelFilter::Trace));

    // To keep things simple, we only run the complicated logic when there is
    // exactly 1 CLI argument (that is not the binary itself).
    let args: Vec<_> = env::args_os().skip(1).collect();
    let 1 = args.len() else {
        let mut cmd = Command::new("git");
        cmd.arg("checkout");
        cmd.args(args);
        shell::run(cmd);
    };
    let Some(goal) = args[0].to_str() else {
        eprintln!("Failed to decode target.");
        return std::process::ExitCode::FAILURE;
    };
    let pool = rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap();
    match pool.install(|| try_main(goal.trim())) {
        Ok(v) => v.exit(),
        Err(()) => return std::process::ExitCode::FAILURE,
    }
}
