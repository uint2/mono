LAKE_BUILD := lake build --log-level=warning

current: focus

focus: check
	slope build Vi

all: check
	slope build

check: generate
	slope check

generate:
	# slope generate Vi.Defs
	# slope generate Vi.Defs.MA3209
	# slope generate Vi.Mathlib
	# slope generate Vi.Mathlib.Set
	# slope generate Vi.Mathlib.AccPt

sorry:
	rg sorry -t lean --colors 'match:fg:yellow' --colors 'line:fg:white'

# First-time Setup =============================================================

setup:
	lake exe cache get

install:
	make -C fmt install

# Running Lean Binaries ========================================================

.PHONY: dino
