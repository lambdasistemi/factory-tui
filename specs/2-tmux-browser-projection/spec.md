# Feature Specification: tmux browser and optional projection

**Feature Branch**: `feat/2-tmux-browser-projection`
**Created**: 2026-08-13
**Status**: Draft
**Input**: lambdasistemi/factory-tui#8

## User Scenarios & Testing

### User Story 1 - Browse tmux as tmux (Priority: P1)

An operator opens factory-tui with no config file. The tree is the
live multiplexer: sessions, then windows. Enter jumps to that window.
The snapshot still does not resize it.

**Why this priority**: A stranger must get a useful browser. The
product is not a private org chart.

**Independent Test**: Point `--dump` at a fake census of two sessions
and four windows. The text tree lists those session names as parents.

**Acceptance Scenarios**:

1. **Given** no config file, **When** the operator runs `factory-tui`
   or `factory-tui --dump`, **Then** every window sits under its tmux
   session.
2. **Given** a selected window, **When** they press Enter, **Then**
   the attached client switches to that session and window.
3. **Given** a preview is showing, **When** another client is attached
   to that window, **Then** that window's size does not change.

---

### User Story 2 - Host tables live in a file (Priority: P1)

An operator keeps session nicknames, infra session patterns, and
“this process counts as running” lists in a local file. The crate
source does not name their products.

**Why this priority**: Today's binary embeds this host. That is the
sanitization.

**Independent Test**: A golden census plus a tiny config that aliases
`shop` → `acme` shows `acme` in the dump. The same census with no
file shows `shop`. `rg` over `src/` finds none of the retired host
identifiers listed on #5.

**Acceptance Scenarios**:

1. **Given** `$FACTORY_TUI_CONFIG` or
   `~/.config/factory-tui/config.toml` with a session alias, **When**
   the operator dumps the tree, **Then** that alias is applied.
2. **Given** an infra pattern in that file, **When** a session name
   matches it, **Then** the window is not treated as a product folder
   unless a later child says otherwise via rules.
3. **Given** a fresh clone, **When** someone searches the sources for
   this host's nicknames, **Then** they find none.

---

### User Story 3 - Fold with rules in the same file (Priority: P2)

An operator writes ordered regex rules with named captures and a
folder path. Matching windows nest as they asked. A window that
matches no rule stays under its session.

**Why this priority**: Tables without a grammar still leave the
factory parser in Rust.

**Independent Test**: The generic example file plus a fake census
dumps a folded tree. The same census with no file stays session →
window.

**Acceptance Scenarios**:

1. **Given** the example config, **When** a window name matches a
   rule, **Then** it appears under the folders named by that rule's
   captures and the fold path.
2. **Given** a window that matches no rule, **When** the tree is
   built, **Then** it remains under its tmux session.
3. **Given** the example file in the repository, **When** a reader
   opens it, **Then** they see generic names only.

---

### User Story 4 - Docs match the binary (Priority: P2)

A new reader of the README learns they have a tmux browser, where
the config file goes, and that projection is optional.

**Why this priority**: A green binary with a factory-first README is
a failed change.

**Independent Test**: README and the docs home do not present a
host-specific factory as the product. The example file they point at
is the one the goldens use.

**Acceptance Scenarios**:

1. **Given** the README on the default branch, **When** a reader
   follows it, **Then** they can bind the popup and know the default
   tree is sessions and windows.
2. **Given** the docs page for the config file, **When** they copy
   the generic example, **Then** it matches the checked-in file.

## Requirements

- R1. No config ⇒ session → window (optional panes stay inside the
  window; they are not extra rungs unless a later change says so).
- R2. Config path: `$FACTORY_TUI_CONFIG`, else
  `~/.config/factory-tui/config.toml`. Missing file is not an error.
- R3. Config is data only: tables, ordered regex rules with named
  captures, a folder path. No scripts.
- R4. Unmatched windows stay under their session.
- R5. Repository example is generic. Host nicknames are local.
- R6. Preview does not resize. Jump uses the multiplexer.
- R7. One executable, existing release pipeline.
- R8. Constitution and README describe the shipped default.

## Success Criteria

- SC1. A person who cloned nothing can install a release whose
  default tree is tmux.
- SC2. `src/` contains no host product nicknames.
- SC3. One generic example config plus goldens prove both trees.

## Assumptions

- tmux remains the process host for this epic.
- Children merge in order: #5, then #6, then #7.

## Non-goals

- Dropping tmux.
- Implementing the unvalidated compositor research.
- Publishing this host's alias table.
- A plugin runtime in the config file.
