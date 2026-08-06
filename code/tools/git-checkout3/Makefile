MAKEFILE_PATH := $(realpath $(lastword $(MAKEFILE_LIST)))
MAKEFILE_DIR  := $(realpath $(dir $(MAKEFILE_PATH)))

current: run

run:
	sh run.sh

build:
	cargo build

TEST_ONE := t7

test1: build
	cargo test $(TEST_ONE) -- --test-threads=1

test: build
	cargo test

install:
		cargo install --locked --force --path .
