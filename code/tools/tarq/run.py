from base64 import b64decode
from subprocess import run

delimiter = "::::::::::"

with open("text.txt", "r") as f:
    text = f.read().strip()

parts = text.split(delimiter)

with open("all.zip", "wb") as fa:
    for part in parts:
        fa.write(b64decode(part))

run(("unzip", "all.zip"))
