use crate::git::GitRepo;

pub mod dwm {
    use super::*;

    pub const REPO: GitRepo = GitRepo::new(
        "https://github.com/nguyenvukhang/dwm.git",
        "bfc06870f25da37383255dd363ff2457d188ae6a",
    );
}

pub mod less {
    use super::*;

    pub const REPO: GitRepo = GitRepo::new(
        "https://github.com/gwsw/less.git",
        "86fc76d1e0e460f303619c050cfce3d29809975e",
    );
}

pub mod ln {
    use super::*;

    pub const REPO: GitRepo =
        GitRepo::new("https://github.com/nvkcc/ln.git", "main");
}

pub mod micromamba {
    pub const DOWNLOAD_URL: &str = concat!(
        "https://github.com/mamba-org/micromamba-releases",
        "/releases/download/2.6.1-0/micromamba-linux-64"
    );
}

pub mod neovim {
    str_enum!(
        Tag, //
        (V0_11_7, "v0.11.7"),
        (V0_12_2, "v0.12.2"),
    );

    pub const REMOTE_URL: &str = "https://github.com/neovim/neovim.git";
}

pub mod n {
    pub const SOURCE_URL: &str = "https://raw.githubusercontent.com/tj/n/f52d2172f12cd76f0efe9524690723f52ab74f40/bin/n";
}

pub mod nvidia {
    // Get driver download URLs from here:
    // https://nvidia.com/drivers

    str_enum!(
        Version,
        // Released Tue Apr 28, 2026; file size: 423.13 MB
        (V595_71_05, "595.71.05"),
    );

    impl Version {
        pub const fn num_bytes(&self) -> usize {
            // Manually update this after each new download.
            match self {
                Self::V595_71_05 => 413216,
            }
        }

        pub fn download_url(&self) -> String {
            format!(
                "https://us.download.nvidia.com/XFree86/{target}/{ver}/NVIDIA-{target}-{ver}.run",
                target = "Linux-x86_64",
                ver = self.as_str(),
            )
        }
    }
}
