nothing: # to prevent loops

build-release:
	cargo build --release

v: build-release
	valgrind -- ./target/release/t

dev: run
run:
	cargo run

install:
	cargo install --locked --force --path .
