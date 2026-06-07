/// NOTE: this requires `sudo` to already be installed.
pub fn install_apt_packages() {
    sh!("sudo", "apt", "update");
    let mut apt = cmd!("sudo", "apt", "install");

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
        ])
        // CLI search tools.
        .args(["ripgrep", "fd-find", "fzf"])
        // Audio drivers.
        .args(["alsa-utils", "pulseaudio"])
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
        ]);

    // Wrap that up and send.
    apt.stdout("apt.stdout.txt").stderr("apt.stderr.txt").run().unwrap();
}
