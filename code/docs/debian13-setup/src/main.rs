//! Setup Debian 13 the way that Khang sets up his workspace.
//!
//! This requires "sudo" to already be installed.

#[macro_use]
mod macros;
#[macro_use]
mod command;
mod apt;
mod cli;
mod git;
mod pins;
mod utils;

use git::GitRepo;
use utils::*;

use std::path::Path;
use std::{env, fs, io};

// Global configuration.

const NEOVIM_TAG: pins::neovim::Tag = pins::neovim::Tag::V0_11_7;
const NVIDIA_VER: pins::nvidia::Version = pins::nvidia::Version::V595_71_05;
/// Prefixed upon the home directory.
const N_PREFIX: &str = ".local/n";

fn clangd_install() {
    const ZIP_FILE: &str = "clangd.zip";
    utils::curl(
        ZIP_FILE,
        "https://github.com/clangd/clangd/releases/download/22.1.0/clangd-linux-22.1.0.zip",
    );
    sh!("unzip", ZIP_FILE);
    sh!("mv", "clangd_22.1.0/bin/clangd", home_dir().join(".local/bin/clangd"));
    let _ = fs::remove_file(ZIP_FILE);
    let _ = fs::remove_dir_all("clangd_22.1.0");
}

/// Install dwm from source.
fn dwm_install(source_dir: &Path) {
    pins::dwm::REPO.ensure_exists_at(source_dir);
    cmd!("make", "configure").current_dir(source_dir).run().unwrap();
    cmd!("make", "build").current_dir(source_dir).run().unwrap();
    cmd!("sudo", "make", "install").current_dir(source_dir).run().unwrap();
}

/// Install firefox from their PPA repository.
fn firefox_install() {
    const GPG_KEY_URL: &str =
        "https://packages.mozilla.org/apt/repo-signing-key.gpg";
    const GPG_KEY_PATH: &str = "/etc/apt/keyrings/packages.mozilla.org.asc";

    // Get the signing key.
    let output =
        cmd!("wget", "--quiet", GPG_KEY_URL, "-O-").collect_stdout().run();
    let signing_key =
        output.unwrap().stdout.expect("Unable to obtain signing key.");
    utils::echo(signing_key.trim(), GPG_KEY_PATH, true);
    // Add the newly added APT repository to our sources list:
    utils::echo(
        format!(
            "\
Types: deb
URIs: https://packages.mozilla.org/apt
Suites: mozilla
Components: main
Signed-By: {GPG_KEY_PATH}"
        )
        .trim(),
        "/etc/apt/sources.list.d/mozilla.sources",
        true,
    );
    // Configure APT to prioritize packages from the Mozilla repository:
    utils::echo(
        "\
Package: *
Pin: origin packages.mozilla.org
Pin-Priority: 1000"
            .trim(),
        "/etc/apt/preferences.d/mozilla",
        true,
    );

    sh!("sudo", "apt", "update");
    sh!("sudo", "apt", "install", "firefox");
}

/// Install less from source.
fn less_install(source_dir: &Path) {
    pins::less::REPO.ensure_exists_at(source_dir);
    cmd!("make", "-f", "Makefile.aut", "distfiles")
        .current_dir(source_dir)
        .run()
        .unwrap();
    cmd!("sh", "configure").current_dir(source_dir).run().unwrap();
    cmd!("make").current_dir(source_dir).run().unwrap();
    cmd!("sudo", "make", "install").current_dir(source_dir).run().unwrap();
}

/// Install ln from source.
fn ln_install(source_dir: &Path) {
    pins::ln::REPO.ensure_exists_at(source_dir);
    cmd!("make", "configure").current_dir(source_dir).run().unwrap();
    cmd!("make", "build").current_dir(source_dir).run().unwrap();
    cmd!("sudo", "make", "install").current_dir(source_dir).run().unwrap();
}

fn micromamba_install() {
    use pins::micromamba::DOWNLOAD_URL as URL;
    const OUTFILE: &str = "micromamba";

    curl(OUTFILE, URL);
    let shasum = cmd!("wget", "--quiet", format!("{URL}.sha256"), "-O-")
        .collect_stdout()
        .run()
        .unwrap();
    assert_eq!(shasum.stdout.unwrap().trim(), utils::sha256sum(OUTFILE));
    sh!("chmod", "u+x", "micromamba");
    fs::rename("micromamba", home_dir().join(".local/bin").join(OUTFILE))
        .unwrap();
    // Later on, run
    // micromamba create --name ml python=3.12 --channel=conda-forge
}

/// Install script for neovim.
fn neovim_install(source_dir: &Path) {
    let build_dir = source_dir.join("build");

    // Clone the repo.
    let repo = GitRepo::new(pins::neovim::REMOTE_URL, NEOVIM_TAG.as_str());
    repo.ensure_exists_at(source_dir);

    // Build neovim.
    cmd!("make", "CMAKE_BUILD_TYPE=Release")
        .current_dir(source_dir)
        .stdout("nvim-build.stdout.txt")
        .stderr("nvim-build.stderr.txt")
        .run()
        .unwrap();

    // Remove existing *.deb files.
    search_dir(&build_dir, |v| has_ext(v.path(), "deb"))
        .iter()
        .for_each(|v| fs::remove_file(v).unwrap());

    // Create a .deb package with the build outputs.
    cmd!("cpack", "-G", "DEB").current_dir(&build_dir).run().unwrap();

    let deb_files = search_dir(&build_dir, |v| has_ext(v.path(), "deb"));
    cmd!("sudo", "dpkg", "--install", &deb_files[0])
        .current_dir(&build_dir)
        .run()
        .unwrap();
}

