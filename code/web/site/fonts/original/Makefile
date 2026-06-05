# This directory should only contain fonts downloaded directly from source, and
# never edited.

SHASUM_FILE := sha256.txt

verify:
	sha256sum --check $(SHASUM_FILE)

generate:
	find * -name '*.ttf' -or -name '*.otf' -or -name '*.woff2' -or -name '*.woff' \
		| xargs sha256sum > $(SHASUM_FILE)
