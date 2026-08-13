# Repository Agent Guide

## What this repo is

factory-tui is a popup browser over live tmux windows. The tree is
tmux as it exists: session → window → pane. A single-pane window is
that pane; a multi-pane window gets one row per pane. An optional local
config file may rewrite what a row displays and nothing else. This
repository does not contain any one operator's factory map.

## How to work here

- Install (operator host): `nix profile add github:lambdasistemi/factory-tui`
- Build: `nix build .#cli`
- Test / CI: `just ci`
- Do not bind `target/release`

## Skills

Activatable procedures live under `skills/`.

- `skills/factory-tui/` — first-run install, census, write local
  `config.toml`, bind F1, verify `--dump`

## First-run setup

If the operator wants to *use* factory-tui on this host, load
`skills/factory-tui/SKILL.md` and follow it. Write configuration only
to `$FACTORY_TUI_CONFIG` or `~/.config/factory-tui/config.toml`, never
into the repository.

## The one rule about configuration

Configuration is label-only. An ordered `[[reinterpreter]]` entry has a
`scope`, a `pattern` and a `label`; the first match rewrites one row's
text and keeps the raw tmux name visible on it. Nothing in a config file
may add, drop, merge, split, reorder or reparent a node — if a change
would let it, that change is wrong.
