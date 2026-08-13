# Quickstart: Flat land browse camera

```
git clone https://github.com/lambdasistemi/factory-tui
cd factory-tui
nix build .#cli
# inside tmux:
./result/bin/factory-tui --dump
./result/bin/factory-tui
```

Tests without a TUI:

```
nix develop -c cargo test
just ci
```

Read the law first: `.specify/memory/constitution.md`, then `docs/m1/`.
