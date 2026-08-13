# factory-tui

The operator's index over a factory of visible agent seats.

The factory is a tree of work. Terminal multiplexers only have sessions,
windows, and panes. This program is the map that makes the first tree
walkable, and the compositor that can look at several seats at once
without turning them back into splits.

## Milestone 1 — Flat terminal land

M1 records the research that decides the product:

- [Flat terminal land](m1/flat-land.md) — seats are windows; sessions and
  panes are not factory structure.
- [User-controlled compositor](m1/compositor.md) — views are cameras;
  observers do not resize; attach is one seat, full glass.
- [Decisions](m1/decisions.md) — the rejected alternatives and why.

The standing prototype in this repository is a browse camera: a tree on
the left, a coloured snapshot of one seat on the right, Enter to jump.

## Install

Tagged releases ship Linux AppImage / DEB / RPM / static-musl tarballs
and a Homebrew formula on Apple Silicon:

```
brew tap lambdasistemi/tap
brew install factory-tui
```

https://github.com/lambdasistemi/factory-tui/releases/latest

From source: `nix build github:lambdasistemi/factory-tui`.

## Run

Inside tmux: `F1` or `prefix + S` opens the browser (local bind).

```
nix develop -c just build
```
