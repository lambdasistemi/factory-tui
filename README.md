# factory-tui

A popup browser over the tmux windows on this host. With no config it
shows session → window. Enter jumps. The snapshot does not resize
the live window.

Optional projection lives in a file, not in the crate:

- `$FACTORY_TUI_CONFIG`
- else `~/.config/factory-tui/config.toml`

A missing file is not an error. Generic example:

https://github.com/lambdasistemi/factory-tui/blob/main/examples/projection.toml

An agent setting this up on a host should load
[AGENTS.md](AGENTS.md) and `skills/factory-tui/SKILL.md`.

## Install

Nix (supported path):

```
nix profile add github:lambdasistemi/factory-tui
```

Pin a tag with `github:lambdasistemi/factory-tui/v0.1.0`. From a
checkout: `nix build .#cli` then `just ci`. Do not bind
`target/release`.

Without Nix: Homebrew `lambdasistemi/tap` on Apple Silicon, or a
Linux binary from

https://github.com/lambdasistemi/factory-tui/releases/latest

## Run

```tmux
bind-key -n F1 display-popup -E -w 90% -h 90% factory-tui
bind-key S     display-popup -E -w 90% -h 90% factory-tui
```

`factory-tui --dump` prints the tree with no UI.

## License

[Apache-2.0](LICENSE).
