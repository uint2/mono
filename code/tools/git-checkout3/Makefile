MAKEFILE_PATH := $(realpath $(lastword $(MAKEFILE_LIST)))
MAKEFILE_DIR  := $(realpath $(dir $(MAKEFILE_PATH)))

current: test

run:
	sh run.sh

build:
	cargo build

TEST_ONE := checkout_branch_matches_directory

test1: build
	cargo test $(TEST_ONE) -- --test-threads=1

test: build
	cargo test -- --test-threads=1

install:
		cargo install --locked --force --path .
