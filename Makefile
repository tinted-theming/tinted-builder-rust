test: build
	@echo "------------"
	@echo "Running test"
	@echo "------------"
	cargo test --release

build:
	@echo "-------------"
	@echo "Running build"
	@echo "-------------"
	cargo build --release
	cargo deny check

install: 
	@echo "---------------"
	@echo "Installing deps"
	@echo "---------------"
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	cargo install --locked cargo-about
	cargo install --locked cargo-deny
