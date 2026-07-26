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
pub enum SchemeState {
    Normal,
    Selected,
    Bar,
}
enum_array!(SchemeStateArray, SchemeState);

#[derive(Debug)]
pub struct Scheme<T> {
    /// Foreground color.
    pub fg: T,
    /// Background color.
    pub bg: T,
    /// Border color.
    pub border: T,
}
