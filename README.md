# factory-tui

A popup browser over the tmux windows on this host. The tree is tmux
itself: session → window → pane. Enter jumps. The snapshot does not
resize the live window.

A window with one pane is that pane — no redundant row. A window with
several panes gets one row per pane, each jumping to exactly that pane.

An optional file may rewrite what a row *says*. Nothing may change what
the tree *is*:

- `$FACTORY_TUI_CONFIG`
- else `~/.config/factory-tui/config.toml`

A missing file is not an error. Generic example:

https://github.com/lambdasistemi/factory-tui/blob/main/examples/config.toml

Ordered `[[reinterpreter]]` entries carry a `scope`
(`session`, `window`, `pane`), a `pattern`, and a `label`. The first
entry that matches a row rewrites that row's text and keeps the raw tmux
name visible beside it. An unmatched name renders raw.

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

`factory-tui --dump` prints the tree with no UI. Each window row carries
one `[window=@id]` marker and each pane exactly one `[pane=%id]` marker,
so a census taken straight from tmux can be compared against it.

`factory-tui --version` prints what the binary is and quits — no tmux,
no UI:

```
factory-tui 0.1.0 (revision 40c0c518992af708b9e140bd846a3d073fb9799c)
```

The popup shows the same line in its bottom chrome. Provenance details:
[docs/using.md](docs/using.md).

## License

[Apache-2.0](LICENSE).
