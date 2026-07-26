MAKEFILE_PATH := $(realpath $(lastword $(MAKEFILE_LIST)))
MAKEFILE_DIR  := $(realpath $(dir $(MAKEFILE_PATH)))

MAKEFLAGS += --no-print-directory
BUILD_DIR := $(MAKEFILE_DIR).build
CLI_BIN   := $(BUILD_DIR)/git-nu
TEST_BIN  := $(BUILD_DIR)/gitnu_tests

# CMAKE_FLAGS := -DCMAKE_BUILD_TYPE=Release

all: run

gen_cmd:
	python3 gen_cmd.py
	@clang-format -i src/git_cmd.h
	@clang-format -i src/git_cmd.cpp

configure:
	@cmake $(CMAKE_FLAGS) -S . -B $(BUILD_DIR)

build: configure
	@echo '-------------- BUILD --------------'
	@cmake --build $(BUILD_DIR) -j --target git-nu

CMD := git -C ~/v/math nu status
CMD := git -c color.status=never nu status

run: gen_cmd build
	@echo '--------------- RUN ---------------'
	sh sandbox.sh

fmt:
	fd --type file --extension cpp --extension h | xargs clang-format -i
