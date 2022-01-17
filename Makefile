prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cd contracts && cargo build --release --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/callercontract.wasm 2>/dev/null | true
	wasm-strip target/wasm32-unknown-unknown/release/targetcontract.wasm 2>/dev/null | true

clippy:
	cd contracts && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd contracts && cargo fmt -- --check

lint: clippy
	cd contracts && cargo fmt

clean:
	cd contracts && cargo clean
	rm Cargo.lock
