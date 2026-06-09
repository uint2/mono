MAKEFILE_PATH := $(realpath $(lastword $(MAKEFILE_LIST)))
MAKEFILE_DIR := $(dir $(MAKEFILE_PATH))
BUILD_DIR := $(MAKEFILE_DIR)target

current: run

configure:
	cmake -B $(BUILD_DIR) -GNinja -S .

build:
	cmake --build $(BUILD_DIR)

run: build
	$(BUILD_DIR)/sudoku
