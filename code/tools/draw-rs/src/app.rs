use crate::prelude::*;

use std::collections::HashMap;

use rwh_06::{DisplayHandle, HasDisplayHandle};
use softbuffer::Context;

use tiny_skia_path::Point;
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{
    DeviceEvent, DeviceId, ElementState, Ime, MouseButton, MouseScrollDelta,
    WindowEvent,
};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{Key, ModifiersState};
use winit::window::{Window, WindowId};

#[cfg(target_os = "macos")]
use winit::platform::macos::{WindowAttributesExtMacOS, WindowExtMacOS};

/// Application state and event handling.
pub struct Application {
    windows: HashMap<WindowId, WindowState>,
    /// Drawing context.
    ///
    /// With OpenGL it could be EGLDisplay.
    pub context: Option<Context<DisplayHandle<'static>>>,
}

impl Application {
    pub fn new<T>(event_loop: &EventLoop<T>) -> Self {
        // SAFETY: we drop the context right before the event loop is stopped, thus making it safe.
        let context =
            Some(
                Context::new(unsafe {
                    std::mem::transmute::<
                        DisplayHandle<'_>,
                        DisplayHandle<'static>,
                    >(event_loop.display_handle().unwrap())
                })
                .unwrap(),
            );

        Self { context, windows: Default::default() }
    }

    fn create_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        _tab_id: Option<String>,
    ) -> Result<WindowId> {
        // TODO read-out activation token.

        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes()
            .with_title("Winit window")
            .with_transparent(true);

        #[cfg(target_os = "macos")]
        if let Some(tab_id) = _tab_id {
            window_attributes =
                window_attributes.with_tabbing_identifier(&tab_id);
        }

        let window = event_loop.create_window(window_attributes)?;

        let window_state = WindowState::new(self, window)?;
        let window_id = window_state.window.id();
        info!("Created new window with id={window_id:?}");
        self.windows.insert(window_id, window_state);
        Ok(window_id)
    }

    fn handle_action(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        action: Action,
    ) {
        // let cursor_position = self.cursor_position;
        let window = self.windows.get_mut(&window_id).unwrap();
        info!("Executing action: {action:?}");
        match action {
            Action::CloseWindow => {
                let _ = self.windows.remove(&window_id);
            }
            Action::CreateNewWindow => {
                if let Err(err) = self.create_window(event_loop, None) {
                    error!("Error creating new window: {err}");
                }
            }
            Action::ToggleDecorations => window.toggle_decorations(),
            Action::ToggleFullscreen => window.toggle_fullscreen(),
            Action::ToggleMaximize => window.toggle_maximize(),
            Action::ToggleImeInput => window.toggle_ime(),
            Action::Minimize => window.minimize(),
            Action::CycleCursorGrab => window.cycle_cursor_grab(),
            Action::DragWindow => window.drag_window(),
            Action::DragResizeWindow => window.drag_resize_window(),
            Action::ShowWindowMenu => window.show_menu(),
            #[cfg(target_os = "macos")]
            Action::CycleOptionAsAlt => window.cycle_option_as_alt(),
            #[cfg(target_os = "macos")]
            Action::CreateNewTab => {
                let tab_id = window.window.tabbing_identifier();
                if let Err(err) = self.create_window(event_loop, Some(tab_id)) {
                    error!("Error creating new window: {err}");
                }
            }
        }
    }

    fn dump_monitors(&self, event_loop: &ActiveEventLoop) {
        info!("Monitors information");
        let primary_monitor = event_loop.primary_monitor();
        for monitor in event_loop.available_monitors() {
            let intro = if primary_monitor.as_ref() == Some(&monitor) {
                "Primary monitor"
            } else {
                "Monitor"
            };

            if let Some(name) = monitor.name() {
                info!("{intro}: {name}");
            } else {
                info!("{intro}: [no name]");
            }

            let PhysicalSize { width, height } = monitor.size();
            info!(
                "  Current mode: {width}x{height}{}",
                if let Some(m_hz) = monitor.refresh_rate_millihertz() {
                    format!(" @ {}.{} Hz", m_hz / 1000, m_hz % 1000)
                } else {
                    String::new()
                }
            );

            let PhysicalPosition { x, y } = monitor.position();
            info!("  Position: {x},{y}");
            info!("  Scale factor: {}", monitor.scale_factor());
        }
    }

    /// Process the key binding.
    fn process_key_binding(key: &str, mods: &ModifiersState) -> Option<Action> {
        KEY_BINDINGS.iter().find_map(|binding| {
            binding.is_triggered_by(&key, mods).then_some(binding.action)
        })
    }

    /// Process mouse binding.
    fn process_mouse_binding(
        button: MouseButton,
        mods: &ModifiersState,
    ) -> Option<Action> {
        MOUSE_BINDINGS.iter().find_map(|binding| {
            binding.is_triggered_by(&button, mods).then_some(binding.action)
        })
    }
}

