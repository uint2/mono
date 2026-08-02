MAKEFILE_PATH := $(realpath $(lastword $(MAKEFILE_LIST)))
MAKEFILE_DIR  := $(realpath $(dir $(MAKEFILE_PATH)))

BINARY_PATH   := $(MAKEFILE_DIR)/target/debug/kopiwm
XINITRC       := $(MAKEFILE_DIR)/xinitrc

STDOUT_LOG := $(MAKEFILE_DIR)/stdout.log
STDERR_LOG := $(MAKEFILE_DIR)/stderr.log

build:
	RUSTFLAGS="$$RUSTFLAGS -A unused" cargo build

run: build
	KOPIWM_BINARY=$(BINARY_PATH) ./startup.sh
	nvim $(STDERR_LOG)

# run: build
# 	KOPIWM_BINARY=$(BINARY_PATH) XINITRC=$(XINITRC) startx \
# 		-- -keeptty \
# 		>$(STDOUT_LOG) \
# 		2>$(STDERR_LOG)
