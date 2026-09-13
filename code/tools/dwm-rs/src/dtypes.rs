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
    next: Option<Ptr<Self>>,
    /* Next client in the display stack. */
    snext: Option<Ptr<Self>>,
    mon: Ptr<Monitor>,
    win: c::Window,
}

pub struct Monitor {}