impl ApplicationHandler<UserEvent> for Application {
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, _event: UserEvent) {
        // info!("User event: {event:?}");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let window = match self.windows.get_mut(&window_id) {
            Some(window) => window,
            None => return,
        };

        match event {
            WindowEvent::Resized(size) => {
                window.resize(size);
            }
            WindowEvent::Focused(focused) => {
                if focused {
                    info!("Window={window_id:?} focused");
                } else {
                    info!("Window={window_id:?} unfocused");
                }
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                info!("Window={window_id:?} changed scale to {scale_factor}");
            }
            WindowEvent::ThemeChanged(theme) => {
                info!("Theme changed to {theme:?}");
                window.set_theme(theme);
            }
            WindowEvent::RedrawRequested => {
                if let Err(err) = window.draw() {
                    error!("Error drawing window: {err}");
                }
            }
            WindowEvent::Occluded(occluded) => {
                window.set_occluded(occluded);
            }
            WindowEvent::CloseRequested => {
                info!("Closing Window={window_id:?}");
                self.windows.remove(&window_id);
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                window.modifiers = modifiers.state();
                info!("Modifiers changed to {:?}", window.modifiers);
            }
            WindowEvent::MouseWheel { delta, .. } => match delta {
                MouseScrollDelta::LineDelta(x, y) => {
                    info!("Mouse wheel Line Delta: ({x},{y})");
                }
                MouseScrollDelta::PixelDelta(px) => {
                    info!("Mouse wheel Pixel Delta: ({},{})", px.x, px.y);
                }
            },
            WindowEvent::KeyboardInput {
                event, is_synthetic: false, ..
            } => {
                let mods = window.modifiers;

                // Dispatch actions only on press.
                if event.state.is_pressed() {
                    let action = if let Key::Character(ch) =
                        event.logical_key.as_ref()
                    {
                        Self::process_key_binding(&ch.to_uppercase(), &mods)
                    } else {
                        None
                    };

                    if let Some(action) = action {
                        self.handle_action(event_loop, window_id, action);
                    }
                }
            }
            WindowEvent::MouseInput { button, state, .. } => {
                match (button, state) {
                    (MouseButton::Left, ElementState::Pressed) => {
                        window.is_drawing = true;
                        let mut line = vec![];
                        if let Some(pos) = window.cursor_position {
                            line.push(Point::from_xy(
                                pos.x as f32,
                                pos.y as f32,
                            ));
                        }
                        window.lines.push(line);
                        window.window.request_redraw();
                    }
                    (MouseButton::Left, ElementState::Released) => {
                        window.is_drawing = false;
                        let last_line = window.lines.last_mut().unwrap();
                        if let Some(pos) = window.cursor_position {
                            last_line.push(Point::from_xy(
                                pos.x as f32,
                                pos.y as f32,
                            ));
                        }
                    }
                    _ => {}
                }
            }
            WindowEvent::CursorLeft { .. } => {
                info!("Cursor left Window={window_id:?}");
                window.cursor_left();
            }
            WindowEvent::CursorMoved { position, .. } => {
                // info!("Moved cursor to {position:?}");
                window.cursor_moved(position);
                if window.is_drawing {
                    let pos =
                        Point::from_xy(position.x as f32, position.y as f32);
                    let line = window.lines.last_mut().unwrap();
                    match line.last() {
                        Some(v) if v.distance(pos) > 1. => line.push(pos),
                        None => line.push(pos),
                        _ => (),
                    }
                    window.window.request_redraw();
                }
            }
            WindowEvent::Ime(event) => match event {
                Ime::Enabled => info!("IME enabled for Window={window_id:?}"),
                Ime::Preedit(text, caret_pos) => {
                    info!("Preedit: {}, with caret at {:?}", text, caret_pos);
                }
                Ime::Commit(text) => {
                    info!("Committed: {}", text);
                }
                Ime::Disabled => info!("IME disabled for Window={window_id:?}"),
            },
            _ => (),
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        _event: DeviceEvent,
    ) {
        // info!("Device {device_id:?} event: {event:?}");
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info!("Resumed the event loop");
        self.dump_monitors(event_loop);

        // Create initial window.
        self.create_window(event_loop, None)
            .expect("failed to create initial window");
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.windows.is_empty() {
            info!("No windows left, exiting...");
            event_loop.exit();
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        // We must drop the context here.
        self.context = None;
    }
}
