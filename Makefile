publish: publish_dry
	@echo "---------------"
	@echo "Running publish"
	@echo "---------------"
	cargo publish

publish_dry: test
	@echo "-------------------"
	@echo "Running publish_dry"
	@echo "-------------------"
	cargo publish --dry-run

test: build
	@echo "------------"
	@echo "Running test"
	@echo "------------"
	cargo test --release $(TINTED_BUILDER_RUST_TEST)

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
	@if [ -z "$$(command -v cargo)" ]; then \
		echo "Installing rustup"; \
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; \
	else \
		echo "rustup already installed"; \
	fi
	@if [ ! "$$(cargo about --version &>/dev/null)" ]; then \
		echo "Installing cargo about"; \
		cargo install --locked cargo-about; \
	else \
		echo "cargo-about already installed"; \
	fi
	@if [ ! "$$(cargo deny --version &>/dev/null)" ]; then \
		echo "Installing cargo deny"; \
		cargo install --locked cargo-deny; \
	else \
		echo "cargo-deny already installed"; \
	fi
