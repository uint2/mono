macro_rules! getter {
    ($field:ident, $ret:ty) => {
        pub const fn $field(&self) -> $ret {
            self.$field
        }
    };
    (&$field:ident, $ret:ty) => {
        pub const fn $field(&self) -> &$ret {
            &self.$field
        }
    };
}

/// An enum with a `&str` representation.
macro_rules! str_enum {
    ($name:ident, $(($enum:ident => $str:expr)),* $(,)?) => {
        #[allow(unused)]
        pub enum $name {
            $($enum),*
        }

        impl $name {
            pub const fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$enum => $str,)*
                }
            }
        }
    };
}
