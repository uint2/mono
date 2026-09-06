const std = @import("std");

const mem = std.mem;
const meta = std.meta;
const log = std.log;

const Allocator = std.mem.Allocator;
const Clk = @import("enums.zig").Clk;
const Rect = @import("rect.zig").Rect;
const SchemeState = @import("color_scheme.zig").SchemeState;
// const MOUSEMASK = @import("config.zig").MOUSEMASK;
const DwmError = @import("errors.zig").DwmError;
const HandlerFn = @import("enums.zig").HandlerFn;
const atoms = @import("atoms.zig");

const X = @import("x11.zig");
const M = @import("x11.zig").masks;
const EM = @import("x11.zig").eventMask;
const CW = @import("x11.zig").CW;
const E = @import("errors.zig");
const Font = @import("font.zig").Font;
const ColorScheme = @import("color_scheme.zig").ColorScheme;

const NAME = @import("build_opts").name;
const VERSION = @import("build_opts").version;
const LINE = "----------------------------------------------------------------------";

const dpy = @import("display.zig");

var screen: c_int = 0;
/// Screen width;
var sw: c_int = 0;
/// Screen height;
var sh: c_int = 0;
var root: X.Window = 0;

/// C standard library.
const C = @cImport({
    @cInclude("locale.h");
    @cInclude("signal.h");
    @cInclude("unistd.h");
    @cInclude("time.h");
});

pub const std_options: std.Options = .{
    .log_level = .debug,
    .logFn = @import("logger.zig").customLog,
};

/// Returns true if we should terminate the process immediately after this
/// function ends.
fn handleCliArgs(buffer: []u8) error{WriteFailed}!bool {
    var stdout_writer = std.fs.File.stdout().writer(buffer);
    var stdout = &stdout_writer.interface;
    const argv = std.os.argv;
    // If the only flag given is "-v", then print the version.
    if (argv.len == 2 and mem.eql(u8, mem.span(argv[1]), "-v")) {
        try stdout.print("{s}-{s}\n", .{ NAME, VERSION });
        try stdout.flush();
        return true;
    }
    // If there are any CLI args at all, print the super-minimal help text,
    // which is to either run the binary with no flags, or run it with the "-v"
    // flag.
    else if (argv.len != 1) {
        try stdout.print("usage: {s} [-v]\n", .{NAME});
        try stdout.flush();
        return true;
    }
    // Otherwise, we continue execution.
    return false;
}

pub fn main() !void {
    defer log.info("This is the final message from " ++ NAME ++ "!", .{});
    log.info("{s}", .{LINE});
    log.info("Started execution of {s}", .{NAME});
    log.info("{s}", .{LINE});

    { // Handle the CLI args, if any.
        var buffer: [64]u8 = undefined;
        if (try handleCliArgs(&buffer)) return;
    }

    // Initialize the global allocator.
    var gpa: std.heap.GeneralPurposeAllocator(.{ .thread_safe = true }) = .init;
    defer _ = gpa.deinit();
    const alloc = gpa.allocator();

    if (C.setlocale(C.LC_CTYPE, "") == null or !X.XSupportsLocale()) {
        std.debug.print("warning: no locale support\n", .{});
    }

    if (!dpy.init()) {
        return std.debug.print(NAME ++ ": cannot open X display\n", .{});
    }
    defer X.XCloseDisplay(dpy.c);

    {
        const setup = @import("setup.zig");
        setup.checkOtherWM(dpy.c);
        setup.terminationHandling();
        setup.clearZombies();
    }

    screen = X.DefaultScreen(dpy.c);
    sw = X.DisplayWidth(dpy.c, screen);
    sh = X.DisplayHeight(dpy.c, screen);
    root = X.RootWindow(dpy.c, screen);

    _ = alloc;
}

