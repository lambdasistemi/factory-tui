# Repository Agent Guide

## What this repo is

factory-tui is a popup browser over live tmux windows. Default tree
is session → window. An optional local config file may fold names
into another tree. This repository does not contain any one
operator's factory map.

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
