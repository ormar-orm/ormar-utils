.PHONY: fmt check test build clean

fmt:
	cargo fmt

check:
	cargo clippy -- -D warnings

test:
	poetry run pytest tests/ -v

build:
	poetry run maturin develop

clean:
	cargo clean
	rm -rf target/

.DEFAULT_GOAL := help

help:
	@echo "Available targets:"
	@echo "  fmt       - Format code with rustfmt"
	@echo "  check     - Run clippy linter with strict warnings"
	@echo "  test      - Run tests with pytest"
	@echo "  build     - Build the extension module with maturin"
	@echo "  clean     - Clean build artifacts"
