MAKEFILE_PATH := $(realpath $(lastword $(MAKEFILE_LIST)))
MAKEFILE_DIR  := $(realpath $(dir $(MAKEFILE_PATH)))

current: test

run:
	sh run.sh

build:
	cargo build

TEST_ONE := tests::empty_directory
TEST_ONE :=

test: build
	cargo test $(TEST_ONE) -- --test-threads=1

install:
		cargo install --locked --force --path .
