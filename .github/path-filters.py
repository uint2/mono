from os import path

__rfile__ = path.realpath(__file__)
__cwd__ = path.dirname(__rfile__)
__root__ = path.dirname(__cwd__)

print("file:", __rfile__)
print("cwd:", __cwd__)
print("root:", __root__)

__file_rel__ = path.relpath(__rfile__, __root__)

OUTPUT_FILE = path.join(__cwd__, "path-filters.yml")

# Lines are sorted alphabetically.
pairs2 = """
code/docs/debian13-setup                       :debian13
code/games/aquarium                            :aquarium
code/games/sudoku                              :sudoku
code/games/wordle                              :wordle
code/tools/c-bufreader                         :c-bufreader
code/tools/canvas-sync-cli                     :canvas-sync-cli
code/tools/diff-rs                             :diff-rs
code/tools/draw-rs                             :draw-rs
code/tools/git-checkout2                       :git-checkout2
code/tools/gitlab-api                          :gitlab-api
code/tools/gitnu/c                             :gitnu-c
code/tools/gitnu/rust                          :gitnu-rs
code/tools/kopiwm                              :kopiwm
code/tools/ln                                  :git-ln
code/tools/loan-payoff-strategy                :loan-payoff-strategy
code/tools/logger.zig                          :logger-zig
code/tools/make-rs                             :make-rs
code/tools/numerical-methods                   :numerical-methods
code/tools/quietr/rust                         :quietr
code/tools/rofi-pdf-search                     :rofi-pdf-search
code/tools/solid-rect                          :solid-rect
code/tools/stats-calc                          :stats-calc
code/tools/t-runner                            :t-runner
code/tools/t-runner/rust                       :t-runner-rs
code/tools/tailwind-rs                         :tailwind-rs
code/tools/tmux-fzf                            :tmux-fzf
code/tools/wacom-macos-precision-mode-daemon   :heliumd
code/tools/wacom-macos-precision-mode-gui      :helium
code/web/site                                  :personal-site
"""

pairs = [
    (path.strip(), name.strip())
    for path, name in (x.split(":", maxsplit=1) for x in pairs2.strip().splitlines())
]


def print2(*v, file):
    print(*v, file=file)
    print(*v)


with open(OUTPUT_FILE, "w") as f:
    for subpath, key in pairs:
        print2(f"{key}:", file=f)
        print2(f'  - "{subpath}/**/*"', file=f)
        print2(f"  - {__file_rel__}", file=f)
        print2(f"  - .github/workflows/ci.yml", file=f)
        abs_subpath = path.join(__root__, subpath)
        assert path.isdir(abs_subpath), "Subpath does not exist!\n" + abs_subpath
