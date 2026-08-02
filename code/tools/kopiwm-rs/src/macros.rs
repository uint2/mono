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
