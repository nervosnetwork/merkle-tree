default: ci

test:
	cargo test

ensure_no_std:
	cd tests/ensure_no_std && cargo rustc -- -C link-arg=-nostartfiles

ci: test ensure_no_std
