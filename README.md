# factory-tui

Browse an agent factory as a **tree of seats**, not as a tmux session
list.

A seat is one visible agent. On this host that is one tmux window. The
app is the index and the compositor: tree on the left, a coloured
snapshot of the selected seat on the right, Enter to jump.

Milestone 1 records why the land is flat and why views must not resize
agents:

https://lambdasistemi.github.io/factory-tui/

## Install

### macOS (Apple Silicon)

```
brew tap lambdasistemi/tap
brew install factory-tui
```

### Linux

Grab a single-file artifact from

https://github.com/lambdasistemi/factory-tui/releases/latest

(AppImage / DEB / RPM / static-musl tarball, x86_64 and aarch64). Asset
names carry the version, e.g. for v0.0.1 on x86_64:

```
curl -L https://github.com/lambdasistemi/factory-tui/releases/download/v0.0.1/factory-tui-0.0.1-x86_64-linux.AppImage -o factory-tui
chmod +x ./factory-tui
```

### From source (Nix)

```
nix build github:lambdasistemi/factory-tui
# or, from a clone:
nix build .#cli          # GC-rooted binary in ./result/bin/factory-tui
just ci                  # = nix flake check = CI
nix develop -c cargo test
```

## Run

Launch from a tmux popup. Bind it in your tmux config:

```tmux
# prefix + F1, or prefix + S
bind-key F1 display-popup -E -w 90% -h 90% factory-tui
bind-key S  display-popup -E -w 90% -h 90% factory-tui
```

`--dump` prints the live tree with no UI (must run inside tmux).

`target/release` from a dev shell is not a release: a store GC can
delete its glibc. Use `nix build .#cli`.

## License

[Apache-2.0](LICENSE).
