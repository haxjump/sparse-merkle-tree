default: fmt clippy test bench-test check

test:
	cargo test --release --workspace --all-features

bench-test:
	cargo bench -- --test

clippy:
	cargo clippy --workspace

fmt:
	cargo fmt

check:
	cargo check --no-default-features
