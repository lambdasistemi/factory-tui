# Flat terminal land

!!! warning "Unvalidated experiment"
    This page is research, not operating advice. Today's binary is a
    browser over ordinary tmux windows. See [Using](../using.md).

**Milestone 1 outcome:** the factory is a tree of **seats**. A seat is
one visible agent process. On today's host a seat is one tmux
**window**. Sessions and panes are not rungs of that tree.

## The problem the chooser created

`prefix + s` lists session names. A name such as `keri` does not say
which project, which milestone, who the desk is, or whether anyone is
blocked. Attach drops the client on the last focused window — usually a
ticket, not the desk. The operator then clicks through encoded window
titles to find the milestone owner.

That is not a messy session. It is the wrong tree. The factory already
knows the desk; the multiplexer chooser cannot say it.

## Two axes

Factory authority (WHAT) and placement (WHERE) are independent.

```text
WHAT                         WHERE (today's host)
operator                     machine
  project                      session          (optional bucket)
    milestone                    window = seat
      epic                         pane         (not factory structure)
        ticket
          role (owner, implementer, auditor)
```

A session named after a milestone does not make it a milestone. A
2×2 split does not make four roles a single authority.

## Sessions are not required

Sessions today do three glued jobs:

1. Operator chooser — replaced by this app.
2. Factory grouping — already in files and window names.
3. Attach target for a tablet camera ([tmux-ws](https://github.com/lambdasistemi/tmux-ws))
   — that product is session-first (`tmux attach -t <session>`).

The factory does not need (1) or (2). Keep at most a few infra
sessions (`0-machine`, a home session for this browser, maybe
`0-projects`). Product seats may all live in one session. The index
must not break if they do.

Do not flatten every product window into one session until the tablet
camera has the same tree. Otherwise the tablet inherits a giant
window list — the old chooser, on glass.

## Panes are not required

Distinct visible seats are required: the ticket director, the
implementer, and a fresh auditor must be other processes a person can
open, each with their own start acknowledgement.

Those seats do not have to be panes of one window. A window per role
is more honest:

```text
project / milestone / epic / ticket
  ● ticket-owner     window
  ● commit-owner     window
  ● auditor          window   (only while the audit exists)
```

A lane is that set, not “one window with a layout.”

Panes fight the rest of the stack:

- A quadrant puts parent and grandchild on one piece of glass.
- A tablet xterm showing four live TUIs is unreadable.
- Same-window checks, detached splits, join-pane, layout drift exist
  because seats share a window.

Keep panes as optional human chrome: a disposable draft beside its
owner for five minutes; a personal editor and shell. Do not store
factory structure as splits.

## What the land looks like

The host is a flat field of windows. This app is the only layout. A
name on a window only has to be unique; the tree carries project,
milestone, epic, and role.

tmux remains the process babysitter: persist, reattach, keep the
agent alive when the glass goes away. It is not the org chart.
