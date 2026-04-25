.PHONY: test lint format

test:
	cargo test --workspace

lint:
	cargo clippy --workspace --all-targets -- -D warnings

format:
	cargo fmt --all
