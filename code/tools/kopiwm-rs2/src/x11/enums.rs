use crate::C;
use crate::prelude::*;

macro_rules! repr_enum {
    (($name:ident, $int:ty), $(($enum:ident, $value:expr)),+ $(,)?) => {
        pub enum $name { $($enum,)* }
        impl $name {
            #[allow(unused)]
            pub const fn c(&self) -> $int {
                match self { $(Self::$enum => $value as $int,)* }
            }
        }
    };
}

repr_enum!(
    (JoinStyle, c_int),
    (Miter, C::JoinMiter),
    (Round, C::JoinRound),
    (Bevel, C::JoinBevel)
);

repr_enum!(
    (LineStyle, c_int),
    (Solid, C::LineSolid),
    (OnOffDash, C::LineOnOffDash),
    (DoubleDash, C::LineDoubleDash),
);

repr_enum!(
    (CapStyle, c_int),
    (NotLast, C::CapNotLast),
    (Butt, C::CapButt),
    (Round, C::CapRound),
    (Projecting, C::CapProjecting),
);

repr_enum!(
    (CloseMode, c_int),
    (DestroyAll, C::DestroyAll),
    (RetainPermanent, C::RetainPermanent),
    (RetainTemporary, C::RetainTemporary),
);

// Hard-coded while waiting for
//
//   - https://github.com/rust-lang/rust-bindgen/issues/2732
//   - https://github.com/jethrogb/rust-cexpr/pull/15
//
// The bug is that `rust-cexpr` just ignores #define macros that are casted.
//
// Relevant C code in <X.h>:
// ```c
// #define RevertToNone		(int)None
// #define RevertToPointerRoot	(int)PointerRoot
// #define RevertToParent		2
// ```
repr_enum!(
    (RevertTo, c_int),
    (None, C::None),
    (PointerRoot, C::PointerRoot),
    (Parent, C::RevertToParent),
);

repr_enum!(
    (GrabMode, c_int), //
    (Sync, C::GrabModeSync),
    (Async, C::GrabModeAsync),
);
