macro_rules! x {
    ($fn:ident ($($arg:expr),*)) => {
        unsafe { c::$fn ( $($arg,)*) }
    };
}
