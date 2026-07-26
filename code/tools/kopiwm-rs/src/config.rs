use crate::prelude::*;

const GRAY_1: &str = "#222222";
const GRAY_2: &str = "#444444";
const GRAY_3: &str = "#bbbbbb";
const ACCENT_400: &str = "#d8b4fe";

pub const COLOR_SCHEME: SchemeStateArray<Scheme<&'static str>> = {
    use {Scheme as S, SchemeState as SS};
    let mut z = SchemeStateArray::new();
    z.set(SS::Normal, S { fg: GRAY_3, bg: GRAY_1, border: GRAY_2 });
    z.set(SS::Selected, S { fg: GRAY_1, bg: ACCENT_400, border: ACCENT_400 });
    z.set(SS::Bar, S { fg: GRAY_3, bg: GRAY_2, border: GRAY_2 });
    z
};
