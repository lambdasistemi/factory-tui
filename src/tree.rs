//! Project a factory tree from a live window census.

use std::collections::BTreeMap;

use crate::parse::{is_infra_session, parse_window, Parsed, Role};
use crate::tmux::Win;

/// Coarse liveness for a node, rolled up from children.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    Running,
    Parked,
    Idle,
    Unknown,
}

/// What the tree row represents.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kind {
    Machine,
    Infra,
    Project,
    Milestone,
    Epic,
    Desk,
    Ticket,
    Window,
    Group,
}

/// One node in the factory tree. Interior nodes may also have a `win` (the
/// desk) so Enter lands on the owner, not a random child.
#[derive(Clone, Debug)]
pub struct Node {
    pub id: String,
    pub kind: Kind,
    pub title: String,
    pub status: Status,
    pub parsed: Option<Parsed>,
    pub win: Option<Win>,
    pub children: Vec<Node>,
}

impl Node {
    /// Windows hanging off this subtree.
    pub fn window_count(&self) -> usize {
        let here = usize::from(self.win.is_some());
        here + self.children.iter().map(Node::window_count).sum::<usize>()
    }
}

type Rows = Vec<(Parsed, Win)>;

/// Build the operator-facing tree.
pub fn build(wins: Vec<Win>) -> Node {
    let mut rows: Rows = wins
        .into_iter()
        .map(|win| {
            let parsed = parse_window(&win.name, &win.session);
            (parsed, win)
        })
        .collect();
    inherit_session_milestones(&mut rows);

    let mut machine_wins: Vec<Win> = Vec::new();
    let mut infra: BTreeMap<String, Rows> = BTreeMap::new();
    let mut projects: BTreeMap<String, BTreeMap<String, Rows>> = BTreeMap::new();
    let mut loose: BTreeMap<String, Rows> = BTreeMap::new();

    for (parsed, win) in rows {
        if win.session == "0-machine" || matches!(parsed.role, Role::Machine | Role::Crew) {
            machine_wins.push(win);
            continue;
        }
        if is_infra_session(&win.session) {
            infra.entry(win.session.clone()).or_default().push((parsed, win));
            continue;
        }
        if let Some(project) = parsed.project.clone() {
            let ms = parsed.milestone.clone().unwrap_or_else(|| "?".into());
            projects.entry(project).or_default().entry(ms).or_default().push((parsed, win));
        } else {
            loose.entry(win.session.clone()).or_default().push((parsed, win));
        }
    }

    let mut children = Vec::new();
    children.push(machine_group(machine_wins));

    if !infra.is_empty() || !loose.is_empty() {
        let mut infra_node = empty("infra", Kind::Infra, "infra");
        for (session, rows) in infra {
            infra_node.children.push(session_group("infra", &session, rows));
        }
        for (session, rows) in loose {
            infra_node.children.push(session_group("unscoped", &session, rows));
        }
        infra_node.status = rollup(&infra_node.children);
        children.push(infra_node);
    }

    for (project, milestones) in projects {
        let mut pnode = empty(&format!("project/{project}"), Kind::Project, &project);
        for (ms, rows) in milestones {
            pnode.children.push(milestone_node(&project, &ms, rows));
        }
        pnode.status = rollup(&pnode.children);
        children.push(pnode);
    }

    let mut root = empty("root", Kind::Group, "factory");
    root.children = children;
    root.status = rollup(&root.children);
    root
}

fn inherit_session_milestones(rows: &mut Rows) {
    let mut session_ms: BTreeMap<String, String> = BTreeMap::new();
    for (parsed, win) in rows.iter() {
        if parsed.role == Role::Desk {
            if let Some(ms) = &parsed.milestone {
                session_ms.insert(win.session.clone(), ms.clone());
            }
        }
    }
    for (parsed, win) in rows.iter_mut() {
        if parsed.milestone.is_none() {
            if let Some(ms) = session_ms.get(&win.session) {
                parsed.milestone = Some(ms.clone());
            }
        }
    }
}

