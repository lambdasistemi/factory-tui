# factory-tui

A popup browser over the tmux windows on this host. With no config it
shows session → window. Enter jumps. The snapshot does not resize
the live window.

Optional projection lives in a file, not in the crate:

- `$FACTORY_TUI_CONFIG`
- else `~/.config/factory-tui/config.toml`

A missing file is not an error. Generic example:

https://github.com/lambdasistemi/factory-tui/blob/main/examples/projection.toml

## Install

### macOS (Apple Silicon)

```
brew tap lambdasistemi/tap
brew install factory-tui
```

### Linux

https://github.com/lambdasistemi/factory-tui/releases/latest

### From source (Nix)

```
nix build github:lambdasistemi/factory-tui
nix build .#cli
just ci
```

Do not bind `target/release`.

## Run

```tmux
bind-key -n F1 display-popup -E -w 90% -h 90% factory-tui
bind-key S     display-popup -E -w 90% -h 90% factory-tui
```

`factory-tui --dump` prints the tree with no UI.

## License

[Apache-2.0](LICENSE).
