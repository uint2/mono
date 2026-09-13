macro_rules! x {
    ($fn:ident ($($arg:expr),*)) => {
        unsafe { c::$fn ( $($arg,)*) }
    };
}

macro_rules! next {
    ($dst:ident = $src:ident.$field:ident) => {
        let next = $src.read().unwrap().$field.clone();
        $dst = next
    };
}
