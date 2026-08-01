LAKE_BUILD := lake build --log-level=warning

current: focus

focus: check
	slope build Munkres

all: check
	slope build

check: generate
	slope check

generate:
	slope generate Munkres.Defs
	slope generate Munkres.Defs.MA3209
	slope generate Munkres.Mathlib
	slope generate Munkres.Mathlib.Set
	slope generate Munkres.Mathlib.AccPt

sorry:
	rg sorry -t lean --colors 'match:fg:yellow' --colors 'line:fg:white'

# First-time Setup =============================================================

setup:
	lake exe cache get

install:
	make -C fmt install

# Running Lean Binaries ========================================================

.PHONY: dino
