use crate::prelude::*;

use std::sync::Arc;

use tiny_skia_path::{PathSegment, Point, Stroke};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::keyboard::ModifiersState;
#[cfg(target_os = "macos")]
use winit::platform::macos::{OptionAsAlt, WindowExtMacOS};
use winit::window::{
    CursorGrabMode, Fullscreen, ResizeDirection, Theme, Window,
};

use pixels::{Pixels, SurfaceTexture};

use cursor_icon::CursorIcon;
use rwh_06::DisplayHandle;
use softbuffer::Buffer;

const DARK_GRAY: u32 = 0xff181818;

/// State of the window.
pub struct WindowState {
    /// IME input.
    pub ime: bool,
    /// Render surface.
    ///
    /// NOTE: This surface must be dropped before the `Window`.
    pub pixels: Pixels,
    /// The actual winit Window.
    pub window: Arc<Window>,
    /// The window theme we're drawing with.
    pub theme: Theme,
    /// Cursor position over the window.
    pub cursor_position: Option<PhysicalPosition<f64>>,
    /// Window modifiers state.
    pub modifiers: ModifiersState,
    /// Occlusion state of the window.
    pub occluded: bool,
    /// Current cursor grab mode.
    pub cursor_grab: CursorGrabMode,

    #[cfg(target_os = "macos")]
    pub option_as_alt: OptionAsAlt,

    pub is_drawing: bool,
    pub lines: Vec<Vec<Point>>,
}

impl WindowState {
    pub fn new(app: &Application, window: Window) -> Result<Self> {
        let window = Arc::new(window);

        let pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(
                window_size.width,
                window_size.height,
                &window,
            );
            Pixels::new(window_size.width, window_size.height, surface_texture)?
        };

        let theme = window.theme().unwrap_or(Theme::Dark);
        info!("Theme: {theme:?}");
        window.set_cursor(CursorIcon::Default);

        // Allow IME out of the box.
        let ime = true;
        window.set_ime_allowed(ime);

        let size = window.inner_size();
        let mut state = Self {
            is_drawing: false,
            #[cfg(target_os = "macos")]
            option_as_alt: window.option_as_alt(),
            cursor_grab: CursorGrabMode::None,
            pixels,
            window,
            theme,
            ime,
            cursor_position: Default::default(),
            modifiers: Default::default(),
            occluded: Default::default(),
            lines: Vec::new(),
        };

        state.resize(size);
        Ok(state)
    }

    pub fn toggle_ime(&mut self) {
        self.ime = !self.ime;
        self.window.set_ime_allowed(self.ime);
        if let Some(position) =
            self.ime.then_some(self.cursor_position).flatten()
        {
            self.window
                .set_ime_cursor_area(position, PhysicalSize::new(20, 20));
        }
    }

    pub fn minimize(&mut self) {
        self.window.set_minimized(true);
    }

    pub fn cursor_moved(&mut self, position: PhysicalPosition<f64>) {
        self.cursor_position = Some(position);
        if self.ime {
            self.window
                .set_ime_cursor_area(position, PhysicalSize::new(20, 20));
        }
    }

    pub fn cursor_left(&mut self) {
        self.cursor_position = None;
    }

    /// Toggle maximized.
    pub fn toggle_maximize(&self) {
        let maximized = self.window.is_maximized();
        self.window.set_maximized(!maximized);
    }

    /// Toggle window decorations.
    pub fn toggle_decorations(&self) {
        let decorated = self.window.is_decorated();
        self.window.set_decorations(!decorated);
    }

    /// Toggle fullscreen.
    pub fn toggle_fullscreen(&self) {
        let fullscreen = if self.window.fullscreen().is_some() {
            None
        } else {
            Some(Fullscreen::Borderless(None))
        };

        self.window.set_fullscreen(fullscreen);
    }

    /// Cycle through the grab modes ignoring errors.
    pub fn cycle_cursor_grab(&mut self) {
        self.cursor_grab = match self.cursor_grab {
            CursorGrabMode::None => CursorGrabMode::Confined,
            CursorGrabMode::Confined => CursorGrabMode::Locked,
            CursorGrabMode::Locked => CursorGrabMode::None,
        };
        info!("Changing cursor grab mode to {:?}", self.cursor_grab);
        if let Err(err) = self.window.set_cursor_grab(self.cursor_grab) {
            error!("Error setting cursor grab: {err}");
        }
    }

    #[cfg(target_os = "macos")]
    pub fn cycle_option_as_alt(&mut self) {
        self.option_as_alt = match self.option_as_alt {
            OptionAsAlt::None => OptionAsAlt::OnlyLeft,
            OptionAsAlt::OnlyLeft => OptionAsAlt::OnlyRight,
            OptionAsAlt::OnlyRight => OptionAsAlt::Both,
            OptionAsAlt::Both => OptionAsAlt::None,
        };
        info!("Setting option as alt {:?}", self.option_as_alt);
        self.window.set_option_as_alt(self.option_as_alt);
    }

    /// Resize the window to the new size.
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        info!("Resized to {size:?}");
        {
            self.pixels
                .resize_surface(size.width, size.height)
                .expect("failed to resize inner buffer");
        }
        self.window.request_redraw();
    }

    /// Change the theme.
    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
        self.window.request_redraw();
    }

    /// Show window menu.
    pub fn show_menu(&self) {
        if let Some(position) = self.cursor_position {
            self.window.show_window_menu(position);
        }
    }

    /// Drag the window.
    pub fn drag_window(&self) {
        if let Err(err) = self.window.drag_window() {
            info!("Error starting window drag: {err}");
        } else {
            info!("Dragging window Window={:?}", self.window.id());
        }
    }

    /// Drag-resize the window.
    pub fn drag_resize_window(&self) {
        let position = match self.cursor_position {
            Some(position) => position,
            None => {
                info!("Drag-resize requires cursor to be inside the window");
                return;
            }
        };

        let win_size = self.window.inner_size();
        let border_size = BORDER_SIZE * self.window.scale_factor();

        let x_direction = if position.x < border_size {
            ResizeDirection::West
        } else if position.x > (win_size.width as f64 - border_size) {
            ResizeDirection::East
        } else {
            // Use arbitrary direction instead of None for simplicity.
            ResizeDirection::SouthEast
        };

        let y_direction = if position.y < border_size {
            ResizeDirection::North
        } else if position.y > (win_size.height as f64 - border_size) {
            ResizeDirection::South
        } else {
            // Use arbitrary direction instead of None for simplicity.
            ResizeDirection::SouthEast
        };

        let direction = match (x_direction, y_direction) {
            (ResizeDirection::West, ResizeDirection::North) => {
                ResizeDirection::NorthWest
            }
            (ResizeDirection::West, ResizeDirection::South) => {
                ResizeDirection::SouthWest
            }
            (ResizeDirection::West, _) => ResizeDirection::West,
            (ResizeDirection::East, ResizeDirection::North) => {
                ResizeDirection::NorthEast
            }
            (ResizeDirection::East, ResizeDirection::South) => {
                ResizeDirection::SouthEast
            }
            (ResizeDirection::East, _) => ResizeDirection::East,
            (_, ResizeDirection::South) => ResizeDirection::South,
            (_, ResizeDirection::North) => ResizeDirection::North,
            _ => return,
        };

        if let Err(err) = self.window.drag_resize_window(direction) {
            info!("Error starting window drag-resize: {err}");
        } else {
            info!("Drag-resizing window Window={:?}", self.window.id());
        }
    }

    /// Change window occlusion state.
    pub fn set_occluded(&mut self, occluded: bool) {
        self.occluded = occluded;
        if !occluded {
            self.window.request_redraw();
        }
    }
}

