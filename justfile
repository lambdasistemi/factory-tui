# Thin wrappers over the Nix gate. `nix flake check` == `just ci` == CI.

set unstable := true

default: ci

build:
    nix run .#build

test:
    nix run .#test

fmt:
    nix develop -c cargo fmt --all

fmt-check:
    nix run .#fmt-check

clippy:
    nix run .#clippy

deny:
    nix run .#deny

ci:
    nix run .#ci
