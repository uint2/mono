import sys
from os import listdir, path

__cwd__ = path.dirname(__file__)
ROOT = path.dirname(__cwd__)

icon_paths = listdir(path.join(ROOT, "icons/feather"))

with open(path.join(__cwd__, "icons.py"), "w") as sys.stdout:
    print("from typing import Literal")
    print("Icon = Literal[")
    for ip in icon_paths:
        print('"%s",' % ip.removesuffix(".svg"))
    print("]")
