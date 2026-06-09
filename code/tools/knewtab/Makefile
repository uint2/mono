# https://addons.mozilla.org/en-US/developers/addon/api/key/
VERSION := $(shell cat version)

generate: generate-icons
	python3 -m tabs
	npm run format

tag:
	git tag -f knewtab-v$(VERSION)

generate-icons:
	python3 -m tabs.generate-icons

test:
	firefox newtab.html
