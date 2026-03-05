.PHONY: run build test clean

run:
	cargo watch -x run

build:
	cargo build --release

test:
	cargo test

clean:
	cargo clean
