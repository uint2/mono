use super::prelude::*;

/// Download and install APT packages.
#[derive(FromArgs)]
#[argh(subcommand, name = "apt")]
pub struct Apt {}

impl Target for Apt {
    /// NOTE: this requires `sudo` to already be installed.
    fn install(&self) {
        sh!("sudo", "apt-get", "update", "--yes");
        let mut apt = cmd!("sudo", "apt-get", "install", "--yes");

        // Build requirements for zsh.
        apt.args(["libncurses-dev"]);

        // Build requirements for neovim.
        apt.args([
            "cmake",
            "curl",
            "file", // for CPack to generate .deb files after building.
            "ninja-build",
        ]);

        // Build requirements for nvidia drivers.
        apt.args(["linux-headers-generic", "libglvnd-dev"]);

        // Build requirements for dwm.
        apt.args(["libx11-dev", "libxft-dev", "libfreetype-dev", "libfontconfig-dev"]);

        // Runtime requirements for QMK.
        apt.args([
            "diffutils",
            "libhidapi-hidraw0",
            "build-essential",
            "dos2unix",
            "unzip",
            "wget",
            "zip",
            "zstd",
        ]);

        // Runtime requirements for gpg with a hardware key.
        apt.args(["gnupg", "gnupg-agent", "scdaemon", "pcscd"]);

        // Runtime requirements yubikey manager CLI tool.
        apt.args(["libpcsclite-dev"]);

        // The standard few.
        apt.args(["git", "curl", "wget", "rsync", "man"])
            // for systemctl commands.
            .arg("polkitd")
            // Xorg stuff.
            .args([
                "xserver-xorg",
                "xinit",
                "x11-xserver-utils", // for "xset r rate 200 40" to work.
                "picom",             // compositor.
            ])
            // Without this, Google Docs looks hella wonky. The bullet circles are
            // disproportionately bigger. TODO: figure out exactly which subset we
            // need.
            .arg("fonts-recommended")
            // General development requirements.
            .args([
                "autoconf",
                "cmake",
                "linux-headers-generic",
                "ninja-build",
                "pkg-config",
                "xz-utils", // for decompressing using `xz` with `tar`.
                "zstd",     // commonly used compression library.
                "unzip",
                "zip",
                "libssl-dev", // for openssl.
                "woff2",      // for compressing fonts.
            ])
            // CLI search tools.
            .args(["ripgrep", "fd-find", "fzf"])
            // Audio drivers.
            .args(["alsa-utils", "pulseaudio"])
            // for NTP (time sync with the internet).
            // ```
            // systemctl enable systemd-timesyncd
            // timedatectl set-ntp true
            // timedatectl set-timezone Asia/Singapore
            // ```
            .arg("systemd-timesyncd")
            // Apps that I use.
            .args([
                "zsh",
                "kitty",
                "rofi",
                "xclip",
                "flameshot",
                "feh",
                "eza",
                "zathura",
                "pcmanfm",
                "nnn",
                "git-crypt",
                "clangd",
                "network-manager", // to help to connect to WiFi in a pinch.
            ]);

        // Wrap that up and send.
        apt.stdout("apt.stdout.txt").stderr("apt.stderr.txt").run().unwrap();
    }
}
