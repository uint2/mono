use crate::prelude::*;

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
