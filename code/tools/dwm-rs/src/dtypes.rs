use crate::prelude::*;

pub enum Cur {
    CurNormal,
    CurResize,
    CurMove,
    CurLast,
} /* cursor */
pub enum Scheme {
    SchemeNorm,
    SchemeSel,
    SchemeBar,
} /* color schemes */
pub enum Net {
    NetSupported,
    NetWMName,
    NetWMState,
    NetWMCheck,
    NetWMFullscreen,
    NetActiveWindow,
    NetWMWindowType,
    NetWMWindowTypeDialog,
    NetClientList,
    NetLast,
} /* EWMH atoms */
pub enum WM {
    WMProtocols,
    WMDelete,
    WMState,
    WMTakeFocus,
    WMLast,
} /* default atoms */
pub enum Clk {
    ClkTagBar,
    ClkLtSymbol,
    ClkStatusText,
    ClkWinTitle,
    ClkClientWin,
    ClkRootWin,
    ClkLast,
} /* clicks */

pub enum Arg {
    Int(c_int),
    Uint(c_uint),
    Float(c_float),
    // Void(c_void), // TODO
}

pub struct Client {
    pub name: String,
    pub mina: c_float,
    pub mixa: c_float,
    pub x: c_int,
    pub y: c_int,
    pub w: c_int,
    pub h: c_int,
    pub oldx: c_int,
    pub oldy: c_int,
    pub oldw: c_int,
    pub oldh: c_int,
    pub basew: c_int,
    pub baseh: c_int,
    pub incw: c_int,
    pub inch: c_int,
    pub maxw: c_int,
    pub maxh: c_int,
    pub minw: c_int,
    pub minh: c_int,
    pub hintsvalid: c_int,
    /* Border width. */
    pub bw: c_int,
    /* Old border width. */
    pub oldbw: c_int,
    /* Bitmask of active tags. */
    pub tags: c_uint,
    pub isfixed: c_int,
    pub isfloating: bool,
    pub isurgent: c_int,
    pub neverfocus: c_int,
    /* Old floating state (previous value for `isfloating`). */
    pub oldstate: c_int,
    pub isfullscreen: c_int,
    /* Next client in the linked list of clients. */
    pub next: Ptr<Self>,
    /* Next client in the display stack. */
    pub snext: Ptr<Self>,
    pub mon: Ptr<Monitor>,
    pub win: c::Window,
}

impl Client {
    #[cfg(test)]
    pub fn mock(monitor: Ptr<Monitor>, window: c::Window) -> Self {
        Self {
            name: String::from("mock"),
            mina: 0.,
            mixa: 0.,
            x: 0,
            y: 0,
            w: 0,
            h: 0,
            oldx: 0,
            oldy: 0,
            oldw: 0,
            oldh: 0,
            basew: 0,
            baseh: 0,
            incw: 0,
            inch: 0,
            maxw: 0,
            maxh: 0,
            minw: 0,
            minh: 0,
            hintsvalid: 0,
            bw: 0,
            oldbw: 0,
            tags: 0,
            isfixed: 0,
            isfloating: false,
            isurgent: 0,
            neverfocus: 0,
            oldstate: 0,
            isfullscreen: 0,
            next: Ptr::NULL,
            snext: Ptr::NULL,
            mon: monitor,
            win: window,
        }
    }
}

pub struct Key {
    pub r#mod: c_uint,
    pub keysym: c::KeySym,
    pub func: fn(Arg),
    pub arg: Arg,
}

pub struct Monitor {
    pub ltsymbol: &'static str,
    pub mfact: c_float,
    pub nmaster: c_int,
    pub num: c_int,
    /* Bar geometry. */
    pub by: c_int,
    /* Screen size: x-coordinate. */
    pub mx: c_int,
    /* Screen size: y-coordinate. */
    pub my: c_int,
    /* Screen size: width. */
    pub mw: c_int,
    /* Screen size: height. */
    pub mh: c_int,
    /* Window area: x-coordinate. */
    pub wx: c_int,
    /* Window area: y-coordinate. */
    pub wy: c_int,
    /* Window area: width. */
    pub ww: c_int,
    /* Window area: height. */
    pub wh: c_int,
    /* Index of selected tags. */
    pub seltags: c_uint,
    /* Index of selected layout. */
    pub sellt: c_uint,
    pub tagset: [c_uint; 2],
    /* 0 means no bar. */
    pub showbar: c_int,
    /* 0 means bottom bar. */
    pub topbar: c_int,
    /* Linked list of clients. */
    pub clients: Ptr<Client>,
    /* Selected client. */
    pub sel: Ptr<Client>,
    /* Clients ordered by stack. */
    pub stack: Ptr<Client>,
    pub next: Ptr<Monitor>,
    pub barwin: c::Window,
    pub layout: [Layout; 2],
}

impl Monitor {
    #[cfg(test)]
    pub fn mock() -> Self {
        Self {
            ltsymbol: "mock",
            mfact: 0.,
            nmaster: 0,
            num: 0,
            by: 0,
            mx: 0,
            my: 0,
            mw: 0,
            mh: 0,
            wx: 0,
            wy: 0,
            ww: 0,
            wh: 0,
            seltags: 0,
            sellt: 0,
            tagset: [0, 0],
            showbar: 0,
            topbar: 0,
            clients: Ptr::NULL,
            sel: Ptr::NULL,
            stack: Ptr::NULL,
            next: Ptr::NULL,
            barwin: 0,
            layout: [Layout::mock(), Layout::mock()],
        }
    }
}

pub struct Layout {
    pub symbol: &'static str,
    pub arrange: fn(Ptr<Monitor>),
}

#[cfg(test)]
fn arrange_mock(_: Ptr<Monitor>) {}

impl Layout {
    #[cfg(test)]
    pub fn mock() -> Self {
        Self { symbol: "nothing", arrange: arrange_mock }
    }
}

pub struct Rule {
    pub class: Option<&'static str>,
    pub instance: Option<&'static str>,
    pub title: Option<&'static str>,
    pub tags: c_uint,
    pub isfloating: bool,
    pub monitor: c_int,
}
