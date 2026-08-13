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
nix develop -c just build
nix develop -c just test
```

`--dump` prints the live tree with no UI (must run inside tmux).

## License

[Apache-2.0](LICENSE).
