# factory-tui

Browse an agent factory as a **tree of seats**, not as a tmux session
list.

A seat is one visible agent. On this host that is one tmux window. The
app is the index and the compositor: tree on the left, a coloured
snapshot of the selected seat on the right, Enter to jump.

Milestone 1 records why the land is flat and why views must not resize
agents:

https://lambdasistemi.github.io/factory-tui/

## Build

```
nix build .#cli          # GC-rooted binary in ./result/bin/factory-tui
just ci                  # = nix flake check = CI
nix develop -c cargo test
```

`--dump` prints the live tree with no UI (must run inside tmux).

`target/release` from a dev shell is not a release: a store GC can
delete its glibc. Use `nix build .#cli`.

## License

[Apache-2.0](LICENSE).
