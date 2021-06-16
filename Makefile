.PHONY: install-deps
install-deps:
	sudo apt install -y pkg-config libssl-dev

.PHONY: build
build:
	cargo build

.PHONY: install
install:
	sudo cp target/debug/libess_pam.so /usr/lib