fn machine_group(wins: Vec<Win>) -> Node {
    let kids: Vec<Node> = wins
        .into_iter()
        .map(|w| {
            let parsed = parse_window(&w.name, &w.session);
            let title = w.name.clone();
            window_node(&format!("machine/{title}"), kind_for(&parsed), &title, parsed, w)
        })
        .collect();
    let mut machine = group("machine", Kind::Machine, "machine", kids);
    if let Some(desk) = machine.children.iter().find(|c| c.title == "machine") {
        machine.win = desk.win.clone();
    }
    machine
}

fn milestone_node(project: &str, ms: &str, rows: Rows) -> Node {
    let id = format!("project/{project}/ms/{ms}");
    let title = if ms == "?" { "no milestone".to_string() } else { format!("M{ms}") };

    let mut desk: Option<(Parsed, Win)> = None;
    let mut by_epic: BTreeMap<String, Rows> = BTreeMap::new();
    let mut standalone: Rows = Vec::new();

    for (parsed, win) in rows {
        if parsed.role == Role::Desk && desk.is_none() {
            desk = Some((parsed, win));
        } else if let Some(epic) = parsed.epic.clone() {
            by_epic.entry(epic).or_default().push((parsed, win));
        } else {
            standalone.push((parsed, win));
        }
    }

    let mut node = empty(&id, Kind::Milestone, &title);
    if let Some((parsed, win)) = desk {
        node.title = format!("M{ms} {}", parsed.goal);
        node.parsed = Some(parsed.clone());
        node.win = Some(win.clone());
        node.status = status_of(&parsed, &win);
        node.children.push(window_node(&format!("{id}/desk"), Kind::Desk, "desk", parsed, win));
    }

    for (epic, erows) in by_epic {
        node.children.push(epic_node(&id, &epic, erows));
    }
    for (parsed, win) in standalone {
        let tid = parsed.ticket.clone().unwrap_or_else(|| win.name.clone());
        let title = ticket_title(&parsed, &win);
        node.children.push(window_node(
            &format!("{id}/t/{tid}"),
            kind_for(&parsed),
            &title,
            parsed,
            win,
        ));
    }
    if node.status == Status::Unknown {
        node.status = rollup(&node.children);
    } else {
        let child = rollup(&node.children);
        node.status = worse(node.status, child);
    }
    node
}

fn epic_node(parent: &str, epic: &str, rows: Rows) -> Node {
    let id = format!("{parent}/e/{epic}");
    let mut node = empty(&id, Kind::Epic, &ref_label("e", epic));
    for (parsed, win) in rows {
        if parsed.role == Role::Epic && parsed.ticket.is_none() && node.win.is_none() {
            node.title = format!("{} {}", ref_label("e", epic), parsed.goal);
            node.parsed = Some(parsed.clone());
            node.win = Some(win.clone());
            node.status = status_of(&parsed, &win);
            continue;
        }
        let tid = parsed.ticket.clone().unwrap_or_else(|| win.name.clone());
        let title = ticket_title(&parsed, &win);
        node.children.push(window_node(
            &format!("{id}/t/{tid}"),
            kind_for(&parsed),
            &title,
            parsed,
            win,
        ));
    }
    if node.win.is_none() {
        if let Some(first) = node.children.first() {
            node.win.clone_from(&first.win);
        }
    }
    if node.status == Status::Unknown {
        node.status = rollup(&node.children);
    } else {
        node.status = worse(node.status, rollup(&node.children));
    }
    node
}

fn session_group(prefix: &str, session: &str, rows: Rows) -> Node {
    let mut node = empty(&format!("{prefix}/{session}"), Kind::Group, session);
    for (parsed, win) in rows {
        let title = win.name.clone();
        let id = format!("{prefix}/{session}/{}", win.id);
        node.children.push(window_node(&id, kind_for(&parsed), &title, parsed, win));
    }
    node.status = rollup(&node.children);
    if let Some(first) = node.children.first() {
        node.win.clone_from(&first.win);
    }
    node
}

