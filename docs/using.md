# Using factory-tui

factory-tui is only as clear as your tmux names. The layout below is
what the current binary understands. It is not a claim that this is
the final factory.

## Bind and keys

Install the binary, then in `~/.tmux.conf`:

```tmux
bind-key -n F1 display-popup -E -w 90% -h 90% factory-tui
bind-key S     display-popup -E -w 90% -h 90% factory-tui
```

Run it from inside tmux (`$TMUX` must be set).

| Key / gesture | Action |
|---|---|
| `j` `k` / arrows | move |
| `h` `l` / `Space` | collapse / expand |
| `Enter` / double-click | jump to that window and quit |
| `r` | refresh the census |
| mouse wheel | scroll the tree or the snapshot |
| `q` / `Esc` | quit without jumping |

`--dump` is the same census as a text tree.

## What to put where

Use **sessions** as bags of related work. Use **windows** as the
seats you want to find. Name the windows; do not rely on
`prefix + s`.

A host that works well with the browser looks like this:

```text
0-machine          window  machine
0-projects         one window per product, named for that product
<product>          window  orch                 (optional session desk)
                   window  <repo>-ms<N>-<goal>  (milestone desk)
                   window  <repo>-e<E>-t<T>-<goal>
                   window  <repo>-no-epic-t<T>-<goal>
```

- `0-machine` and `0-projects` (and any other `0-*` session, plus
  `warden`) are **infrastructure**. They show under *machine* /
  *infra*, not as product folders.
- Product work belongs in a **non-`0-` session**. That is what
  becomes a project in the tree.
- Window `orch` is recognised and left as a session desk. It is not
  a project node.
- Panes inside a window (splits, quadrants) are invisible to the
  tree. Split freely; still name the **window**.

If a product session is named `acme` and a window is only
`ms1-ship`, the project is taken from the session. If the window is
`acme-ms1-ship`, the prefix on the window wins.

A session name that itself contains `-ms<N>` (for example
`acme-ms2`) fills in milestone `N` when the window name does not.

## Window names the parser understands

Unknown names are kept, under *unscoped*. Nothing is dropped.

| Kind | Pattern | Example |
|---|---|---|
| host desk | `machine` | `machine` |
| host helpers | `machine-crew`, `machine-bootstrap`, `machine-orphans` | `machine-crew` |
| session desk | `orch` | `orch` |
| milestone desk | `[<repo>-]ms<N>-<goal>` | `acme-ms1-ship` |
| ticket on a milestone | `[<repo>-]ms<N>-t<id>-<goal>` | `acme-ms1-t12-docs` |
| ticket, slug id | `[<repo>-]ms<N>-t-<slug>` | `acme-ms1-t-release-notes` |
| epic | `[<repo>-]e<id>-<goal>` | `acme-e4-parser` |
| ticket on an epic | `[<repo>-]e<id>-t<id>-<goal>` | `acme-e4-t18-rename` |
| named epic | `[<repo>-]e-<slug>` | `acme-e-rfq` |
| ticket on a named epic | `[<repo>-]e-<slug>-t-<goal>` | `acme-e-rfq-t-survey` |
| ticket, no epic | `[<repo>-]no-epic-t<id>-<goal>` | `acme-no-epic-t9-hotfix` |

`PARKED` anywhere in the name (any case) marks the seat parked.

The exact grammar lives in `src/parse.rs`. A handful of session-name
aliases for known repos are there too.

## What the tree then shows

```text
⚙ machine
• infra                         # 0-* sessions, warden, …
▶ acme
  ◆ M1 ship                     # window acme-ms1-ship
    ● desk
    ▸ e4 parser                 # window acme-e4-parser
      · t18 rename              # window acme-e4-t18-rename
    · t12 docs                  # window acme-ms1-t12-docs
```

- **RUNNING** — a pane in that window is running a known agent
  binary (`claude`, `codex`, `codex-raw`, `agy`, `qwen`, `grok`,
  `kimi`, `gemini`, `node`).
- **PARKED** — the window name contains `PARKED`.
- **idle** — only a shell (or tmux itself) is in the window.

Parent rows roll those up. Enter on a folder that has a desk window
lands on the desk, not on a random child.

## Habits that make the browser useful

1. Rename the window as soon as the lane has an id and a goal.
   Generic names (`zsh`, `codex`, `window`) fall into *unscoped*.
2. Put the milestone desk in the same session as its tickets, and
   give it an `ms<N>` name. That row becomes the folder title.
3. Keep one window per lane. Extra panes are fine; extra unnamed
   windows are noise.
4. Park by renaming (`…-PARKED`), not by hoping the process list
   looks quiet.
5. Do not expect `0-projects` windows to appear under the product
   folder. They are infra. The product folder is filled by windows
   in the product session.

## Check

```
factory-tui --dump
```

If a lane you care about is under *unscoped* or *infra*, the session
or the window name is what to fix, not the UI.
