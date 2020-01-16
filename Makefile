all:
	$(MAKE) build

build:
	cargo build --release
	cp target/release/*.dylib reconciliation/tests/plugin/

clean:
	cargo clean

test:
	cargo test

fmt:
	rustup component add rustfmt --toolchain 1.40.0-x86_64-unknown-linux-gnu
	cargo fmt --all

clippy:
	rustup component add clippy --toolchain 1.40.0-x86_64-unknown-linux-gnu
	cargo clippy --all

check:
	$(MAKE) clean
	$(MAKE) fmt
	$(MAKE) clippy
	$(MAKE) build
	$(MAKE) test

.PHONY: all build clean test fmt clippy check
