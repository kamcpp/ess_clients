.PHONY: install-deps
install-deps:
	sudo apt install -y pkg-config libssl-dev

.PHONY: build
build:
	cargo build
