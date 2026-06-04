use argh::FromArgs;

#[derive(FromArgs)]
/// Top-level command.
pub struct TopLevel {
    #[argh(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum Subcommand {
    Apt(target::Apt),
    Clangd(target::Clangd),
    DisableNouveau(target::DisableNouveau),
    Dwm(target::Dwm),
    Firefox(target::Firefox),
    Less(target::Less),
    Ln(target::Ln),
    Micromamba(target::Micromamba),
    Neovim(target::Neovim),
    NVersionMgr(target::NVersionMgr),
    Nvidia(target::Nvidia),
    Qmk(target::Qmk),
    Yubikey(target::Yubikey),
}

pub mod target {
    use argh::FromArgs;

    macro_rules! make_subcommand {
        ($(#[$outer:meta])*
            $struct:ident, $arg:expr) => {
            #[derive(FromArgs)]
            $(#[$outer])*
            #[argh(subcommand, name = $arg)]
            pub struct $struct {}
        };
    }

    make_subcommand!(
    /// Install APT packages.
    Apt, "apt");

    make_subcommand!(
    /// Install clangd.
    Clangd, "clangd");

    make_subcommand!(
    /// Disable nouveau from loading.
    DisableNouveau, "disable-nouveau");

    make_subcommand!(
    /// Download and install dwm.
    Dwm, "dwm");

    make_subcommand!(
    /// Download and install firefox.
    Firefox, "firefox");

    make_subcommand!(
    /// Download and install less.
    Less , "less");

    make_subcommand!(
    /// Download and install ln (my custom git logger).
    Ln , "ln");

    make_subcommand!(
    /// Download and install micromamba.
    Micromamba , "micromamba");

    make_subcommand!(
    /// Download and install neovim.
    Neovim , "neovim");

    make_subcommand!(
    /// Download and install the `n` node version manager.
    NVersionMgr , "n");

    /// Download and install NVIDIA drivers.
    #[derive(FromArgs)]
    #[argh(subcommand, name = "nvidia")]
    pub struct Nvidia {
        #[argh(subcommand)]
        pub subaction: nvidia::Subaction,
    }

    pub mod nvidia {
        use super::*;

        #[derive(FromArgs)]
        #[argh(subcommand)]
        pub enum Subaction {
            Install(Install),
            Uninstall(Uninstall),
        }

        make_subcommand!(
        /// Install.
        Install , "install");
        make_subcommand!(
        /// Uninstall.
        Uninstall , "uninstall");
    }

    make_subcommand!(
    /// Setup qmk.
    Qmk , "qmk");

    make_subcommand!(
    /// Setup Yubikey.
    Yubikey , "yubikey");
}

pub fn parse() -> TopLevel {
    return argh::from_env();
}
