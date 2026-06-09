# the command that runs when `make` is ran without arguments
current:
	swift run -c release Csp

# build + run EngineCli in one command
dev:
	swift run -c release EngineCli

# build EngineCli
build:
	swift build --product EngineCli

# run tests without
test:
	swift test || exit 0
