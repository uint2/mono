MAKEFILE_PATH := $(realpath $(lastword $(MAKEFILE_LIST)))
MAKEFILE_DIR  := $(realpath $(dir $(MAKEFILE_PATH)))

APP := Cycle
APP := CycleOneNote

TARGET_BIN := $(MAKEFILE_DIR)/../$(APP).app/Contents/MacOS/$(APP)
OUTPUT_BIN := $(MAKEFILE_DIR)/.build/release/intuous

debug:
	swift run

release:
	swift build --configuration release

fmt:
	swiftformat .

all:
	@make release
	@rm -f $(TARGET_BIN)
	@cp $(OUTPUT_BIN) $(TARGET_BIN)
