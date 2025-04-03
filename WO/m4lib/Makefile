SHELL := /bin/bash
.PHONY: run build test doctest clean help

# Default target
help: ## Show available commands
	@grep -E '^[a-zA-Z_-]+:' $(MAKEFILE_LIST) | sed 's/:.*##/:/g' | column -t -s ':'

run: ## Run the project
	cargo run

build: ## Build the project
	cargo build --release

test: ## Run unit and integration tests
	cargo test

doctest: ## Run documentation tests
	cargo test --doc

clean: ## Remove build artifacts
	cargo clean
