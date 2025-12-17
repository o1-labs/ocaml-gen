OCAMLFORMAT_VERSION = 0.28.1

setup-ocaml-deps:
		opam install . -y --deps-only --with-test
		opam install ocamlformat.$(OCAMLFORMAT_VERSION) -y


lint: ## Lint the code
		cargo clippy --all-features --all-targets --tests $(CARGO_EXTRA_ARGS) -- -W clippy::all -D warnings

build: ## Build the project
		cargo build --all-targets --all-features


release: ## Build the project in release mode
		cargo build --release --all-targets --all-features

help: ## Ask for help!
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

check-format: ## Check the code formatting
		rustup run nightly cargo fmt -- --check
		cargo sort --check
		dune build @fmt

format: ## Format the code
		rustup run nightly cargo fmt
		cargo sort

.PHONY: build check-format format help lint release setup-ocaml-deps
