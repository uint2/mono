from os import path
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
]


for i in range(len(links)):
    v = links[i]
    for u in links[:i]:
        assert v.dst != u.dst, "Conflict in target."
    assert path.exists(v.src), "Source of link should exist."
    v.symlink()
