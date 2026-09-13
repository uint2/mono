#[macro_use]
mod macros;

mod c;
mod config;
mod dtypes;
mod globals;
mod pointer;
mod prelude;

use prelude::*;

/// (dwm) static void applyrules(Client *c);
fn applyrules(c: Ptr<Client>) {
    let mut ch: c::XClassHint = c::undefined();

    /* rule matching */
    c.write().unwrap().isfloating = 0;
    c.write().unwrap().tags = 0;
    x!(XGetClassHint(dpy, c.read().unwrap().win, &mut ch));
    let class = c::to_str(ch.res_class).unwrap_or("broken");
    let instance = c::to_str(ch.res_name).unwrap_or("broken");

    for i in 0..rules.len() {
        let r = &rules[i];
        if r.title.map_or(true, |v| c.read().unwrap().name.contains(v))
            && r.class.map_or(true, |v| class.contains(v))
            && r.instance.map_or(true, |v| instance.contains(v))
        {
            c.write().unwrap().isfloating = r.isfloating;
            c.write().unwrap().tags |= r.tags;
            let mut m = Ptr::clone(&mons);
            while !m.is_null() && m.read().unwrap().num != r.monitor {
                let next = m.read().unwrap().next.clone();
                m = next;
            }
            if !m.is_null() {
                c.write().unwrap().mon = m;
            }
        }
    }
    if !ch.res_class.is_null() {
        c::xfree(ch.res_class);
    }
    if !ch.res_name.is_null() {
        c::xfree(ch.res_name);
    }
    c.write().unwrap().tags = if c.r().tags & TAGMASK != 0 {
        c.r().tags & TAGMASK
    } else {
        let c = c.r();
        let m = c.mon.r();
        m.tagset[m.seltags as usize]
    };
}

/*
/// (dwm) static int applysizehints(Client *c, int *x, int *y, int *w, int *h, int interact);
/// (dwm) static void arrange(Monitor *m);
/// (dwm) static void arrangemon(Monitor *m);
/// (dwm) static void attach(Client *c);
/// (dwm) static void attachstack(Client *c);
/// (dwm) static void buttonpress(XEvent *e);
/// (dwm) static void checkotherwm(void);
/// (dwm) static void cleanup(void);
/// (dwm) static void cleanupmon(Monitor *mon);
/// (dwm) static void clientmessage(XEvent *e);
/// (dwm) static void configure(Client *c);
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
/// (dwm) static void manage(Window w, XWindowAttributes *wa);
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
/// (dwm) static void resize(Client *c, int x, int y, int w, int h, int interact);
/// (dwm) static void resizeclient(Client *c, int x, int y, int w, int h);
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
/// (dwm) static void showhide(Client *c);
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
/// (dwm) static void updatesizehints(Client *c);
/// (dwm) static void updatestatus(void);
/// (dwm) static void updatetitle(Client *c);
/// (dwm) static void updatewindowtype(Client *c);
/// (dwm) static void updatewmhints(Client *c);
/// (dwm) static void view(const Arg *arg);
/// (dwm) static Client *wintoclient(Window w);
/// (dwm) static Monitor *wintomon(Window w);
/// (dwm) static int xerror(Display *dpy, XErrorEvent *ee);
/// (dwm) static int xerrordummy(Display *dpy, XErrorEvent *ee);
/// (dwm) static int xerrorstart(Display *dpy, XErrorEvent *ee);
/// (dwm) static void zoom(const Arg *arg);
*/

fn main() {
    let display = x!(XOpenDisplay(ptr::null()));
    unsafe { dpy = display };
}
