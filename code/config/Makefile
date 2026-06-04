MAKEFILE_PATH := $(realpath $(lastword $(MAKEFILE_LIST)))
MAKEFILE_DIR  := $(realpath $(dir $(MAKEFILE_PATH)))

attach:
	python3 $(MAKEFILE_DIR)/attach.py
	ls -l ~/.config
