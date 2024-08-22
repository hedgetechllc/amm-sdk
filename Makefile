.PHONY: all clean lib wasm publish format checknostd check test testunit testmidi

all:
	$(error You must specify one of the following targets: clean lib wasm publish format checknostd test testunit testmidi)

clean:
	cd ensure_no_std && cargo clean && rm -rf Cargo.lock
	cargo clean
	@rm -rf pkg target*

lib:
	cargo build --lib --release

wasm:
	wasm-pack build --target nodejs
# wasm-opt -Os(or -Oz) add_bg.wasm -o add.wasm

publish: wasm
	wasm-pack publish

format:
	cargo fmt

checknostd:
	cd ensure_no_std && cargo rustc -- -C link-arg=-nostartfiles

check:
	cargo clippy -- -W clippy::all -W clippy::correctness -W clippy::suspicious -W clippy::complexity -W clippy::perf -W clippy::style -W clippy::pedantic -A clippy::module_name_repetitions -A clippy::module_inception -A clippy::too_many_lines -D warnings

test:
	cargo run --release --bin test

testunit:
	cargo test -- --nocapture

testmidi:
	cargo test -- --nocapture test_midi_
