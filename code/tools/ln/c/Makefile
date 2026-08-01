MAKEFILE_PATH := $(realpath $(lastword $(MAKEFILE_LIST)))
MAKEFILE_DIR  := $(realpath $(dir $(MAKEFILE_PATH)))

BUILD_DIR := $(MAKEFILE_DIR)/target

VALGRIND_LOG := $(BUILD_DIR)/valgrind-log-%p.txt

VALGRIND_FLAGS :=
VALGRIND_FLAGS += --trace-children=yes
VALGRIND_FLAGS += --show-error-list=yes
VALGRIND_FLAGS += --leak-check=full
VALGRIND_FLAGS += --show-leak-kinds=all
VALGRIND_FLAGS += --log-file=$(VALGRIND_LOG)
# VALGRIND_FLAGS += --xml=yes
# VALGRIND_FLAGS += --xml-file=valgrind.xml

current: run

configure:
	cmake -DCMAKE_INSTALL_PREFIX=$(HOME)/.local -S . -B $(BUILD_DIR)

build:
	cmake --build $(BUILD_DIR)

install: build
	cmake --install $(BUILD_DIR)

run: install
	git -C $(HOME)/mono ln --bound --all -n 100

valgrind-clear:
	-@rm -f $(BUILD_DIR)/valgrind-log*.txt

v0: valgrind-clear
	-valgrind $(VALGRIND_FLAGS) -- git -C ~/repos/alatty ln --all

v1: valgrind-clear
	valgrind $(VALGRIND_FLAGS) -- /usr/lib/git-core/git --no-pager log --all --graph --format='_%h %ar_%s_%C(auto)%D' --color=always
