macro_rules! c {
    ($struct:ident, $field:tt) => {
        impl $struct {
            pub const fn c(&self) -> *mut C::$struct {
                self.$field.as_ptr()
            }
        }
    };
}

macro_rules! make_new {
    ($struct:ident) => {
        impl $struct {
            pub fn new(value: *mut C::$struct) -> Option<Self> {
                NonNull::new(value).map(Self)
            }
        }
        c!($struct, 0);
    };
}

mod fcpattern;
mod xftfont;
mod xmodifierkeymap;

pub use fcpattern::FcPattern;
pub use xftfont::XftFont;
pub use xmodifierkeymap::XModifierKeymap;
