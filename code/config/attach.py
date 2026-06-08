from os import path
from urllib.request import urlretrieve
from dataclasses import dataclass
import os

HOME = path.expanduser("~")
CONFIG_DIR = path.join(HOME, ".config")
CWD = path.dirname(__file__)


@dataclass
class Link:
    src: str
    "The source directory."

    dst: str
    "The target directory."

    def symlink(self):
        try:
            os.remove(self.dst)
        except FileNotFoundError:
            pass
        os.symlink(src=self.src, dst=self.dst)


links = [
    Link(src=path.join(CWD, "nvim"), dst=path.join(CONFIG_DIR, "nvim")),
    Link(src=path.join(CWD, "zsh/.zshrc"), dst=path.join(HOME, ".zshrc")),
    Link(src=path.join(CWD, "@/git/config"), dst=path.join(HOME, ".gitconfig")),
    Link(src=path.join(CWD, "@/xorg/.xinitrc"), dst=path.join(HOME, ".xinitrc")),
    Link(src=path.join(CWD, "@/kitty"), dst=path.join(CONFIG_DIR, "kitty")),
    Link(src=path.join(CWD, "@/htop"), dst=path.join(CONFIG_DIR, "htop")),
    Link(src=path.join(CWD, "@/flameshot"), dst=path.join(CONFIG_DIR, "flameshot")),
]


def link_dotfiles():
    for i in range(len(links)):
        v = links[i]
        for u in links[:i]:
            assert v.dst != u.dst, "Conflict in target."
        assert path.exists(v.src), "Source of link should exist."
    # Yes, TOCTOU, but come on. Doing the links after all these checks ensures
    # that all checks pass before anything is done.
    for link in links:
        link.symlink()


def photo_link(sha: str, file: str) -> str:
    "Gets a download url from the uint2/photos repository."
    return "/".join(("https://raw.githubusercontent.com/uint2/photos", sha, file))


def download_files():
    print("Downloading wallpaper...")
    urlretrieve(
        photo_link(
            "9ea5fdbd460b3b77cd50bc432169446ce5d58a31",
            "b78e47dd63166dd1ec381951defb3a633521457e.jpg",
        ),
        path.join(HOME, ".local/share/wall.jpg"),
    )


def main():
    link_dotfiles()
    download_files()


if __name__ == "__main__":
    main()
