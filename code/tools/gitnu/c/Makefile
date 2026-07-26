MAKEFILE_PATH := $(realpath $(lastword $(MAKEFILE_LIST)))
MAKEFILE_DIR  := $(realpath $(dir $(MAKEFILE_PATH)))

RED := $(shell echo "\033[31m")
YELLOW := $(shell echo "\033[33m")
END := $(shell echo "\033[m")

VALGRIND_LOG := $(MAKEFILE_DIR)/valgrind-log-%p.txt

VALGRIND_FLAGS :=
VALGRIND_FLAGS += --trace-children=yes
VALGRIND_FLAGS += --show-error-list=yes
VALGRIND_FLAGS += --leak-check=full
VALGRIND_FLAGS += --show-leak-kinds=all
VALGRIND_FLAGS += --log-file=$(VALGRIND_LOG)
# VALGRIND_FLAGS += --xml=yes
# VALGRIND_FLAGS += --xml-file=valgrind.xml

# One of: Debug | Release | RelWithDebInfo | MinSizeRel
CMAKE_BUILD_TYPE := Release

# One of: ON | OFF
BUILD_GITNV_TESTS := OFF

TEST_DIR := $(MAKEFILE_DIR)/target/Test
BUILD_DIR := $(MAKEFILE_DIR)/target/$(CMAKE_BUILD_TYPE)

DEV_DIR := ~/repos
DEV_DIR := ~/repos/alatty/kittens/ask

current: test

configure_:
	@echo "\e[33m-------- CONFIGURE CMAKE --------\e[m"
	cmake -G Ninja -S . \
		-DCMAKE_BUILD_TYPE=$(CMAKE_BUILD_TYPE) \
		-DBUILD_GITNV_TESTS=$(BUILD_GITNV_TESTS) \
		-B $(CONFIGURE_DIR)

configure: CONFIGURE_DIR := $(BUILD_DIR)
configure: configure_

configure-test: CONFIGURE_DIR := $(TEST_DIR)
configure-test: BUILD_GITNV_TESTS := ON
configure-test: configure_

# Requires that we run `configure` first.
build:
	cmake --build $(BUILD_DIR)

install: build
	cmake --install $(BUILD_DIR)

# Requires that we run `configure-test` first.
test:
	cmake --build $(TEST_DIR) --parallel 4
	# cd $(BUILD_DIR) && ctest # run CTest. That's one way of doing things.
	$(TEST_DIR)/git-nv-test

v: install
	-@rm -f $(MAKEFILE_DIR)/valgrind-log*.txt
	-cd $(DEV_DIR) && valgrind $(VALGRIND_FLAGS) -- git-nv status

fmt:
	git ls-files '*.c' '*.h' | xargs clang-format -i

.PHONY: test
