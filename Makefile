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
	docker run -d -p 5000:5000 --name registry registry:2
	docker image tag tw-blockchain/reconciliation-demo localhost:5000/reconciliation-demo
	docker push localhost:5000/reconciliation-demo

check:
	$(MAKE) clean
	$(MAKE) fmt
	$(MAKE) clippy
	$(MAKE) build
	$(MAKE) test

.PHONY: all build clean test fmt clippy check image
