.PHONY: run build setup check fmt

run:
	cargo run

build:
	cargo build --release

fmt:
	cargo fmt --all

check:
	cargo fmt --all -- --check
	cargo clippy --all -- -D warnings

setup:
	@if [ ! -f .env ]; then \
		cp .env.example .env; \
		echo ".env created from .env.example"; \
	else \
		echo ".env already exists, skipping"; \
	fi
