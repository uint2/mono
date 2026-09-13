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
    name: String,
    mina: c_float,
    mixa: c_float,
    x: c_int,
    y: c_int,
    w: c_int,
    h: c_int,
    oldx: c_int,
    oldy: c_int,
    oldw: c_int,
    oldh: c_int,
    basew: c_int,
    baseh: c_int,
    incw: c_int,
    inch: c_int,
    maxw: c_int,
    maxh: c_int,
    minw: c_int,
    minh: c_int,
    hintsvalid: c_int,
    /* Border width. */
    bw: c_int,
    /* Old border width. */
    oldbw: c_int,
    /* Bitmask of active tags. */
    tags: c_uint,
    isfixed: c_int,
    isfloating: c_int,
    isurgent: c_int,
    neverfocus: c_int,
    /* Old floating state (previous value for `isfloating`). */
    oldstate: c_int,
    isfullscreen: c_int,
    /* Next client in the linked list of clients. */
    next: Ptr<Self>,
    /* Next client in the display stack. */
    snext: Ptr<Self>,
    mon: Ptr<Monitor>,
    win: c::Window,
}

pub struct Key {
    r#mod: c_uint,
    keysym: c::KeySym,
    func: fn(Arg),
    arg: Arg,
}

pub struct Monitor {
    ltsymbol: String,
    mfact: c_float,
    nmaster: c_int,
    num: c_int,
    /* Bar geometry. */
    by: c_int,
    /* Screen size: x-coordinate. */
    mx: c_int,
    /* Screen size: y-coordinate. */
    my: c_int,
    /* Screen size: width. */
    mw: c_int,
    /* Screen size: height. */
    mh: c_int,
    /* Window area: x-coordinate. */
    wx: c_int,
    /* Window area: y-coordinate. */
    wy: c_int,
    /* Window area: width. */
    ww: c_int,
    /* Window area: height. */
    wh: c_int,
    /* Index of selected tags. */
    seltags: c_uint,
    /* Index of selected layout. */
    sellt: c_uint,
    tagset: [c_uint; 2],
    /* 0 means no bar. */
    showbar: c_int,
    /* 0 means bottom bar. */
    topbar: c_int,
    /* Linked list of clients. */
    clients: Ptr<Client>,
    /* Selected client. */
    sel: Ptr<Client>,
    /* Clients ordered by stack. */
    stack: Ptr<Client>,
    next: Ptr<Monitor>,
    barwin: c::Window,
    layout: [Layout; 2],
}

pub struct Layout {
    symbol: &'static str,
    arrange: fn(Ptr<Monitor>),
}

pub struct Rule {
    class: &'static str,
    instance: &'static str,
    title: &'static str,
    tags: c_uint,
    isfloating: c_int,
    ismonitor: c_int,
}
