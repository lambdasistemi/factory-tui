# factory-tui

A popup browser over the tmux windows on this host. Tree on the left,
a coloured snapshot of the selected window on the right, Enter jumps
there.

It does not replace tmux and it does not create windows. Name the
windows; the browser decodes them. A window that does not match the
grammar still appears, under *unscoped*.

## Lay out tmux

Sessions are bags of related work. Windows are the seats you want to
find. Panes stay inside a window; they are not extra tree rows.

```text
0-machine          window  machine
0-projects         one window per product, named for that product
<product>          window  orch                      (optional)
                   window  <repo>-ms<N>-<goal>       milestone desk
                   window  <repo>-e<E>-t<T>-<goal>   ticket on an epic
                   window  <repo>-no-epic-t<T>-<goal>
```

`0-machine`, `0-projects`, and other `0-*` sessions are infrastructure.
They show under *machine* / *infra*. Product folders are filled by
windows in a **non-`0-` session**.

| Kind | Pattern | Example |
|---|---|---|
| milestone desk | `[<repo>-]ms<N>-<goal>` | `acme-ms1-ship` |
| ticket on a milestone | `[<repo>-]ms<N>-t<id>-<goal>` | `acme-ms1-t12-docs` |
| epic | `[<repo>-]e<id>-<goal>` | `acme-e4-parser` |
| ticket on an epic | `[<repo>-]e<id>-t<id>-<goal>` | `acme-e4-t18-rename` |
| ticket, no epic | `[<repo>-]no-epic-t<id>-<goal>` | `acme-no-epic-t9-hotfix` |

`PARKED` anywhere in the name marks the seat parked. Rename as soon
as the lane has an id and a goal; generic names (`zsh`, `codex`)
fall into *unscoped*.

`factory-tui --dump` prints the tree. If a lane sits under *unscoped*
or *infra*, fix the session or the window name.

Full grammar and keys: [docs/using.md](docs/using.md).

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

## License

[Apache-2.0](LICENSE).
