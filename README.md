# factory-tui

A popup browser over the tmux windows on this host: tree on the left,
a coloured snapshot of the selected window on the right, Enter to
jump.

It does not replace tmux. Name your windows; the browser decodes
them. How to lay the sessions out:

https://lambdasistemi.github.io/factory-tui/using/

## Install

### macOS (Apple Silicon)

```
brew tap lambdasistemi/tap
brew install factory-tui
```

### Linux

https://github.com/lambdasistemi/factory-tui/releases/latest

```
curl -L https://github.com/lambdasistemi/factory-tui/releases/download/v0.0.1/factory-tui-0.0.1-x86_64-linux.AppImage -o factory-tui
chmod +x ./factory-tui
```

### From source (Nix)

```
nix build github:lambdasistemi/factory-tui
nix build .#cli          # from a clone; GC-rooted
just ci
```

Do not bind `target/release`: a store GC can delete its glibc.

## Run

Inside tmux:

```tmux
bind-key -n F1 display-popup -E -w 90% -h 90% factory-tui
bind-key S     display-popup -E -w 90% -h 90% factory-tui
```

`factory-tui --dump` prints the tree with no UI.

## License

[Apache-2.0](LICENSE).
