# Data Model: Flat land browse camera

## Seat (Win)

- **Module**: `src/tmux.rs`
- **Fields**: session, window id, index, name, pixel/cell size, panes
- **Validation**: window id is the jump target; missing name → unknown
- **Invariant**: a seat is one window; panes are glass regions, not seats

## Pane

- **Module**: `src/tmux.rs`
- **Fields**: id, index, geometry, active, command, path
- **Relationship**: many per seat; preview selects one

## Parsed

- **Module**: `src/parse.rs`
- **Fields**: role, project, milestone, epic, ticket, goal, parked
- **Validation**: best-effort; unknown names still produce a Parsed
- **Invariant**: session hint may fill missing project/milestone

## Node

- **Module**: `src/tree.rs`
- **Fields**: id, kind, title, status, optional Parsed, optional Win,
  children
- **Relationship**: tree of factory kinds; a node may bind at most one
  seat window
- **Invariant**: jump uses `win`, never invents a session as identity

## Status

- **Module**: `src/tree.rs`
- **Values**: Running, Parked, Idle, Unknown
- **Rule**: parked name wins; agent command → Running; else idle/unknown
- **Invariant**: preview refresh must not write this from pane size
