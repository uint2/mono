from base64 import b64decode
from subprocess import run

def deconflict(arr: list[str], i: int):
    pass

with open("text.txt", "r") as f:
    lines = f.readlines()
lines = [x.strip() for x in lines]
lines = lines[::-1]

original_last = lines[-1]

k = len(lines[-1]) - 1
print(lines[-1][k:])
while lines[-2].endswith(lines[-1][k:]):
    k -= 1

# k += 1 + diff * 4
lines[-1] = original_last[: 1 + k]

with open("all.zip", "wb") as fa:
    for i, line in enumerate(lines):
        i = i + 1
        # with open("output.z%02d" % i, "wb") as f:
        print(line[:20], "...", line[-20:])
        # f.write(b64decode(line))
        fa.write(b64decode(line))

run(("unzip", "all.zip"))