// (dwm) static void applyrules(Client *c);
// (dwm) static int applysizehints(Client *c, int *x, int *y, int *w, int *h, int interact);
// (dwm) static void arrange(Monitor *m);
// (dwm) static void arrangemon(Monitor *m);
// (dwm) static void attach(Client *c);
// (dwm) static void attachstack(Client *c);
// (dwm) static void buttonpress(XEvent *e);
// (dwm) static void cleanup(void);
// (dwm) static void cleanupmon(Monitor *mon);
// (dwm) static void clientmessage(XEvent *e);
// (dwm) static void configure(Client *c);
// (dwm) static void configurenotify(XEvent *e);
// (dwm) static void configurerequest(XEvent *e);
// (dwm) static Monitor *createmon(void);
// (dwm) static void destroynotify(XEvent *e);
// (dwm) static void detach(Client *c);
// (dwm) static void detachstack(Client *c);
// (dwm) static Monitor *dirtomon(int dir);
// (dwm) static void drawbar(Monitor *m);
// (dwm) static void drawbars(void);
// (dwm) static void enternotify(XEvent *e);
// (dwm) static void expose(XEvent *e);
// (dwm) static void focus(Client *c);
// (dwm) static void focusin(XEvent *e);
// (dwm) static void focusmon(const Arg *arg);
// (dwm) static void focusstack(const Arg *arg);
// (dwm) static Atom getatomprop(Client *c, Atom prop);
// (dwm) static int getrootptr(int *x, int *y);
// (dwm) static long getstate(Window w);
// (dwm) static int gettextprop(Window w, Atom atom, char *text, unsigned int size);
// (dwm) static void grabbuttons(Client *c, int focused);
// (dwm) static void grabkeys(void);
// (dwm) static void incnmaster(const Arg *arg);
// (dwm) static void keypress(XEvent *e);
// (dwm) static void killclient(const Arg *arg);
// (dwm) static void manage(Window w, XWindowAttributes *wa);
// (dwm) static void mappingnotify(XEvent *e);
// (dwm) static void maprequest(XEvent *e);
// (dwm) static void monocle(Monitor *m);
// (dwm) static void motionnotify(XEvent *e);
// (dwm) static void movemouse(const Arg *arg);
// (dwm) static Client *nexttiled(Client *c);
// (dwm) static void pop(Client *c);
// (dwm) static void propertynotify(XEvent *e);
// (dwm) static void quit(const Arg *arg);
// (dwm) static Monitor *recttomon(int x, int y, int w, int h);
// (dwm) static void resize(Client *c, int x, int y, int w, int h, int interact);
// (dwm) static void resizeclient(Client *c, int x, int y, int w, int h);
// (dwm) static void resizemouse(const Arg *arg);
// (dwm) static void restack(Monitor *m);
// (dwm) static void run(void);
// (dwm) static void scan(void);
// (dwm) static int sendevent(Client *c, Atom proto);
// (dwm) static void sendmon(Client *c, Monitor *m);
// (dwm) static void setclientstate(Client *c, long state);
// (dwm) static void setfocus(Client *c);
// (dwm) static void setfullscreen(Client *c, int fullscreen);
// (dwm) static void setlayout(const Arg *arg);
// (dwm) static void setmfact(const Arg *arg);
// (dwm) static void setup(void);
// (dwm) static void seturgent(Client *c, int urg);
// (dwm) static void showhide(Client *c);
// (dwm) static void spawn(const Arg *arg);
// (dwm) static void tag(const Arg *arg);
// (dwm) static void tagmon(const Arg *arg);
// (dwm) static void tile(Monitor *m);
// (dwm) static void togglebar(const Arg *arg);
// (dwm) static void togglefloating(const Arg *arg);
// (dwm) static void toggletag(const Arg *arg);
// (dwm) static void toggleview(const Arg *arg);
// (dwm) static void unfocus(Client *c, int setfocus);
// (dwm) static void unmanage(Client *c, int destroyed);
// (dwm) static void unmapnotify(XEvent *e);
// (dwm) static void updatebarpos(Monitor *m);
// (dwm) static void updatebars(void);
// (dwm) static void updateclientlist(void);
// (dwm) static int updategeom(void);
// (dwm) static void updatenumlockmask(void);
// (dwm) static void updatesizehints(Client *c);
// (dwm) static void updatestatus(void);
// (dwm) static void updatetitle(Client *c);
// (dwm) static void updatewindowtype(Client *c);
// (dwm) static void updatewmhints(Client *c);
// (dwm) static void view(const Arg *arg);
// (dwm) static Client *wintoclient(Window w);
// (dwm) static Monitor *wintomon(Window w);
// (dwm) static int xerror(Display *dpy, XErrorEvent *ee);
// (dwm) static int xerrordummy(Display *dpy, XErrorEvent *ee);
// (dwm) static int xerrorstart(Display *dpy, XErrorEvent *ee);
// (dwm) static void zoom(const Arg *arg);
