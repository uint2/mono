const std = @import("std");

const X = @import("x11.zig");
const E = @import("errors.zig");
const EM = X.eventMask;
const C = @import("c_lib.zig").C;

/// Ensures that there are no other window managers currently running.
///
// (dwm) static void checkotherwm(void);
pub fn checkOtherWM(dpy: *X.Display) void {
    E.xerrorlib = X.XSetErrorHandler(E.xerrorstart);
    // this causes an error if some other window manager is running
    X.XSelectInput(dpy, X.DefaultRootWindow(dpy), EM.SubstructureRedirectMask);
    X.XSync(dpy, false);
    _ = X.XSetErrorHandler(E.xerror);
    X.XSync(dpy, false);
}

/// Do not let children turn into zombies when they terminate.
pub fn terminationHandling() void {
    var sa: C.struct_sigaction = undefined;

    // Initialize and empty a signal set.
    _ = C.sigemptyset(&sa.sa_mask);

    // source: https://man7.org/linux/man-pages/man2/sigaction.2.html
    sa.sa_flags =
        // Do not receive notification when child processes stop or resume.
        C.SA_NOCLDSTOP |
        // Do not transform children into zombies when they terminate.
        //
        // If the SA_NOCLDWAIT flag is set when establishing a handler for
        // SIGCHLD, POSIX.1 leaves it unspecified whether a SIGCHLD signal is
        // generated when a child process terminates.  On Linux, a SIGCHLD
        // signal is generated in this case; on some other implementations, it
        // is not.
        C.SA_NOCLDWAIT |
        // Provide behavior compatible with BSD signal semantics by making
        // certain system calls restartable across signals. This flag is
        // meaningful only when establishing a signal handler.
        C.SA_RESTART;

    // sa_handler specifies the action to be associated with _signum_ and can be
    // one of the following:
    //   *  SIG_DFL for the default action.
    //   *  SIG_IGN to ignore this signal.
    //   *  A pointer to a signal handling function.  This function receives
    //      the signal number as its only argument.
    sa.__sigaction_handler.sa_handler = C.SIG_IGN;

    // The sigaction() system call is used to change the action taken by a
    // process on receipt of a specific signal.
    _ = C.sigaction(C.SIGCHLD, &sa, null);
}

/// Clean up any zombies (inherited from .xinitrc etc).
///
/// pid=-1 means to wait for any child process.
///
/// On success, `waitpid` returns the pid of the child whose state has
/// changed; if WNOHANG was specified and one or more child(ren) specified
/// by pid exist, but have not yet changed state, then 0 is returned. On
/// failure, -1 is returned.
///
/// docs: https://man7.org/linux/man-pages/man2/waitpid.2.html
pub fn clearZombies() void {
    while (std.c.waitpid(-1, null, std.c.W.NOHANG) > 0) {}
}

// (dwm) scan
// pub fn scan(z: *App, allocator: Allocator) error{OutOfMemory}!void {
//     var wa: X.XWindowAttributes = undefined;
//     var i: c_uint = undefined;
//     var d1: X.Window = undefined;
//     var d2: X.Window = undefined;
//
//     // No need to call XFree because null in Zig means NULL in C.
//     const wins: []X.Window = X.XQueryTree(z.dpy, z.root, &d1, &d2) orelse return;
//     defer X.XFree(wins.ptr);
//
//     // Note: this section down here in important in deciding which window to be
//     // `manage`d. We specifically do NOT want to be `manage`-ing the bar
//     // window.
//
//     i = 0;
//     while (i < wins.len) : (i += 1) {
//         const ok = X.XGetWindowAttributes(z.dpy, wins[i], &wa);
//         if (!ok or wa.override_redirect != 0) continue;
//         if (X.XGetTransientForHint(z.dpy, wins[i]) == null) continue;
//         if (wa.map_state == X.IsViewable or getState(z.dpy, wins[i]) == X.IconicState) {
//             log.info("Start managing window {d} (scan, non-transient)", .{wins[i]});
//             try manage(z, allocator, wins[i], &wa);
//         }
//     }
//     i = 0;
//     while (i < wins.len) : (i += 1) { // now the transients
//         if (!X.XGetWindowAttributes(z.dpy, wins[i], &wa)) continue;
//         if (X.XGetTransientForHint(z.dpy, wins[i]) == null) continue;
//         const viewable = wa.map_state == X.IsViewable;
//         const iconic = getState(z.dpy, wins[i]) == X.IconicState;
//         if (viewable or iconic) {
//             log.info("Start managing window {d} (scan, transient)", .{wins[i]});
//             try manage(z, allocator, wins[i], &wa);
//         }
//     }
// }
