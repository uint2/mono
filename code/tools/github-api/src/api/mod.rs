macro_rules! expose {
    ($module:ident) => {
        mod $module;
        #[allow(unused)]
        pub use $module::*;
    };
}

expose!(list_user_repos);
