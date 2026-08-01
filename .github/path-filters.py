from os import path
from sys import argv

__rfile__ = path.realpath(__file__)
__cwd__ = path.dirname(__rfile__)
__root__ = path.dirname(__cwd__)

print("file:", __rfile__)
print("cwd:", __cwd__)
print("root:", __root__)

# Path to this particular file from the project root.
__file_rel__ = path.relpath(__rfile__, __root__)

OUTPUT_FILE = path.join(__cwd__, "path-filters.yml")
INPUT_FILE = path.join(__cwd__, "project-map.txt")

with open(INPUT_FILE, "r") as f:
    pairs2 = f.read()

pairs = [
    (path.strip(), name.strip())
    for path, name in (x.split(":", maxsplit=1) for x in pairs2.strip().splitlines())
]


def print2(*v, file):
    print(*v, file=file)
    print(*v)


def main(command: str | None):
    if command == "generate":
        with open(OUTPUT_FILE, "w") as f:
            for subpath, key in pairs:
                print2(f"{key}:", file=f)
                print2(f'  - "{subpath}/**/*"', file=f)
                print2(f"  - {__file_rel__}", file=f)
                print2(f"  - .github/workflows/ci.yml", file=f)
                abs_subpath = path.join(__root__, subpath)
                assert path.isdir(abs_subpath), (
                    "Subpath does not exist!\n" + abs_subpath
                )


if __name__ == "__main__":
    command = None if len(argv) <= 1 else argv[1]
    main(command)
