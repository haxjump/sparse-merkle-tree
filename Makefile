default: fmt lint test

lint:
	cargo clippy --workspace
	cargo clippy --workspace --no-default-features

fmt:
	cargo +nightly fmt

test:
	rm -rf ~/.vsdb ${VSDB_BASE_DIR}
	cargo test --release --workspace --all-features

testall: test
	cargo bench -- --test
	cargo test --release --workspace --all-features -- --ignored

bench:
	cargo bench