fn window_node(id: &str, kind: Kind, title: &str, parsed: Parsed, win: Win) -> Node {
    Node {
        id: id.to_string(),
        kind,
        title: title.to_string(),
        status: status_of(&parsed, &win),
        parsed: Some(parsed),
        win: Some(win),
        children: Vec::new(),
    }
}

fn ticket_title(parsed: &Parsed, win: &Win) -> String {
    if let Some(t) = &parsed.ticket {
        let label = ref_label("t", t);
        if parsed.goal == *t {
            label
        } else {
            format!("{label} {}", parsed.goal)
        }
    } else {
        win.name.clone()
    }
}

fn ref_label(prefix: &str, id: &str) -> String {
    if id.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        format!("{prefix}{id}")
    } else {
        format!("{prefix}-{id}")
    }
}

fn kind_for(parsed: &Parsed) -> Kind {
    match parsed.role {
        Role::Machine => Kind::Machine,
        Role::Crew | Role::Orch | Role::Unknown => Kind::Window,
        Role::Desk => Kind::Desk,
        Role::Epic => Kind::Epic,
        Role::Ticket => Kind::Ticket,
    }
}

fn empty(id: &str, kind: Kind, title: &str) -> Node {
    Node {
        id: id.to_string(),
        kind,
        title: title.to_string(),
        status: Status::Unknown,
        parsed: None,
        win: None,
        children: Vec::new(),
    }
}

fn group(id: &str, kind: Kind, title: &str, children: Vec<Node>) -> Node {
    let mut n = empty(id, kind, title);
    n.status = rollup(&children);
    n.children = children;
    n
}

fn status_of(parsed: &Parsed, win: &Win) -> Status {
    if parsed.parked {
        return Status::Parked;
    }
    if win.panes.iter().any(|p| is_agent(&p.cmd)) {
        Status::Running
    } else if win.panes.iter().any(|p| is_shell(&p.cmd)) {
        Status::Idle
    } else {
        Status::Unknown
    }
}

fn is_agent(cmd: &str) -> bool {
    matches!(
        cmd,
        "claude" | "codex" | "codex-raw" | "agy" | "qwen" | "grok" | "kimi" | "gemini" | "node"
    )
}

fn is_shell(cmd: &str) -> bool {
    matches!(cmd, "bash" | "zsh" | "fish" | "sh" | "tmux")
}

fn rollup(kids: &[Node]) -> Status {
    kids.iter().fold(Status::Unknown, |acc, k| worse(acc, k.status))
}

fn worse(a: Status, b: Status) -> Status {
    use Status::{Idle, Parked, Running, Unknown};
    fn rank(s: Status) -> u8 {
        match s {
            Parked => 3,
            Running => 2,
            Idle => 1,
            Unknown => 0,
        }
    }
    if rank(b) > rank(a) {
        b
    } else {
        a
    }
}

/// Print a plain-text tree for `--dump` and tests.
pub fn dump(node: &Node) -> String {
    let mut out = String::new();
    dump_into(node, 0, &mut out);
    out
}

fn dump_into(node: &Node, depth: usize, out: &mut String) {
    if node.id != "root" {
        let pad = "  ".repeat(depth);
        let seat =
            node.win.as_ref().map(|w| format!("  [{}:{}]", w.session, w.name)).unwrap_or_default();
        out.push_str(&format!(
            "{pad}{} {}  {}{seat}\n",
            glyph(node),
            node.title,
            status_label(node.status),
        ));
    }
    let next = if node.id == "root" { depth } else { depth + 1 };
    for c in &node.children {
        dump_into(c, next, out);
    }
}

fn glyph(node: &Node) -> &'static str {
    match node.kind {
        Kind::Machine => "⚙",
        Kind::Project => "▶",
        Kind::Milestone => "◆",
        Kind::Desk => "●",
        Kind::Epic => "▸",
        Kind::Ticket => "·",
        _ => "•",
    }
}

/// Short status word for the tree column.
pub fn status_label(s: Status) -> &'static str {
    match s {
        Status::Running => "RUNNING",
        Status::Parked => "PARKED",
        Status::Idle => "idle",
        Status::Unknown => "",
    }
}
