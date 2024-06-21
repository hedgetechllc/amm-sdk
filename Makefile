.PHONY: all clean lib wasm format test

all:
	$(error You must specify one of the following targets: clean lib wasm format test)

clean:
	@rm -rf pkg target

lib:
	cargo build --lib --release

wasm:
	wasm-pack build --target nodejs
# wasm-opt -Os add_bg.wasm -o add.wasm

format:
	cargo fmt

test:
	cargo run --release --bin test
