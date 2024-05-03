publish: publish_dry
	@echo "---------------"
	@echo "Running publish"
	@echo "---------------"
	cargo publish

lib_publish_dry: lib_test
	@echo "-------------------"
	@echo "Running publish_dry"
	@echo "-------------------"
	cargo publish --dry-run --package tinted-builder

cli_publish_dry: cli_test
	@echo "-------------------"
	@echo "Running publish_dry"
	@echo "-------------------"
	cargo publish --dry-run --package tinted-builder-rust

lib_test: lib_build
	@echo "------------"
	@echo "Running test"
	@echo "------------"
	cargo test --package tinted-builder --release $(TINTED_BUILDER_RUST_TEST)

cli_test: cli_build
	@echo "------------"
	@echo "Running test"
	@echo "------------"
	cargo test --package tinted-builder-rust --release $(TINTED_BUILDER_RUST_TEST)

lib_build:
	@echo "-------------"
	@echo "Running build"
	@echo "-------------"
	cargo build --release --package tinted-builder
	cargo deny check

cli_build:
	@echo "-------------"
	@echo "Running build"
	@echo "-------------"
	cargo build --release --package tinted-builder-rust
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
