mod C;
mod config;
mod nonempty;
mod prelude;
mod rect;
mod tag;
mod window;
mod x11;

use prelude::*;

#[allow(unused)]
struct Layout {
    arrange: Option<fn() -> ()>,
}

struct OwnedWindow {}

#[allow(unused)]
struct Client {
    window: OwnedWindow,
    tags: TagMask,
}

#[allow(unused)]
struct Monitor {
    stack: Vec<Client>,
    sel: Option<usize>,
    lt: [Layout; 2],
    // Only can be 0 or 1.
    sellt: usize,
    tagset: [TagMask; 2],
    // Only can be 0 or 1.
    seltags: usize,
}

#[allow(unused)]
struct App {
    monitors: Vec<Monitor>,
}

#[allow(unused)]
impl Client {
    pub fn new(window: OwnedWindow) -> Self {
        Self { window, tags: TagMask::EMPTY }
    }

    pub fn window(&self) -> &OwnedWindow {
        &self.window
    }

    /// (dwm) static void resize(Client *c, int x, int y, int w, int h, int interact);
    pub fn resize(&mut self) {}

    /// (dwm) static void configure(Client *c);
    pub fn configure(&self) {}

    /// (dwm) static void resizeclient(Client *c, int x, int y, int w, int h);
    pub fn resize_client(&mut self) {}

    /// (dwm) static void updatesizehints(Client *c);
    pub fn update_size_hints(&mut self) {}
}

#[allow(unused)]
impl Monitor {
    /// (dwm) static void showhide(Client *c);
    pub fn showhide(&self, client: &Client) {
        if self.is_client_visible(client) {

            /* show clients top down */
        } else {
            /* hide clients bottom up */
        }
    }

    /// (dwm) static int applysizehints(Client *c, int *x, int *y, int *w, int *h, int interact);
    pub fn apply_size_hints<'a>(&self, client: &mut Client) {
        // Potential call list
        client.update_size_hints();
    }

    /// (dwm) #define ISVISIBLE(C) ((C->tags & C->mon->tagset[C->mon->seltags]))
    pub fn is_client_visible(&self, client: &Client) -> bool {
        (client.tags & self.tagset[self.seltags]).non_zero()
    }
}

#[allow(unused)]
impl App {
    /// (dwm) static void applyrules(Client *c);
    fn apply_rules(&self, client: &mut Client) {}

    /// (dwm) static Client *wintoclient(Window w);
    fn win_to_client(&self) {}

    /// (dwm) static void manage(Window w, XWindowAttributes *wa);
    fn manage(&mut self, window: OwnedWindow) {
        let mut c = Client::new(window);
        // let mon = self.monitors.first_mut().unwrap();
        // let client = mon.stack.first_mut().unwrap();
        // mon.stack.push(unsafe { core::mem::zeroed() });
        // client.resize();
        // mon.apply_size_hints(client);
    }
}

/// (dwm) static void arrange(Monitor *m);
/// (dwm) static void arrangemon(Monitor *m);
/// (dwm) static void attach(Client *c);
/// (dwm) static void attachstack(Client *c);
/// (dwm) static void buttonpress(XEvent *e);
/// (dwm) static void checkotherwm(void);
/// (dwm) static void cleanup(void);
/// (dwm) static void cleanupmon(Monitor *mon);
/// (dwm) static void clientmessage(XEvent *e);
/// (dwm) static void configurenotify(XEvent *e);
/// (dwm) static void configurerequest(XEvent *e);
/// (dwm) static Monitor *createmon(void);
/// (dwm) static void destroynotify(XEvent *e);
/// (dwm) static void detach(Client *c);
/// (dwm) static void detachstack(Client *c);
/// (dwm) static Monitor *dirtomon(int dir);
/// (dwm) static void drawbar(Monitor *m);
/// (dwm) static void drawbars(void);
/// (dwm) static void enternotify(XEvent *e);
/// (dwm) static void expose(XEvent *e);
/// (dwm) static void focus(Client *c);
/// (dwm) static void focusin(XEvent *e);
/// (dwm) static void focusmon(const Arg *arg);
/// (dwm) static void focusstack(const Arg *arg);
/// (dwm) static Atom getatomprop(Client *c, Atom prop);
/// (dwm) static int getrootptr(int *x, int *y);
/// (dwm) static long getstate(Window w);
/// (dwm) static int gettextprop(Window w, Atom atom, char *text, unsigned int size);
/// (dwm) static void grabbuttons(Client *c, int focused);
/// (dwm) static void grabkeys(void);
/// (dwm) static void incnmaster(const Arg *arg);
/// (dwm) static void keypress(XEvent *e);
/// (dwm) static void killclient(const Arg *arg);
/// (dwm) static void mappingnotify(XEvent *e);
/// (dwm) static void maprequest(XEvent *e);
/// (dwm) static void monocle(Monitor *m);
/// (dwm) static void motionnotify(XEvent *e);
/// (dwm) static void movemouse(const Arg *arg);
/// (dwm) static Client *nexttiled(Client *c);
/// (dwm) static void pop(Client *c);
/// (dwm) static void propertynotify(XEvent *e);
/// (dwm) static void quit(const Arg *arg);
/// (dwm) static Monitor *recttomon(int x, int y, int w, int h);
/// (dwm) static void resizemouse(const Arg *arg);
/// (dwm) static void restack(Monitor *m);
/// (dwm) static void run(void);
/// (dwm) static void scan(void);
/// (dwm) static int sendevent(Client *c, Atom proto);
/// (dwm) static void sendmon(Client *c, Monitor *m);
/// (dwm) static void setclientstate(Client *c, long state);
/// (dwm) static void setfocus(Client *c);
/// (dwm) static void setfullscreen(Client *c, int fullscreen);
/// (dwm) static void setlayout(const Arg *arg);
/// (dwm) static void setmfact(const Arg *arg);
/// (dwm) static void setup(void);
/// (dwm) static void seturgent(Client *c, int urg);
/// (dwm) static void spawn(const Arg *arg);
/// (dwm) static void tag(const Arg *arg);
/// (dwm) static void tagmon(const Arg *arg);
/// (dwm) static void tile(Monitor *m);
/// (dwm) static void togglebar(const Arg *arg);
/// (dwm) static void togglefloating(const Arg *arg);
/// (dwm) static void toggletag(const Arg *arg);
/// (dwm) static void toggleview(const Arg *arg);
/// (dwm) static void unfocus(Client *c, int setfocus);
/// (dwm) static void unmanage(Client *c, int destroyed);
/// (dwm) static void unmapnotify(XEvent *e);
/// (dwm) static void updatebarpos(Monitor *m);
/// (dwm) static void updatebars(void);
/// (dwm) static void updateclientlist(void);
/// (dwm) static int updategeom(void);
/// (dwm) static void updatenumlockmask(void);
/// (dwm) static void updatestatus(void);
/// (dwm) static void updatetitle(Client *c);
/// (dwm) static void updatewindowtype(Client *c);
/// (dwm) static void updatewmhints(Client *c);
/// (dwm) static void view(const Arg *arg);
/// (dwm) static Monitor *wintomon(Window w);
/// (dwm) static int xerror(Display *dpy, XErrorEvent *ee);
/// (dwm) static int xerrordummy(Display *dpy, XErrorEvent *ee);
/// (dwm) static int xerrorstart(Display *dpy, XErrorEvent *ee);
/// (dwm) static void zoom(const Arg *arg);

fn main() {
    println!("Hello, world!");
}
