SHELL := /bin/fish
.PHONY: help clean build

help: ## Show help for each target
	@grep -E '^[a-zA-Z_-]+:' $(MAKEFILE_LIST) | sed 's/:.*##/:/g' | column -t -s ':'

clean: ## Clean the project using cargo
	cargo clean

build: ## Build the project using cargo
	cargo build --release

lint: ## Lint the project using cargo
	cargo clippy --all-targets --all-features -- -D warnings

format: ## Format the project using cargo
	cargo fmt --all -- --check

test: ## Run tests using cargo
	cargo nextest run

