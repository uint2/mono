MAKEFILE_PATH := $(realpath $(lastword $(MAKEFILE_LIST)))
MAKEFILE_DIR  := $(realpath $(dir $(MAKEFILE_PATH)))

current: run

run:
	sh run.sh

build:
	cargo build

test: build
	PATH=$(MAKEFILE_LIST)/target/debug:$$PATH cargo test \
		-- --test-threads=1
