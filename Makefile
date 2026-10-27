.PHONY: build
build: fmt
	cargo build


.PHONY: check
check: fmt
	cargo clippy --all-targets -- --no-deps -D warnings
	cargo machete
	cargo test --all-targets -- --show-output


.PHONY: fmt
fmt:
	cargo fmt --all


.PHONY: clean
clean:
	cargo clean


.PHONY: outdated
outdated:
	cargo install --locked cargo-outdated
	cargo outdated -R
