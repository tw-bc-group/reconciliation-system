all:
	$(MAKE) build

build:
	cargo build --release
	cp target/release/*.dylib reconciliation/tests/plugin/

clean:
	cargo clean

test: build
	cargo test

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all

image:
	docker build -t tw-blockchain/reconciliation-demo .

publish:
	docker image tag tw-blockchain/reconciliation-demo localhost:5000/reconciliation-plugin
	docker push localhost:5000/reconciliation-plugin

check:
	$(MAKE) clean
	$(MAKE) fmt
	$(MAKE) clippy
	$(MAKE) build
	$(MAKE) test

.PHONY: all build clean test fmt clippy check image
