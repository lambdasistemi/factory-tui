# M1 — Flat terminal land with a user-controlled compositor

!!! warning "Unvalidated experiment"
    This page is research, not operating advice. Today's binary is a
    browser over ordinary tmux windows. See [Using](../using.md).

**Outcome.** The factory is a tree of seats. A seat is one visible
agent process (today: one tmux window). This program is the index and
the compositor. Sessions and panes are not factory structure. Views
are cameras; they do not resize agents.

**Observable test.** A reader of `docs/m1/` can name seat, view, and
observer. `nix develop -c cargo test` passes. Inside tmux the binary
shows a factory tree (not a session list) and opens a seat without
resizing other seats.

**Artifact.** This document set plus the browse-camera prototype
(`factory-tui`). Temporary: later milestones own a Nix-rooted package
and the standing `--serve` embed.

**Scope boundary.** M1 does not ship a seat server, a live mosaic that
attaches four windows, or a tablet-native second factory UI.

**State page.** https://github.com/lambdasistemi/factory-tui/wiki/M1-State
