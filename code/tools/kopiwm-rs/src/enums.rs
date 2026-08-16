use crate::C;
use crate::prelude::*;

use strum_macros::EnumCount;

/// (dwm) Cur* enums.
/// The different possible states of the mouse cursor.
#[derive(Clone, Copy, EnumCount)]
pub enum CursorState {
    Normal,
    Resize,
    Move,
}
enum_array!(CursorStateArray, CursorState);
all_array!(CursorState, Normal, Resize, Move);

/// (dwm) Clk* enums.
#[derive(Clone, Copy, EnumCount)]
pub enum Clk {
    /// User clicked on one of the tags in the tags list (traditionally located
    /// at the top-left) in the bar window.
    TagBar,
    /// User clicked the layout symbol (traditionally located to the left of the
    /// tags) in the bar window.
    LtSymbol,
    /// User clicked the status text (traditionally located at top-right) in the
    /// bar window.
    StatusText,
    /// User clicked the window title in the bar window.
    WinTitle,
    /// User clicked on a client window.
    ClientWin,
    /// The base case: User clicked on none of the above.
    RootWin,
}

/// Represents a possible which one might be in that warrants a unique color scheme.
#[derive(Clone, Copy, EnumCount)]
pub enum WindowColorState {
    Normal,
    Selected,
    Bar,
}
enum_array!(WindowColorStateArray, WindowColorState);
all_array!(WindowColorState, Normal, Selected, Bar);

#[derive(Debug, Clone, Copy)]
pub struct WindowColors<T> {
    /// Foreground color.
    pub fg: T,
    /// Background color.
    pub bg: T,
    /// Border color.
    pub border: T,
}

/// (dwm) WM* atoms.
#[derive(Clone, Copy, EnumCount)]
pub enum WM {
    Delete,
    Protocols,
    State,
    TakeFocus,
}
enum_array!(WMArray, WM);
all_array!(WM, Delete, Protocols, State, TakeFocus);

impl WM {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Delete => "WM_DELETE_WINDOW",
            Self::Protocols => "WM_PROTOCOLS",
            Self::State => "WM_STATE",
            Self::TakeFocus => "WM_TAKE_FOCUS",
        }
    }

    pub fn init() -> WMArray<C::Atom> {
        let mut arr = WMArray::new();
        for variant in WM::ALL {
            arr.set(variant, x11::XInternAtom(variant.as_str(), false).unwrap());
        }
        arr
    }
}

/// (dwm) Net* atoms.
///
/// See  https://specifications.freedesktop.org/wm/1.5/  For more details.
#[derive(Clone, Copy, EnumCount)]
pub enum Net {
    ActiveWindow,
    ClientList,
    /// This property MUST be set by the Window Manager to indicate which
    /// hints it supports. For example: considering _NET_WM_STATE both this
    /// atom and all supported states e.g. _NET_WM_STATE_MODAL,
    /// _NET_WM_STATE_STICKY, would be listed. This assumes that backwards
    /// incompatible changes will not be made to the hints (without being
    /// renamed).
    Supported,
    /// The Window Manager MUST set this property on the root window to be the
    /// ID of a child window created by himself, to indicate that a compliant
    /// window manager is active. The child window MUST also have the
    /// _NET_SUPPORTING_WM_CHECK property set to the ID of the child window.
    /// The child window MUST also have the _NET_WM_NAME property set to the
    /// name of the Window Manager.
    ///
    /// Rationale: The child window is used to distinguish an active Window
    /// Manager from a stale _NET_SUPPORTING_WM_CHECK property that happens to
    /// point to another window. If the _NET_SUPPORTING_WM_CHECK window on the
    /// client window is missing or not properly set, clients SHOULD assume
    /// that no conforming Window Manager is present.
    ///
    /// source: https://specifications.freedesktop.org/wm/1.5/
    ///
    /// Look for "_NET_SUPPORTING_WM_CHECK" in the subpages (for the subpage,
    /// look for "Root Window Properties (and Related Messages)").
    WMCheck,
    WMFullscreen,
    WMName,
    WMState,
    WMWindowType,
    WMWindowTypeDialog,
}
enum_array!(NetArray, Net);
all_array!(
    Net,
    ActiveWindow,
    ClientList,
    Supported,
    WMCheck,
    WMFullscreen,
    WMName,
    WMState,
    WMWindowType,
    WMWindowTypeDialog
);

impl Net {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::ActiveWindow => "_NET_ACTIVE_WINDOW",
            Self::ClientList => "_NET_CLIENT_LIST",
            Self::Supported => "_NET_SUPPORTED",
            Self::WMCheck => "_NET_SUPPORTING_WM_CHECK",
            Self::WMFullscreen => "_NET_WM_STATE_FULLSCREEN",
            Self::WMName => "_NET_WM_NAME",
            Self::WMState => "_NET_WM_STATE",
            Self::WMWindowType => "_NET_WM_WINDOW_TYPE",
            Self::WMWindowTypeDialog => "_NET_WM_WINDOW_TYPE_DIALOG",
        }
    }

    pub fn init() -> NetArray<C::Atom> {
        let mut arr = NetArray::new();
        for variant in Net::ALL {
            arr.set(variant, x11::XInternAtom(variant.as_str(), false).unwrap());
        }
        arr
    }
}

#[derive(Clone, Copy)]
pub enum BarPosition {
    Top,
    Bottom,
}
