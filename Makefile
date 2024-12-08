.PHONY: all clean setup lib wasm publish publishweb format checknostd check test testunit testmidi testwasm testweb

all:
	$(error You must specify one of the following targets: clean setup docs lib wasm publish publishweb format checknostd check test testunit testwasm testweb)

clean:
	cd ensure_no_std && cargo clean && rm -rf Cargo.lock
	cargo clean
	@rm -rf pkg target*

setup:
	rustup target add wasm32-unknown-unknown x86_64-unknown-none
	cargo install wasm-pack

docs:
	RUSTDOCFLAGS="--extend-css amm_sdk/assets/docs.css" cargo doc --workspace --no-deps --release --lib --exclude amm_internal --exclude amm_macros
	echo "<meta http-equiv=\"refresh\" content=\"0;url=amm_sdk/index.html\" />" > target/doc/index.html
	cp -r amm_sdk/assets/* target/doc/amm_sdk/
	mv target/doc/amm_sdk/fonts target/doc/

lib:
	cargo build --lib --release

wasm:
	wasm-pack build --target nodejs
# wasm-opt -Os(or -Oz) add_bg.wasm -o add.wasm

publish:
	cargo publish -p amm_internal
	cargo publish -p amm_macros
	cargo publish -p amm_sdk

publishweb: wasm
	wasm-pack publish

format:
	cargo fmt

checknostd:
	cd ensure_no_std && cargo build --target x86_64-unknown-none

check:
	cargo clippy -- -W clippy::all -W clippy::correctness -W clippy::suspicious -W clippy::complexity -W clippy::perf -W clippy::style -W clippy::pedantic -A clippy::module_name_repetitions -A clippy::module_inception -A clippy::too_many_lines -D warnings

test:
	cargo run --release --bin test

testunit:
	cargo test -- --nocapture

testwasm:
	wasm-pack test --node

testweb:
	wasm-pack build --target --web
	cp tests/wasm_index.html pkg/index.html
	npx live-server ./pkg
