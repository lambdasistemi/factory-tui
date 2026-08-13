# Spike helpers. Prefer `nix develop -c just …`.

default: dump

build:
    cargo build --release

dump:
    cargo run --release -- --dump

tui:
    cargo run --release

fmt:
    cargo fmt --all

test:
    cargo test