fn n_install() {
    const INSTALLER_PATH: &str = "n-install.sh";
    let _ = fs::remove_file(INSTALLER_PATH);
    utils::curl(INSTALLER_PATH, pins::n::SOURCE_URL);
    let n_prefix = utils::home_dir().join(N_PREFIX);
    cmd!("bash", INSTALLER_PATH, "install", "lts")
        .env("N_PREFIX", &n_prefix)
        .run()
        .unwrap();

    if let pins::neovim::Tag::V0_11_7 = NEOVIM_TAG {
        let path = env::var("PATH").unwrap();
        // Install this so that neovim can use it.
        cmd!("npm", "install", "-g", "tree-sitter-cli@0.25.10")
            .env("N_PREFIX", &n_prefix)
            .env("PATH", format!("{}:{path}", n_prefix.display()))
            .run()
            .unwrap();
    }
}

mod nvidia {
    use super::*;
    use pins::nvidia::Version;

    const SCRIPT_PATH: &str = "nvidia.sh";

    /// Downloads the install script if it doesn't exist.
    fn ensure_script_exists(version: Version) {
        if get_file_size_with_du(SCRIPT_PATH) == version.num_bytes() {
            return println!("Skip the download!");
        }
        // Run the download.
        utils::curl(SCRIPT_PATH, &version.download_url());
    }

    const STANDARD_ARGS: [&str; 2] = ["--ui=none", "--rebuild-initramfs"];

    pub fn install(version: Version) {
        ensure_script_exists(version);
        let mut cmd = cmd!("sudo", "sh", SCRIPT_PATH);
        cmd.args(STANDARD_ARGS);
        cmd.arg("--no-questions");
        // Use MIT/GPL drivers instead of NVIDIA proprietary.
        cmd.args(["-M", "open"]);
        cmd.run().unwrap();
    }

    pub fn uninstall(version: Version) {
        ensure_script_exists(version);
        let mut cmd = cmd!("sudo", "sh", SCRIPT_PATH);
        cmd.args(STANDARD_ARGS);
        cmd.arg("--uninstall");
        cmd.run().unwrap();
    }
}

fn setup_qmk() {
    const INSTALLER: &str = "qmk-install.sh";
    utils::curl(INSTALLER, "https://install.qmk.fm");
    sh!("sh", INSTALLER, "--confirm", "--skip-package-manager");
    let _ = fs::remove_file(INSTALLER);
}

/// Gets my public key from github, and then tells gpg of the presence of the
/// private key on the currently plugged-in hardware key.
fn setup_yubikey() {
    const TMP_PUB_KEY_PATH: &str = "tmp.asc";
    utils::curl(TMP_PUB_KEY_PATH, "https://github.com/nguyenvukhang.gpg");
    sh!("gpg", "--import", TMP_PUB_KEY_PATH);
    sh!("gpg", "--card-status");
    sh!("gpg", "--list-secret-keys");
    let _ = fs::remove_file(TMP_PUB_KEY_PATH);
}

fn main() {
    utils::initialize_home_dir();

    use cli::Subcommand as SC;
    use cli::target::nvidia::Subaction as NS;

    let opts = cli::parse();

    let home_dir = env::home_dir().unwrap();
    let repo_dir = home_dir.join("repos");

    let _ = fs::create_dir_all(home_dir.join(".local/bin"));

    if let Err(err) = fs::create_dir(&repo_dir) {
        match err.kind() {
            io::ErrorKind::AlreadyExists => {}
            _ => return println!("Filesystem error: {err:?}"),
        }
    }

    match opts.subcommand {
        SC::Apt(_) => apt::install_apt_packages(),
        SC::Clangd(_) => clangd_install(),
        SC::DisableNouveau(_) => {
            utils::echo(
                "\
blacklist nouveau
options nouveau modeset=0",
                "/etc/modprobe.d/nouveau-blacklist.conf",
                true,
            );
        }
        SC::Dwm(_) => dwm_install(&repo_dir.join("dwm")),
        SC::Firefox(_) => {
            firefox_install();
        }
        SC::Less(_) => less_install(&repo_dir.join("less")),
        SC::Ln(_) => ln_install(&repo_dir.join("ln")),
        SC::Micromamba(_) => micromamba_install(),
        SC::Neovim(_) => neovim_install(&repo_dir.join("neovim")),
        SC::NVersionMgr(_) => n_install(),
        SC::Nvidia(opts) => match opts.subaction {
            NS::Install(_) => nvidia::install(NVIDIA_VER),
            NS::Uninstall(_) => nvidia::uninstall(NVIDIA_VER),
        },
        SC::Qmk(_) => setup_qmk(),
        SC::Yubikey(_) => setup_yubikey(),
    }
}
