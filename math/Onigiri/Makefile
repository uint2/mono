LAKE_BUILD := lake build --log-level=warning

current: onigiri

onigiri:
	# slope generate Onigiri.DnF.Chapter1
	# lake build Onigiri.DnF.Chapter1
	slope generate Onigiri
	lake build Onigiri

focus: check
	lake build
	# slope build Onigiri

all: check
	slope build

check: generate
	slope check

generate:
	slope generate Onigiri.Defs
	# slope generate Onigiri.Defs.MA3209
	# slope generate Onigiri.Mathlib
	# slope generate Onigiri.Mathlib.Set
	# slope generate Onigiri.Mathlib.AccPt

sorry:
	rg sorry -t lean --colors 'match:fg:yellow' --colors 'line:fg:white'

# First-time Setup =============================================================

setup:
	lake exe cache get

install:
	make -C fmt install

# Running Lean Binaries ========================================================

.PHONY: dino
