pub use crate::position_traits::*;
pub use crate::{app::Application, window_state::WindowState};

pub use ::tracing::{error, info};

use std::fmt;
use std::fmt::Debug;

use winit::event::MouseButton;
use winit::keyboard::ModifiersState;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// The amount of points to around the window for drag resize direction calculations.
pub const BORDER_SIZE: f64 = 20.;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum UserEvent {
    WakeUp,
}

pub struct Binding<T: Eq> {
    pub trigger: T,
    pub mods: ModifiersState,
    pub action: Action,
}

impl<T: Eq> Binding<T> {
    pub const fn new(trigger: T, mods: ModifiersState, action: Action) -> Self {
        Self { trigger, mods, action }
    }

    pub fn is_triggered_by(&self, trigger: &T, mods: &ModifiersState) -> bool {
        &self.trigger == trigger && &self.mods == mods
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    CloseWindow,
    CreateNewWindow,
    ToggleImeInput,
    ToggleDecorations,
    ToggleFullscreen,
    ToggleMaximize,
    Minimize,
    CycleCursorGrab,
    DragWindow,
    DragResizeWindow,
    ShowWindowMenu,
    #[cfg(target_os = "macos")]
    CycleOptionAsAlt,
    #[cfg(target_os = "macos")]
    CreateNewTab,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self, f)
    }
}

pub const KEY_BINDINGS: &[Binding<&'static str>] = &[
    Binding::new("Q", ModifiersState::CONTROL, Action::CloseWindow),
    Binding::new("F", ModifiersState::CONTROL, Action::ToggleFullscreen),
    Binding::new("D", ModifiersState::CONTROL, Action::ToggleDecorations),
    Binding::new("I", ModifiersState::CONTROL, Action::ToggleImeInput),
    Binding::new("L", ModifiersState::CONTROL, Action::CycleCursorGrab),
    // M.
    Binding::new("M", ModifiersState::CONTROL, Action::ToggleMaximize),
    Binding::new("M", ModifiersState::ALT, Action::Minimize),
    // N.
    Binding::new("N", ModifiersState::CONTROL, Action::CreateNewWindow),
    // C.
    #[cfg(target_os = "macos")]
    Binding::new("T", ModifiersState::SUPER, Action::CreateNewTab),
    #[cfg(target_os = "macos")]
    Binding::new("O", ModifiersState::CONTROL, Action::CycleOptionAsAlt),
];

pub const MOUSE_BINDINGS: &[Binding<MouseButton>] = &[
    Binding::new(
        MouseButton::Left,
        ModifiersState::ALT,
        Action::DragResizeWindow,
    ),
    Binding::new(
        MouseButton::Left,
        ModifiersState::CONTROL,
        Action::DragWindow,
    ),
    Binding::new(
        MouseButton::Right,
        ModifiersState::CONTROL,
        Action::ShowWindowMenu,
    ),
];
