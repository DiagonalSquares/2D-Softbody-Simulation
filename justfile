fmt:
    cargo fmt --all

lint:
    cargo clippy --all --all-targets --all-features -- -D warnings

apply-fixes:
    cargo clippy --all --all-targets --all-features --fix --allow-dirty