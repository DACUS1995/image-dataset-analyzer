

build-dev:
	cargo build

build-release:
	cargo build --release

run:
	cargo run -- --root-dir="assets" --timeit --trackit

test:
	cargo test
