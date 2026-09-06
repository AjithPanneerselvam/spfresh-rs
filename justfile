set dotenv-load

build-sys:
    cargo build -p spfresh-sys

build:
    cargo build --workspace

check:
    cargo check --workspace

test:
    cargo test --workspace

example-smoke:
    cargo run --example smoke
