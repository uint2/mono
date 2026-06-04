MAKEFILE_PATH := $(realpath $(lastword $(MAKEFILE_LIST)))
MAKEFILE_DIR  := $(realpath $(dir $(MAKEFILE_PATH)))

CARGO_DEBUG_DIR := $(MAKEFILE_DIR)/target/debug

current: bound

debug:
	cargo build
	PATH=$(CARGO_DEBUG_DIR):$$PATH git -C ~/repos/math ln -n 200

bound:
	cargo build
	PATH=$(CARGO_DEBUG_DIR):$$PATH git -C ~/repos/math ln --bound

install:
	cargo install --locked --force --path .