fn draw_dot(
    buffer: &mut Buffer<DisplayHandle<'static>, Arc<Window>>,
    dot: &PhysicalPosition<u32>,
    width: u32,
) {
    let x = dot.x;
    let y = dot.y;
    for y in y.checked_sub(5).unwrap_or(0)..y + 5 {
        for x in x.checked_sub(5).unwrap_or(0)..x + 5 {
            let index = (width * y + x) as usize;
            if let Some(x) = buffer.get_mut(index) {
                *x = 0xff60a5fa;
            }
        }
    }
}

/// Drawing stuff.
impl WindowState {
    pub fn draw(&mut self) -> Result<()> {
        if self.occluded {
            info!("Skipping drawing occluded window={:?}", self.window.id());
            return Ok(());
        }
        let frame = self.pixels.frame_mut();
        let wwidth = self.window.inner_size().width;

        for line in &self.lines {
            draw(frame, wwidth, line);
        }
        // for dot in self.dots {
        // }

        self.window.pre_present_notify();
        Ok(self.pixels.render()?)
    }
}

fn dot(frame: &mut [u8], w: u32, dot: Point) {
    const RADIUS: u32 = 2;
    let x = dot.x as u32;
    let y = dot.y as u32;
    let left = x.checked_sub(RADIUS).unwrap_or(0);
    let right = x + RADIUS;
    let down = y + RADIUS;
    let up = y.checked_sub(RADIUS).unwrap_or(0);
    for x in left..=right {
        for y in up..=down {
            let i = ((y * w + x) * 4) as usize;
            frame[i] = 0xff;
            frame[i + 1] = 0x60;
            frame[i + 2] = 0xa5;
            frame[i + 3] = 0xfa;
        }
    }
}

fn line(frame: &mut [u8], w: u32, a: Point, b: Point) {
    const RADIUS: u32 = 2;
    const RES: f32 = 1.;
    let dist = a.distance(b);
    let x = b - a;
    let n = dist / RES;
    let inc = Point { x: x.x / n, y: x.y / n };
    let mut t = a;
    for _ in 0..(n as usize) {
        dot(frame, w, t);
        t += inc
    }
}

fn draw(frame: &mut [u8], width: u32, dots: &Vec<Point>) -> Option<()> {
    use tiny_skia_path::PathBuilder;
    let mut pb = PathBuilder::with_capacity(dots.len(), dots.len());
    let mut iter = dots.iter();
    match dots.len() {
        0 => return None,
        1 => return Some(dot(frame, width, dots[0])),
        _ => {}
    }
    let pt = iter.next().unwrap();
    pb.move_to(pt.x, pt.y);
    while let Some(&Point { x, y }) = iter.next() {
        pb.line_to(x, y)
    }
    let stroke = Stroke::default();
    let path = pb.finish()?.stroke(&stroke, 0.1)?;
    let mut iter = path.segments();

    let mut first_pt = Point::zero();
    let mut last_pt = Point::zero();

    while let Some(segment) = iter.next() {
        let verb = iter.curr_verb();
        let verb2 = iter.next_verb();
        let mut last_pt2 = last_pt;
        match segment {
            PathSegment::MoveTo(p) => {
                first_pt = p;
                last_pt = p;
                last_pt2 = p;
            }
            PathSegment::LineTo(p) => {
                line(frame, width, last_pt, p);
                last_pt = p;
                last_pt2 = last_pt;
            }
            PathSegment::QuadTo(p0, p1) => {}
            PathSegment::CubicTo(p0, p1, p2) => {}
            PathSegment::Close => {}
        }
    }
    Some(())
}
