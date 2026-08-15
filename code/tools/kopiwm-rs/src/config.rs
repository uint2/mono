use crate::C;
use crate::prelude::*;

pub type Coordinate = c_int;
pub type Distance = c_uint;

const GRAY_1: &str = "#222222";
const GRAY_2: &str = "#444444";
const GRAY_3: &str = "#bbbbbb";
const ACCENT_400: &str = "#d8b4fe";

pub const FONTS: &[&str] = &["sans:size=10.5"];

pub const COLOR_SCHEME: WindowColorStateArray<WindowColors<&str>> = {
    use {WindowColorState as WS, WindowColors as W};
    let mut z = WindowColorStateArray::new();
    z.set(WS::Normal, W { fg: GRAY_3, bg: GRAY_1, border: GRAY_2 });
    z.set(WS::Selected, W { fg: GRAY_1, bg: ACCENT_400, border: ACCENT_400 });
    z.set(WS::Bar, W { fg: GRAY_3, bg: GRAY_2, border: GRAY_2 });
    z
};

/// Factor of the master area size [0.05...0.95].
pub const MFACT: f32 = 0.5;

/// Number of clients in master area
pub const NMASTER: u8 = 1;

/// False means hide bar.
pub const SHOW_BAR: bool = true;

pub const BAR_HEIGHT: Distance = 20;

pub const BAR_POSITION: BarPosition = BarPosition::Top;

fn placeholder(arg: &Arg) {}

const MODKEY: c_uint = C::Mod4Mask;

macro_rules! key {
    ($mod:expr, $keysym:expr, $func:expr, $arg:expr) => {
        Key { modifier: $mod, keysym: $keysym, func: $func, arg: $arg }
    };
}

pub const KEYS: [Key; 1] = [key!(MODKEY, 0, placeholder, Arg::Int(0))];
