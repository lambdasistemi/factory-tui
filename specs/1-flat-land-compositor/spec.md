# Feature Specification: Flat land browse camera

**Feature Branch**: `feat/m1-flat-land-compositor`
**Created**: 2026-08-13
**Status**: Delivered (retroactive record)
**Input**: lambdasistemi/factory-tui#1

## User Scenarios & Testing

### User Story 1 - Walk the factory, not the session list (Priority: P1)

An operator sitting at a busy host opens the factory index and sees
work grouped as machine → project → milestone → epic → ticket → role.
They do not have to decode session names or hunt for a desk window.

**Why this priority**: Without this, the rest of the product is still
`prefix + s`.

**Independent Test**: Inside a live multiplexer, run the program and
confirm the left-hand tree is a factory tree. A milestone desk is a
named row. Opening that row does not require listing sessions first.

**Acceptance Scenarios**:

1. **Given** several product seats on the host, **When** the operator
   opens the index, **Then** they see project and milestone groupings
   rather than a flat session list.
2. **Given** a milestone that has a desk seat, **When** the operator
   selects that milestone or its desk row, **Then** the desk is the
   jump target, not an arbitrary last-focused window.
3. **Given** a ticket with more than one role seat, **When** the
   operator expands that ticket, **Then** each role is a distinct row.

---

### User Story 2 - Glance without disturbing the seat (Priority: P1)

The operator selects a seat and sees what is on that glass right now,
including colour, without shrinking or otherwise resizing the live
agent.

**Why this priority**: Browse must be an observer. Resizing a live
agent from a mosaic or preview is forbidden by the constitution.

**Independent Test**: Select a colourful agent seat. The preview shows
its current text and colours. The seat's own size does not change.

**Acceptance Scenarios**:

1. **Given** a selected seat, **When** the operator waits about a
   second, **Then** the preview refreshes from the live glass.
2. **Given** a seat whose process uses colours, **When** the operator
   looks at the preview, **Then** those colours are visible and the
   preview background matches the host terminal.
3. **Given** a preview is showing, **When** another client is attached
   to that seat at full size, **Then** that client is not forced down
   to the preview's cell size.

---

### User Story 3 - Enter the seat they meant (Priority: P1)

The operator jumps from the index into the selected seat with keyboard
or a double-click, and lands on that seat — not “some window in that
session.”

**Why this priority**: The old chooser fails exactly here.

**Independent Test**: Select a desk, press Enter. The attached client
is now on that desk. Repeat with double-click.

**Acceptance Scenarios**:

1. **Given** a selected seat, **When** the operator presses Enter,
   **Then** the attached client shows that seat.
2. **Given** a selected seat, **When** the operator double-clicks the
   tree row, the preview, or a pane box, **Then** the same jump occurs.
3. **Given** a light terminal background, **When** the operator reads
   the tree, **Then** labels remain readable (no white-on-white).

---

### User Story 4 - Record and gate the product (Priority: P2)

A stranger who clones the repository can read what a seat, a view, and
an observer are, run the test suite, and build a store-rooted binary.

**Why this priority**: The record outranks the implementation; an
unrooted binary is not a release.

**Independent Test**: Read `docs/m1/` and `.specify/memory/constitution.md`.
Run `nix develop -c cargo test` and `nix build .#cli`.

**Acceptance Scenarios**:

1. **Given** a clean checkout of this branch, **When** a reader opens
   the M1 pages, **Then** they can state seat, view, and observer.
2. **Given** the same checkout, **When** they run the unit tests,
   **Then** the window-name parser and colour tests pass.
3. **Given** the same checkout, **When** they `nix build .#cli`,
   **Then** they get a store-rooted `factory-tui` binary.

### Edge Cases

- A window whose name does not match the factory grammar stays visible
  under an unscoped / unknown grouping; it is not dropped.
- A seat with several panes: the operator can Tab or click a box to
  preview another pane; jump uses the selected pane.
- Wheel input arrives in bursts; one physical notch moves one row or
  one preview line.
- The index is opened as a popup that exits after jump. A standing
  `--serve` embed is out of scope for this ticket.

## Requirements

### Functional Requirements

- **FR-001**: The operator MUST be able to open an index that groups
  live seats by factory structure, not by multiplexer session list.
- **FR-002**: Selecting a seat MUST show an observer preview of that
  seat's current glass, including colour.
- **FR-003**: The preview MUST NOT change the live seat's terminal size.
- **FR-004**: Enter and double-click MUST move the attached client to
  the selected seat.
- **FR-005**: Tree labels MUST remain readable on a light host
  background.
- **FR-006**: The repository MUST carry a constitution and M1 pages
  that define seat, view, and observer.
- **FR-007**: The repository MUST provide a store-rooted build of the
  index (`nix build .#cli`) and a test command a clone can run.

### Key Entities

- **Seat**: one visible agent process (today: one multiplexer window).
- **View**: a named camera over a set of seats; does not own processes.
- **Observer**: a display that may snapshot or crop and must not resize.
- **Lane**: the set of role seats for one ticket.

## Success Criteria

### Measurable Outcomes

- **SC-001**: An operator finds a named milestone desk from the index
  without opening a session list first.
- **SC-002**: Previewing a seat leaves other attached clients at their
  existing size.
- **SC-003**: Enter and double-click produce the same jump.
- **SC-004**: A clean clone can run the documented test and Nix build
  commands without a pre-existing `target/release`.

## Assumptions

- The host already runs tmux and has live factory seats with the usual
  window-name grammar.
- A later milestone owns the standing `--serve` embed and a seat server.
- Views as live mosaics are specified in M1 docs but not shipped here.

## Non-goals

- A seat server with a retained cell buffer.
- Live four-window attach at view size.
- A second factory UI inside tmux-ws.
- Musl / AppImage / Homebrew release artifacts.
