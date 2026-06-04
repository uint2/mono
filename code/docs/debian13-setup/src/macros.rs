macro_rules! repo {
    (https, $site:expr, $owner:expr, $repo:expr) => {
        concat!("https://", $site, "/", $owner, "/", $repo, ".git")
    };
    (ssh, $site:expr, $owner:expr, $repo:expr) => {
        concat!("git@", $site, ":", $owner, "/", $repo, ".git")
    };
    ($owner:expr, $repo:expr) => {
        concat!($owner, "/", $repo, ".git")
    };
}

macro_rules! github {
    (https, $owner:expr, $repo:expr $(,)?) => {
        repo!(https, "github.com", $owner, $repo)
    };
    (ssh, $owner:expr, $repo:expr $(,)?) => {
        repo!(ssh, "github.com", $owner, $repo)
    };
}

macro_rules! str_enum {
    ($name:ident, $(($enum:ident, $str:expr)),* $(,)?) => {
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
