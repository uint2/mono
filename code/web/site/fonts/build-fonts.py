from fontTools.subset import Subsetter, load_font, Options, save_font
from fontTools.ttLib import TTLibError

from shutil import rmtree, copy
from os import makedirs, path, walk, listdir, environ
from subprocess import run

__cwd__ = path.dirname(__file__)
__root__ = path.dirname(__cwd__)
__package_lock__ = path.join(__root__, "package-lock.json")
__build_dir__ = path.join(__cwd__, "build")
__output_dir__ = path.join(__root__, "src/assets/fonts")
assert path.exists(__package_lock__), "Root path may be wrong."


# Build this binary locally from https://github.com/google/woff2.
WOFF2_COMPRESS = "/home/khang/mono/code/external/woff2/woff2_compress"
WOFF2_COMPRESS = environ.get("WOFF2_COMPRESS", WOFF2_COMPRESS)


def gather_all_chars(dir_path):
    """
    Gets all (unicode) characters used inside of a directory.
    """

    charset = set(" ")
    file_read_count = 0
    for root, subdirs, files in walk(dir_path):
        if path.basename(root) in ["node_modules"]:
            subdirs[:] = []
            continue
        for file in files:
            with open(path.join(root, file), "rb") as f:
                raw = f.read()
            file_read_count += 1
            try:
                utf8string = raw.decode("utf-8")
            except UnicodeDecodeError:
                continue
            charset.update(utf8string)
    print("Scanned", file_read_count, "files")
    return sorted(charset)


def main():
    # build the set of characters that this entire repository uses.
    charset = gather_all_chars(__root__)

    options = Options()
    options.ignore_missing_glyphs = False

    subsetter = Subsetter()
    subsetter.populate(unicodes=map(ord, charset))

    font_dir = path.join(__cwd__, "original")
    rmtree(__build_dir__, ignore_errors=True)
    makedirs(__build_dir__, exist_ok=True)
    for filename in listdir(font_dir):
        input_path = path.join(font_dir, filename)
        output_path = path.join(__build_dir__, filename)
        if input_path.endswith(".woff2"):
            copy(src=input_path, dst=output_path)
            continue
        try:
            font = load_font(input_path, options=options)
        except TTLibError:
            continue
        subsetter.subset(font)
        save_font(font, output_path, options)
        if not output_path.endswith(".woff2"):
            run((WOFF2_COMPRESS, output_path))

    # clear the entire fonts/ directory in public/
    rmtree(__output_dir__, ignore_errors=True)
    makedirs(__output_dir__, exist_ok=True)
    for filename in listdir(__build_dir__):
        if not filename.endswith(".woff2"):
            continue
        filepath = path.join(__build_dir__, filename)
        copy(src=filepath, dst=__output_dir__)


if __name__ == "__main__":
    main()
