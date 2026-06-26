const std = @import("std");

const X = @import("x11.zig");

const NAME = @import("build_opts").name;

pub const DwmError = error{
    OutOfMemory,
    SomeOtherError,
    FontCreateError,
};

////////////////////////////////////////////////////////////////////////////////
// X-specific error handling.
// The old-school way.
////////////////////////////////////////////////////////////////////////////////

/// The do-nothing error handler.
pub fn xerrordummy(_: ?*X.Display, _: [*c]X.XErrorEvent) callconv(.c) c_int {
    return 0;
}

/// This is only used at initialization to check if there is another window
/// manager already running.
///
/// (dwm) xerrorstart
pub fn xerrorstart(_dpy: ?*X.Display, _event: [*c]X.XErrorEvent) callconv(.c) c_int {
    _ = _dpy;
    _ = _event;
    std.debug.print(NAME ++ ": another window manager is already running\n", .{});
    std.process.exit(1);
}

/// (dwm) xerror
pub fn xerror(dpy: ?*X.Display, event: [*c]X.XErrorEvent) callconv(.c) c_int {
    const err_event: *X.XErrorEvent = event orelse {
        std.debug.print(NAME ++ ": called xerror with null X.XErrorEvent value\n", .{});
        if (xerrorlib) |f| return f(dpy, event);
        @panic("xerror called but xerrorlib not defined yet.");
    };
    const rc = err_event.request_code;
    const ec = err_event.error_code;
    if (ec == X.err.BadWindow or
        (rc == X.rq.SetInputFocus and ec == X.err.BadMatch) or
        (rc == X.rq.PolyText8 and ec == X.err.BadDrawable) or
        (rc == X.rq.PolyFillRectangle and ec == X.err.BadDrawable) or
        (rc == X.rq.PolySegment and ec == X.err.BadDrawable) or
        (rc == X.rq.ConfigureWindow and ec == X.err.BadMatch) or
        (rc == X.rq.GrabButton and ec == X.err.BadAccess) or
        (rc == X.rq.GrabKey and ec == X.err.BadAccess) or
        (rc == X.rq.CopyArea and ec == X.err.BadDrawable))
    {
        return 0;
    }
    std.debug.print(NAME ++ ": fatal error: request code={d}, error code={d}\n", .{ rc, ec });
    if (xerrorlib) |f| {
        return f(dpy, err_event);
    }
    @panic("xerror called but xerrorlib not defined yet.");
}

pub var xerrorlib: ?*const fn (?*X.Display, [*c]X.XErrorEvent) callconv(.c) c_int = null;
