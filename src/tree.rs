//! Build the raw session → window → pane tree from a live tmux census.

use crate::config::{Config, Sampler, SamplerStatus};
use crate::label::{node_label, pane_label};
use crate::tmux::{Pane, Win};

/// Coarse liveness for a node, rolled up from children.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    Running,
    Parked,
    Idle,
    Unknown,
}

impl Status {
    /// Every status a row can carry. Whatever reserves the status vocabulary
    /// derives it from here, so a status added later cannot arrive unreserved.
    pub const ALL: [Status; 4] = [Status::Running, Status::Parked, Status::Idle, Status::Unknown];
}

/// What the tree row represents.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kind {
    SessionGroup,
    Window,
    Pane,
}

/// One node in the tmux tree.
///
/// `title` is the node's *raw* name: the tmux session name, window name, or
/// pane identity. It is display source, never identity — `id`, `kind`, `win`
/// and `pane` are what the tree is made of, and a display label is composed
/// from `title` only once the tree already exists.
#[derive(Clone, Debug)]
pub struct Node {
    pub id: String,
    pub kind: Kind,
    pub title: String,
    pub status: Status,
    pub win: Option<Win>,
    /// The exact tmux pane this row jumps to, when it targets one.
    pub pane: Option<String>,
    pub children: Vec<Node>,
}

impl Node {
    /// Window rows hanging off this subtree.
    ///
    /// Pane rows are not windows. Counting them here would inflate the badge a
    /// collapsed row carries, and would quietly report a three-pane seat as
    /// three seats.
    pub fn window_count(&self) -> usize {
        let here = usize::from(self.kind == Kind::Window);
        here + self.children.iter().map(Node::window_count).sum::<usize>()
    }
}

/// Build the tree tmux actually has: session → window → pane.
///
/// No configuration reaches this function beyond the status vocabulary, which
/// is why no naming rule can hide, move, merge or invent a seat. The only
/// judgement made here is the single-pane rule below.
pub fn build(wins: Vec<Win>, samplers: &[Sampler]) -> Node {
    // Internal sampling keeps its `&Config` seam, so the deliberately narrow
    // public input is adapted here. This adapter cannot smuggle a label rule
    // in: it is `Config::empty()` with the sampler table put back, and nothing
    // else.
    let config = Config { sampler: samplers.to_vec(), ..Config::empty() };

    let mut sessions: Vec<(String, Vec<Win>)> = Vec::new();
    for win in wins {
        match sessions.iter_mut().find(|(name, _)| *name == win.session) {
            Some((_, windows)) => windows.push(win),
            None => sessions.push((win.session.clone(), vec![win])),
        }
    }

    let children: Vec<Node> =
        sessions.into_iter().map(|(name, windows)| session_node(name, windows, &config)).collect();
    let status = rollup(&children);

    Node {
        id: "root".to_string(),
        kind: Kind::SessionGroup,
        title: "tmux".to_string(),
        status,
        win: None,
        pane: None,
        children,
    }
}

fn session_node(name: String, windows: Vec<Win>, config: &Config) -> Node {
    let id = format!("session/{name}");
    let children: Vec<Node> =
        windows.into_iter().map(|win| window_node(&id, win, config)).collect();
    let status = rollup(&children);

    Node { id, kind: Kind::SessionGroup, title: name, status, win: None, pane: None, children }
}

fn window_node(parent: &str, win: Win, config: &Config) -> Node {
    let id = format!("{parent}/{}", win.id);
    let status = status_of(&win, config);

    // A window holding one pane *is* that pane. A child row would repeat what
    // the window row already says and would turn every solo seat into two
    // rows; a window holding several needs one row each, because that is the
    // only place their separate identities and jump targets can live.
    let children: Vec<Node> = if win.panes.len() > 1 {
        win.panes.iter().map(|pane| pane_node(&id, pane, &win, config)).collect()
    } else {
        Vec::new()
    };

    let pane = win
        .panes
        .iter()
        .find(|pane| pane.active)
        .or_else(|| win.panes.first())
        .map(|pane| pane.id.clone());

    Node { id, kind: Kind::Window, title: win.name.clone(), status, win: Some(win), pane, children }
}

fn pane_node(parent: &str, pane: &Pane, win: &Win, config: &Config) -> Node {
    Node {
        id: format!("{parent}/{}", pane.id),
        kind: Kind::Pane,
        // The pane's own safe identity, at no width bound: a tree row has no
        // cell budget, and what a pane may say is owned once, in the label
        // module.
        title: pane_label(pane, u16::MAX),
        status: status_of_pane(pane, win, config),
        // The window is carried so that selecting a pane row previews, peeks
        // and jumps through exactly the same path a window row uses.
        win: Some(win.clone()),
        pane: Some(pane.id.clone()),
        children: Vec::new(),
    }
}

fn status_of(win: &Win, config: &Config) -> Status {
    win.panes.iter().map(|pane| status_of_pane(pane, win, config)).fold(Status::Unknown, worse)
}

fn status_of_pane(pane: &Pane, win: &Win, config: &Config) -> Status {
    config
        .sampler
        .iter()
        .find(|sampler| sampler.matches(win, pane))
        .and_then(Sampler::outcome)
        .map_or(Status::Unknown, |status| match status {
            SamplerStatus::Running => Status::Running,
            SamplerStatus::Idle => Status::Idle,
            SamplerStatus::Parked => Status::Parked,
        })
}

fn rollup(children: &[Node]) -> Status {
    children.iter().fold(Status::Unknown, |status, child| worse(status, child.status))
}

fn worse(left: Status, right: Status) -> Status {
    fn rank(status: Status) -> u8 {
        match status {
            Status::Parked => 3,
            Status::Running => 2,
            Status::Idle => 1,
            Status::Unknown => 0,
        }
    }
    if rank(right) > rank(left) {
        right
    } else {
        left
    }
}

/// The exact tmux pane a row jumps to, when the row names one on its own.
///
/// A `Pane` row is its own pane. A window with a single pane is that pane, so
/// selecting it needs no fallback. A window with several panes returns `None`
/// and leaves the existing preview/cycle behaviour untouched.
pub fn selected_pane_id(node: &Node) -> Option<&str> {
    match node.kind {
        Kind::Pane => node.pane.as_deref(),
        Kind::Window => match node.win.as_ref() {
            Some(win) if win.panes.len() == 1 => node.pane.as_deref(),
            _ => None,
        },
        _ => None,
    }
}

/// One node as the proof sees it: everything the tree *is*, and nothing it
/// merely displays.
#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructuralEntry {
    /// Stable node identity.
    pub id: String,
    /// Identity of the node that holds it, or `None` at the root.
    pub parent: Option<String>,
    /// What the row represents.
    pub kind: Kind,
    /// The window this row jumps to, if any.
    pub window_target: Option<String>,
    /// The pane this row jumps to, if any.
    pub pane_target: Option<String>,
}

/// The tree's structure in row order, with every display string excluded.
///
/// This is the comparator INV-26-C8 is stated against: if reinterpretation can
/// insert, drop, merge, split, reorder, reparent or retarget a row, this
/// sequence changes. It carries no title, so a pure rename cannot move it —
/// which is what makes the equality it proves worth anything.
#[cfg(test)]
pub fn structural_fingerprint(root: &Node) -> Vec<StructuralEntry> {
    fn walk(node: &Node, parent: Option<&str>, out: &mut Vec<StructuralEntry>) {
        out.push(StructuralEntry {
            id: node.id.clone(),
            parent: parent.map(ToString::to_string),
            kind: node.kind,
            window_target: node.win.as_ref().map(|win| win.id.clone()),
            pane_target: node.pane.clone(),
        });
        for child in &node.children {
            walk(child, Some(&node.id), out);
        }
    }
    let mut out = Vec::new();
    walk(root, None, &mut out);
    out
}

/// Print a plain-text tree for `--dump` and tests.
///
/// Labels are composed here, after the tree exists. Each window row carries
/// one `[window=…]` marker and each pane exactly one `[pane=…]` marker, on its
/// single-pane window or on its own row, so that reachability can be checked
/// against a census taken straight from tmux instead of against the tree's own
/// opinion of itself.
pub fn dump(node: &Node, config: &Config) -> String {
    let mut output = String::new();
    dump_into(node, 0, config, &mut output);
    output
}

fn dump_into(node: &Node, depth: usize, config: &Config, output: &mut String) {
    if node.id != "root" {
        output.push_str(&"  ".repeat(depth));
        output.push_str("• ");
        output.push_str(&node_label(node, config));
        let status = status_field(node.status);
        if !status.is_empty() {
            output.push_str("  ");
            output.push_str(&status);
        }
        if node.kind == Kind::Window {
            if let Some(win) = &node.win {
                output.push_str(&format!("  [window={}]", win.id));
            }
        }
        if let Some(pane) = selected_pane_id(node) {
            output.push_str(&format!("  [pane={pane}]"));
        }
        output.push('\n');
    }

    let child_depth = if node.id == "root" { depth } else { depth + 1 };
    for child in &node.children {
        dump_into(child, child_depth, config, output);
    }
}

/// Short status word for the tree column.
pub fn status_label(status: Status) -> &'static str {
    match status {
        Status::Running => "RUNNING",
        Status::Parked => "PARKED",
        Status::Idle => "idle",
        Status::Unknown => "",
    }
}

/// The status column as rendered: bracketed, or empty when the row makes no
/// claim. A row's liveness is the renderer's to state, so it is stated in a
/// form a label cannot produce — [`crate::label::defang_reserved`] neutralises
/// this exact shape in any display string.
pub fn status_field(status: Status) -> String {
    match status_label(status) {
        "" => String::new(),
        word => format!("[{word}]"),
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::ffi::OsString;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::Mutex;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{build, dump, selected_pane_id, structural_fingerprint, Kind, Node, Status};
    use crate::config::{self, Config, Scope};
    use crate::label::{node_label, reinterpret};
    use crate::tmux::{Pane, Win};

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    /// The tree under proof.
    ///
    /// `build` loses its configuration parameter in this slice (FN-26-BUILD):
    /// raw topology is built from the tmux census and the unchanged status
    /// table alone, and no reinterpreter reaches it. Every proof goes through
    /// this one adapter so that narrowing is a single edit rather than a
    /// rewrite of the assertions it must not disturb.
    fn built(wins: Vec<Win>, config: &Config) -> Node {
        build(wins, &config.sampler)
    }

    /// The six windows of the reproduced `factory-tui` session, in the order
    /// tmux reported them. The three `no-epic-t…` windows match none of the
    /// shipped projection rules, which is exactly why they were folded out of
    /// the projected project and into a second, identically titled sibling.
    const D14_D15_WINDOWS: [&str; 6] = [
        "factory-tui-e8-t5-raw-tree",
        "factory-tui-ms3-no-bugs",
        "factory-tui-e15-t16-status-samplers",
        "factory-tui-no-epic-t19-m3-preview-tag",
        "factory-tui-no-epic-t22-pane-titles",
        "factory-tui-no-epic-t24-build-provenance",
    ];

    /// The configuration this runs against.
    ///
    /// The reproduction ran on the shipped projection example, which folded
    /// three of these six windows into a project row and left the other three
    /// under an identically titled session row. That schema no longer exists,
    /// so the strongest remaining configuration stands in its place: rules
    /// that relabel every session and every window they can see. The point of
    /// the proof is that this is now the *most* a config file can do.
    fn d14_d15_config() -> Config {
        config::load_from_str(
            r#"
[[reinterpreter]]
scope = "session"
pattern = "^(?P<name>.+)$"
label = "project $name"

[[reinterpreter]]
scope = "window"
pattern = "^(?P<project>.+)-(?:no-epic|e[0-9]+|ms[0-9]+)-(?P<rest>.+)$"
label = "$rest"
"#,
        )
        .expect("the reproduction configuration parses")
    }

    /// Every title that appears more than once among one parent's children,
    /// anywhere in the tree. Two sibling rows with the same name are
    /// indistinguishable to a reader, whatever the ids behind them say.
    fn colliding_sibling_titles(node: &Node) -> Vec<String> {
        let mut found = Vec::new();
        for (index, child) in node.children.iter().enumerate() {
            if node.children[..index].iter().any(|earlier| earlier.title == child.title) {
                found.push(child.title.clone());
            }
            found.extend(colliding_sibling_titles(child));
        }
        found
    }

    #[test]
    fn d14_d15_six_windows_are_one_raw_session() {
        let config = d14_d15_config();
        let wins: Vec<Win> = D14_D15_WINDOWS
            .iter()
            .enumerate()
            .map(|(index, name)| {
                fake_win("factory-tui", &format!("@{}", 4455 + index), name, "worker")
            })
            .collect();

        let root = built(wins, &config);

        // D15: one tmux session produces one row. A configuration that can
        // also emit a project row of the same name makes authority unreadable.
        let sessions: Vec<&str> = root.children.iter().map(|node| node.title.as_str()).collect();
        assert_eq!(sessions, ["factory-tui"], "root children were {sessions:?}");
        let session = &root.children[0];
        assert_eq!(session.kind, Kind::SessionGroup);

        // D14: every window the census reported is present, exactly once, and
        // directly beneath that session.
        let windows: Vec<&str> = session.children.iter().map(|node| node.title.as_str()).collect();
        for name in D14_D15_WINDOWS {
            let seen = windows.iter().filter(|title| **title == name).count();
            assert_eq!(seen, 1, "window {name} appeared {seen} times in {windows:?}");
        }
        assert_eq!(windows.len(), D14_D15_WINDOWS.len(), "extra rows in {windows:?}");
        assert_eq!(root.window_count(), D14_D15_WINDOWS.len());

        // And no reader anywhere in the tree is asked to tell two identically
        // titled siblings apart — not by raw name, and not by what the rules
        // above rewrote those names into.
        assert_eq!(colliding_sibling_titles(&root), Vec::<String>::new());
        let shown: Vec<String> =
            session.children.iter().map(|node| node_label(node, &config)).collect();
        let mut distinct = shown.clone();
        distinct.sort();
        distinct.dedup();
        assert_eq!(distinct.len(), shown.len(), "two rows display the same text: {shown:?}");
        for (raw, label) in D14_D15_WINDOWS.iter().zip(&shown) {
            assert!(label.contains(raw), "{label:?} no longer reaches back to {raw:?}");
        }
    }

    #[test]
    fn c8_reinterpreters_only_change_display_strings() {
        let plain = Config::empty();
        for (state, wins) in generated_states() {
            for (rules, config) in reinterpreter_configs() {
                let raw = built(wins.clone(), &plain);
                let relabelled = built(wins.clone(), &config);

                let fingerprint = structural_fingerprint(&raw);
                assert_eq!(
                    fingerprint.len(),
                    node_count(&raw),
                    "{state}: the oracle skipped nodes"
                );
                assert!(fingerprint.len() > 2, "{state}: nothing structural to compare");

                // The configuration must actually reach the display. An
                // equality between two things that were never made to differ
                // would pass with reinterpretation deleted, and would say
                // nothing about whether it can move a node.
                let before = display_strings(&raw, &plain);
                let after = display_strings(&relabelled, &config);
                assert_ne!(
                    before, after,
                    "{rules} changed no display string on {state}; the equality below proves nothing"
                );

                // Structure is untouched by that change: same nodes, same
                // parents, same kinds, same jump targets, same order.
                assert_eq!(
                    structural_fingerprint(&relabelled),
                    fingerprint,
                    "{rules} changed the structure of {state}"
                );
                assert_eq!(after.len(), before.len(), "{rules} changed the row count of {state}");
            }
        }
    }

    #[test]
    fn c8_oracle_rejects_structural_split_mutant() {
        let (state, wins) = generated_states().swap_remove(1);
        let root = built(wins, &Config::empty());
        let baseline = structural_fingerprint(&root);
        assert!(baseline.len() > 4, "{state}: the oracle must be reading a real tree");
        assert_eq!(structural_fingerprint(&root.clone()), baseline, "two copies must agree");

        // The mutant this invariant is named for: one row becomes two.
        let mut split = root.clone();
        let mut twin = split.children[0].children[0].clone();
        twin.id = format!("{}-split", twin.id);
        split.children[0].children.insert(1, twin);
        assert_eq!(node_count(&split), node_count(&root) + 1, "the split mutant did not apply");
        assert_ne!(structural_fingerprint(&split), baseline, "the oracle accepted a split row");

        // A row that disappears.
        let mut dropped = root.clone();
        dropped.children[0].children.remove(0);
        assert!(node_count(&dropped) < node_count(&root), "the drop mutant did not apply");
        assert_ne!(structural_fingerprint(&dropped), baseline, "the oracle accepted a dropped row");

        // A row that keeps its name and changes parent.
        let mut moved = root.clone();
        let taken = moved.children[1].children.remove(0);
        moved.children[0].children.push(taken);
        assert_eq!(node_count(&moved), node_count(&root), "the reparent mutant changed the count");
        assert_ne!(
            structural_fingerprint(&moved),
            baseline,
            "the oracle accepted a reparented row"
        );

        // Two rows that swap places.
        let mut reordered = root.clone();
        reordered.children[1].children.swap(0, 1);
        assert_ne!(
            reordered.children[1].children[0].id, root.children[1].children[0].id,
            "the reorder mutant did not apply"
        );
        assert_ne!(structural_fingerprint(&reordered), baseline, "the oracle accepted a reorder");

        // A row that keeps its identity and points somewhere else.
        let mut retargeted = root.clone();
        retargeted.children[1].children[0].pane = Some("%999".to_string());
        assert_ne!(
            retargeted.children[1].children[0].pane, root.children[1].children[0].pane,
            "the retarget mutant did not apply"
        );
        assert_ne!(structural_fingerprint(&retargeted), baseline, "the oracle accepted a retarget");

        // The control in the other direction, without which the comparator
        // would just be "anything changed" and INV-26-C8 would be unstatable:
        // renaming every row must be *accepted*.
        fn rename(node: &mut Node) {
            node.title = format!("renamed-{}", node.title);
            for child in &mut node.children {
                rename(child);
            }
        }
        let mut renamed = root.clone();
        rename(&mut renamed);
        assert_ne!(
            display_strings(&renamed, &Config::empty()),
            display_strings(&root, &Config::empty()),
            "the rename mutant did not apply"
        );
        assert_eq!(
            structural_fingerprint(&renamed),
            baseline,
            "the oracle is watching display strings, not structure"
        );
    }

    #[test]
    fn pane_nodes_follow_single_multi_rule() {
        let root = built(
            vec![
                fake_win_panes("work", "@1", "solo", vec![fake_pane("%10", "0", "bash", "")]),
                fake_win_panes(
                    "work",
                    "@2",
                    "split",
                    vec![
                        fake_pane("%20", "0", "worker-a", "owner-1"),
                        fake_pane("%21", "1", "worker-b", "auditor-1"),
                        fake_pane("%22", "2", "bash", ""),
                    ],
                ),
            ],
            &Config::empty(),
        );
        let session = &root.children[0];
        assert_eq!(session.children.len(), 2, "one row per window");

        let solo = &session.children[0];
        assert_eq!(solo.kind, Kind::Window);
        assert!(solo.children.is_empty(), "a one-pane window carries no redundant pane child");
        assert_eq!(selected_pane_id(solo), Some("%10"), "a one-pane window targets its own pane");

        let split = &session.children[1];
        assert_eq!(split.kind, Kind::Window);
        assert_eq!(split.children.len(), 3, "one row per pane of a multi-pane window");
        assert!(split.children.iter().all(|node| node.kind == Kind::Pane));
        assert_eq!(
            selected_pane_id(split),
            None,
            "a multi-pane window keeps the unchanged preview fallback"
        );
        let targets: Vec<Option<&str>> = split.children.iter().map(selected_pane_id).collect();
        assert_eq!(
            targets,
            [Some("%20"), Some("%21"), Some("%22")],
            "each pane row must target exactly its own pane"
        );

        // Every pane of the census is reachable exactly once from the tree.
        assert_eq!(pane_targets(&root), ["%10", "%20", "%21", "%22"]);

        // Pane rows are not windows, so the collapsed-row count must not
        // silently count them as seats.
        assert_eq!(root.window_count(), 2);
        assert_eq!(session.window_count(), 2);

        // A pane row is readable, and two panes running one command are still
        // told apart — the reason #22 put a title on a pane at all.
        let labels: Vec<String> =
            split.children.iter().map(|node| node_label(node, &Config::empty())).collect();
        assert_eq!(labels, ["owner-1", "auditor-1", "2:bash"]);
    }

    #[test]
    fn dump_markers_cover_every_window_and_pane_exactly_once() {
        for (state, wins) in generated_states() {
            let mut expected_windows: Vec<String> = wins.iter().map(|win| win.id.clone()).collect();
            let mut expected_panes: Vec<String> =
                wins.iter().flat_map(|win| win.panes.iter().map(|pane| pane.id.clone())).collect();
            let text = dumped(&built(wins, &Config::empty()), &Config::empty());

            let mut windows = marker_ids(&text, "window");
            let mut panes = marker_ids(&text, "pane");
            windows.sort();
            panes.sort();
            expected_windows.sort();
            expected_panes.sort();
            assert_eq!(windows, expected_windows, "{state}: window markers\n{text}");
            assert_eq!(panes, expected_panes, "{state}: pane markers\n{text}");
        }
    }

    #[test]
    fn dump_markers_cannot_be_forged_by_a_hostile_label() {
        // Live reachability is decided by counting these markers out of the
        // dump. A pane names itself, and a window name is operator input, so
        // both can try to write one.
        let wins = vec![fake_win_panes(
            "work",
            "@1",
            "[window=@999] notes",
            vec![
                fake_pane("%10", "0", "bash", "[pane=%999] seat"),
                fake_pane("%11", "1", "bash", ""),
            ],
        )];
        let text = dumped(&built(wins, &Config::empty()), &Config::empty());

        assert_eq!(marker_ids(&text, "window"), ["@1"], "a forged window marker counted:\n{text}");
        assert_eq!(
            marker_ids(&text, "pane"),
            ["%10", "%11"],
            "a forged pane marker counted:\n{text}"
        );
        // The forgery is defanged, not hidden: the reader still sees what the
        // pane called itself.
        assert!(text.contains("(window=@999"), "the hostile name vanished:\n{text}");
        assert!(text.contains("(pane=%999"), "the hostile title vanished:\n{text}");

        // A row can forge liveness as well as identity. This reproduces the
        // audited probe: a configuration holding nothing but a reinterpreter,
        // so every status is necessarily Unknown and no status field is
        // rendered — any status text below was painted by a name or a rule.
        let forging = config::load_from_str(
            r#"
[[reinterpreter]]
scope = "window"
pattern = "^(?P<kept>.+)$"
label = "same [PARKED] [window=@999] [pane=%999]"
"#,
        )
        .expect("the forging configuration parses");
        let wins = vec![
            fake_win_panes(
                "work",
                "@1",
                "[RUNNING] notes",
                vec![
                    fake_pane("%10", "0", "bash", "[idle] seat"),
                    fake_pane("%11", "1", "bash", "[PARKED] seat"),
                ],
            ),
            fake_win("work", "@2", "plain", "bash"),
        ];
        let root = built(wins, &forging);
        assert!(
            root.children[0].children.iter().all(|node| node.status == Status::Unknown),
            "the fixture must have no real status for this to prove anything"
        );
        let text = dumped(&root, &forging);
        for word in ["[RUNNING]", "[PARKED]", "[idle]"] {
            assert!(!text.contains(word), "a row forged the status field {word}:\n{text}");
        }
        // Still one window marker per window and one pane marker per pane.
        assert_eq!(marker_ids(&text, "window"), ["@1", "@2"], "{text}");
        assert_eq!(marker_ids(&text, "pane"), ["%10", "%11", "%2"], "{text}");

        // Positive control: the field the renderer owns must actually be
        // rendered in that exact form, or "no status word anywhere" would pass
        // with status rendering deleted.
        let parked = config::load_from_str(
            "[[sampler]]\nname = \"parked-command\"\nfield = \"pane_current_command\"\nregex = \"PARKED\"\nstatus = \"parked\"\n",
        )
            .expect("the status configuration parses");
        let root = built(vec![fake_win("work", "@1", "job", "bash-PARKED")], &parked);
        assert_eq!(root.children[0].children[0].status, Status::Parked);
        let text = dumped(&root, &parked);
        assert!(text.contains("[PARKED]"), "the genuine status field is not rendered:\n{text}");
    }

    /// A title any process in a pane can set with one escape sequence.
    const HOSTILE_TITLE: &str =
        "\u{1b}[31m\u{1b}[2Jred\u{1b}]0;owned\u{7}\r\nsecond\tline\u{8}\u{9b}5m";

    fn fake_pane(id: &str, index: &str, command: &str, title: &str) -> Pane {
        Pane {
            id: id.to_string(),
            index: index.to_string(),
            left: 0,
            top: 0,
            width: 80,
            height: 24,
            active: index == "0",
            cmd: command.to_string(),
            path: "/tmp".to_string(),
            title: title.to_string(),
        }
    }

    fn fake_win_panes(session: &str, id: &str, name: &str, panes: Vec<Pane>) -> Win {
        Win {
            session: session.to_string(),
            id: id.to_string(),
            index: id.trim_start_matches('@').to_string(),
            name: name.to_string(),
            w: 80,
            h: 24,
            panes,
        }
    }

    fn fake_win(session: &str, id: &str, name: &str, command: &str) -> Win {
        let pane = fake_pane(&format!("%{}", id.trim_start_matches('@')), "0", command, "");
        fake_win_panes(session, id, name, vec![pane])
    }

    /// Deterministic tmux states. Each one carries at least one session, one
    /// solo or split window, and one window with more than one pane, so a
    /// scope that only exists on pane rows is still exercised everywhere.
    fn generated_states() -> Vec<(&'static str, Vec<Win>)> {
        vec![
            (
                "one session, one split window",
                vec![fake_win_panes(
                    "ops-cache",
                    "@1",
                    "api-deploy-staging",
                    vec![
                        fake_pane("%10", "0", "bash", ""),
                        fake_pane("%11", "1", "worker-a", "owner-1"),
                    ],
                )],
            ),
            (
                "two sessions, solo and split windows",
                vec![
                    fake_win("ops-cache", "@2", "api-deploy-staging", "bash"),
                    fake_win_panes(
                        "work",
                        "@3",
                        "notes",
                        vec![
                            fake_pane("%30", "0", "worker-a", "owner-1"),
                            fake_pane("%31", "1", "worker-b", "auditor-1"),
                            fake_pane("%32", "2", "bash", ""),
                        ],
                    ),
                    fake_win("work", "@4", "web-deploy-prod", "zsh"),
                ],
            ),
            (
                "hostile pane title and two windows of the same name",
                vec![
                    fake_win_panes(
                        "ops-edge",
                        "@5",
                        "api-deploy-staging",
                        vec![
                            fake_pane("%50", "0", "bash", HOSTILE_TITLE),
                            fake_pane("%51", "1", "bash", ""),
                        ],
                    ),
                    fake_win("ops-edge", "@6", "api-deploy-staging", "bash"),
                ],
            ),
        ]
    }

    /// One reinterpreter configuration per scope, plus all of them at once.
    /// Every entry matches every raw name in its scope, so each configuration
    /// is guaranteed to reach the display of every state above — which is what
    /// makes the structural equality in `c8_*` a caused comparison rather than
    /// an awaited one.
    fn reinterpreter_configs() -> Vec<(&'static str, Config)> {
        let entry = |scope: &str| {
            format!(
                "[[reinterpreter]]\nscope = \"{scope}\"\npattern = \"^(?P<raw>.+)$\"\nlabel = \"{scope}!$raw\"\n\n"
            )
        };
        let parse = |source: String| {
            config::load_from_str(&source).expect("generated reinterpreter config parses")
        };
        vec![
            ("session-scoped rules", parse(entry("session"))),
            ("window-scoped rules", parse(entry("window"))),
            ("pane-scoped rules", parse(entry("pane"))),
            (
                "rules in all three scopes",
                parse(format!("{}{}{}", entry("session"), entry("window"), entry("pane"))),
            ),
        ]
    }

    /// Nodes in the tree, root included.
    fn node_count(node: &Node) -> usize {
        1 + node.children.iter().map(node_count).sum::<usize>()
    }

    /// Every display string the tree would draw, in row order.
    fn display_strings(node: &Node, config: &Config) -> Vec<String> {
        let mut out = vec![node_label(node, config)];
        for child in &node.children {
            out.extend(display_strings(child, config));
        }
        out
    }

    /// Every pane this tree can jump to, in row order.
    fn pane_targets(node: &Node) -> Vec<String> {
        let mut out: Vec<String> =
            selected_pane_id(node).map(ToString::to_string).into_iter().collect();
        for child in &node.children {
            out.extend(pane_targets(child));
        }
        out
    }

    /// The dump under proof. `dump` gains the configuration it renders labels
    /// with in this slice (FN-26-DUMP); the assertions do not change with it.
    fn dumped(node: &Node, config: &Config) -> String {
        dump(node, config)
    }

    /// Identity markers read out of a dump the way the immutable gate reads
    /// them: line-oriented, one marker per line.
    fn marker_ids(text: &str, key: &str) -> Vec<String> {
        let open = format!("[{key}=");
        text.lines()
            .filter_map(|line| {
                let id = line.split(&open).nth(1)?.split(']').next()?;
                (!id.is_empty()).then(|| id.to_string())
            })
            .collect()
    }

    #[test]
    fn empty_config_dumps_every_window_under_its_session() {
        let wins = vec![
            fake_win("alpha", "@1", "editor", "bash"),
            fake_win("alpha", "@2", "tests", "cargo"),
            fake_win("beta", "@3", "shell", "zsh"),
            fake_win("beta", "@4", "logs", "tail"),
        ];

        let root = built(wins, &Config::empty());

        assert_eq!(root.children.len(), 2);
        assert!(root.children.iter().all(|node| node.kind == Kind::SessionGroup));
        assert_eq!(root.children[0].title, "alpha");
        assert_eq!(root.children[1].title, "beta");
        assert_eq!(root.children[0].children.len(), 2);
        assert_eq!(root.children[1].children.len(), 2);
        assert_eq!(root.window_count(), 4);
        assert_eq!(
            dumped(&root, &Config::empty()),
            concat!(
                "• alpha\n",
                "  • editor  [window=@1]  [pane=%1]\n",
                "  • tests  [window=@2]  [pane=%2]\n",
                "• beta\n",
                "  • shell  [window=@3]  [pane=%3]\n",
                "  • logs  [window=@4]  [pane=%4]\n",
            )
        );
    }

    #[test]
    fn session_alias_is_a_reinterpreter_and_infra_stays_label_only() {
        // The alias table this ticket deletes was a second renaming mechanism
        // that could also merge two raw sessions into one bucket. Expressed as
        // a session-scoped reinterpreter it renames exactly one row, keeps the
        // raw name on it, and leaves identity alone.
        let config = config::load_from_str(
            r#"
[sessions]
infra = ["ops-*"]

[[reinterpreter]]
scope = "session"
pattern = "^shop$"
label = "acme"

[[sampler]]
name = "parked-command"
field = "pane_current_command"
regex = "PARKED"
status = "parked"

[[sampler]]
name = "working-command"
field = "pane_current_command"
regex = "^worker$"
status = "running"

[[sampler]]
name = "resting-command"
field = "pane_current_command"
regex = "^bash$"
status = "idle"
"#,
        )
        .expect("config parses");
        let wins = vec![
            fake_win("shop", "@1", "storefront", "worker"),
            fake_win("ops-cache", "@2", "maintenance", "bash-PARKED"),
        ];

        let root = built(wins, &config);

        // Identity stays raw; only the label moves.
        assert_eq!(root.children[0].id, "session/shop");
        assert_eq!(root.children[0].title, "shop");
        assert_eq!(node_label(&root.children[0], &config), "acme (shop)");
        assert_eq!(root.children[0].status, Status::Running);
        assert_eq!(node_label(&root.children[1], &config), "ops-cache [infra]");
        assert_eq!(root.children[1].status, Status::Parked);
        assert_eq!(
            dumped(&root, &config),
            concat!(
                "• acme (shop)  [RUNNING]\n",
                "  • storefront  [RUNNING]  [window=@1]  [pane=%1]\n",
                "• ops-cache [infra]  [PARKED]\n",
                "  • maintenance  [PARKED]  [window=@2]  [pane=%2]\n",
            )
        );
    }

    #[test]
    fn every_configured_sampler_status_reaches_status_and_dump() {
        let cases = [
            (
                "running",
                "[[sampler]]\nname = \"working\"\nfield = \"pane_current_command\"\nregex = \"^worker$\"\nstatus = \"running\"\n",
                "worker",
                Status::Running,
                "RUNNING",
            ),
            (
                "idle",
                "[[sampler]]\nname = \"resting\"\nfield = \"pane_current_command\"\nregex = \"^bash$\"\nstatus = \"idle\"\n",
                "bash",
                Status::Idle,
                "idle",
            ),
            (
                "parked",
                "[[sampler]]\nname = \"parked\"\nfield = \"pane_current_command\"\nregex = \"PARKED\"\nstatus = \"parked\"\n",
                "bash-PARKED",
                Status::Parked,
                "PARKED",
            ),
        ];

        for (bucket, source, command, expected_status, expected_label) in cases {
            let config = config::load_from_str(source).expect("exclusive status config parses");
            let root = built(vec![fake_win("alpha", "@1", "window", command)], &config);
            let window = &root.children[0].children[0];

            assert_eq!(window.status, expected_status, "status bucket {bucket}");
            assert_eq!(
                dumped(&root, &config),
                format!(
                    "• alpha  [{expected_label}]\n  • window  [{expected_label}]  [window=@1]  [pane=%1]\n"
                ),
                "dump label for status bucket {bucket}"
            );
        }
    }

    #[test]
    fn c1_title_evidence_distinguishes_work_from_command_occupancy() {
        let config = config::load_from_str(
            r#"
[[sampler]]
name = "busy-title"
field = "pane_title"
regex = "^busy:"
status = "running"
"#,
        )
        .expect("sampler config parses");
        let wins = vec![
            fake_win_panes(
                "work",
                "@1",
                "waiting",
                vec![fake_pane("%1", "0", "occupied", "waiting")],
            ),
            fake_win_panes(
                "work",
                "@2",
                "active",
                vec![fake_pane("%2", "0", "occupied", "busy: compiling")],
            ),
        ];

        let root = built(wins, &config);
        let waiting = &root.children[0].children[0];
        let active = &root.children[0].children[1];
        assert_eq!(waiting.status, Status::Unknown, "occupancy alone reported RUNNING");
        assert_eq!(active.status, Status::Running, "matching evidence was not RUNNING");
    }

    #[test]
    fn c2_every_supported_field_is_queried_resolved_and_evaluated() {
        let tmux_source = include_str!("tmux.rs");
        for (field, expected) in [
            ("pane_current_command", "occupied"),
            ("pane_current_path", "/tmp"),
            ("pane_title", "busy: working"),
            ("window_name", "queue"),
        ] {
            assert!(tmux_source.contains(&format!("#{{{field}}}")), "tmux does not query {field}");
            let source = format!(
                "[[sampler]]\nname = \"field-{field}\"\nfield = \"{field}\"\nregex = \"^{}$\"\nstatus = \"running\"\n",
                regex::escape(expected)
            );
            let config = config::load_from_str(&source).expect("supported sampler parses");
            let pane = fake_pane("%1", "0", "occupied", "busy: working");
            let root = built(vec![fake_win_panes("work", "@1", "queue", vec![pane])], &config);
            assert_eq!(
                root.children[0].children[0].status,
                Status::Running,
                "supported field {field} was not resolved and evaluated"
            );
        }
    }

    #[test]
    fn c_rollup_keeps_per_pane_status_and_all_unknown_unmarked() {
        let config = config::load_from_str(
            r#"
[[sampler]]
name = "busy"
field = "pane_title"
regex = "^busy:"
status = "running"

[[sampler]]
name = "resting"
field = "pane_title"
regex = "^rest:"
status = "idle"
"#,
        )
        .expect("sampler config parses");
        let root = built(
            vec![
                fake_win_panes(
                    "work",
                    "@1",
                    "mixed",
                    vec![
                        fake_pane("%10", "0", "occupied", "busy: working"),
                        fake_pane("%11", "1", "occupied", "rest: waiting"),
                        fake_pane("%12", "2", "occupied", "unmarked"),
                    ],
                ),
                fake_win_panes(
                    "work",
                    "@2",
                    "unknown",
                    vec![
                        fake_pane("%20", "0", "occupied", "unmarked one"),
                        fake_pane("%21", "1", "occupied", "unmarked two"),
                    ],
                ),
            ],
            &config,
        );

        let mixed = &root.children[0].children[0];
        assert_eq!(
            mixed.children.iter().map(|node| node.status).collect::<Vec<_>>(),
            [Status::Running, Status::Idle, Status::Unknown]
        );
        assert_eq!(mixed.status, Status::Running, "Unknown lowered an established status");

        let unknown = &root.children[0].children[1];
        assert!(unknown.children.iter().all(|node| node.status == Status::Unknown));
        assert_eq!(unknown.status, Status::Unknown, "all-unmarked became a positive idle reading");
    }

    #[test]
    fn ordered_samplers_use_the_first_match_per_pane() {
        let config = config::load_from_str(
            r#"
[[sampler]]
name = "broad-idle"
field = "pane_title"
regex = "working"
status = "idle"

[[sampler]]
name = "later-running"
field = "pane_title"
regex = "^busy: working$"
status = "running"
"#,
        )
        .expect("sampler config parses");
        let root = built(
            vec![fake_win_panes(
                "work",
                "@1",
                "ordered",
                vec![fake_pane("%1", "0", "occupied", "busy: working")],
            )],
            &config,
        );
        assert_eq!(root.children[0].children[0].status, Status::Idle);
    }

    #[test]
    fn c4_shipped_example_sampler_has_its_documented_effect() {
        let source = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/config.toml"),
        )
        .expect("actual shipped example is present");
        assert!(source.contains("[[sampler]]"), "actual shipped example has no sampler");
        let config = config::load_from_str(&source).expect("actual shipped example parses");
        let root = built(
            vec![fake_win_panes(
                "work",
                "@1",
                "sample",
                vec![fake_pane("%1", "0", "occupied", "⠁ working")],
            )],
            &config,
        );
        assert_eq!(root.children[0].children[0].status, Status::Running);
    }

    #[test]
    fn unknown_top_level_config_table_is_forward_compatible() {
        let config = config::load_from_str(
            r#"
[fabricated]
future = true

[[reinterpreter]]
scope = "window"
pattern = "^notes$"
label = "inbox"
"#,
        )
        .expect("unknown top-level tables are ignored");

        assert!(reinterpret(Scope::Window, "notes", &config).starts_with("inbox"));
    }

    #[test]
    fn config_load_precedence_is_env_then_xdg_then_empty() {
        let _lock = ENV_LOCK.lock().expect("environment lock");
        let temp = TempConfig::new();
        let env_path = temp.root.join("explicit.toml");
        let xdg_path = temp.root.join("xdg/factory-tui/config.toml");
        fs::create_dir_all(xdg_path.parent().expect("XDG config parent")).expect("create XDG");
        let source = |label: &str| {
            format!("[[reinterpreter]]\nscope = \"window\"\npattern = \"^source$\"\nlabel = \"{label}\"\n")
        };
        fs::write(&env_path, source("env")).expect("write env config");
        fs::write(&xdg_path, source("xdg")).expect("write XDG config");

        let factory = EnvRestore::set("FACTORY_TUI_CONFIG", Some(env_path.as_os_str()));
        let xdg = EnvRestore::set("XDG_CONFIG_HOME", Some(temp.root.join("xdg").as_os_str()));

        let loaded = config::load();
        assert!(reinterpret(Scope::Window, "source", &loaded).starts_with("env"));

        env::remove_var("FACTORY_TUI_CONFIG");
        let loaded = config::load();
        assert!(reinterpret(Scope::Window, "source", &loaded).starts_with("xdg"));

        env::set_var("XDG_CONFIG_HOME", temp.root.join("missing"));
        assert_eq!(config::load(), Config::empty());

        drop(xdg);
        drop(factory);
    }

    #[test]
    fn names_that_look_like_a_fold_path_stay_one_raw_window() {
        // The deleted projection would have turned this single window into a
        // project folder, a milestone folder and a leaf. There is no longer a
        // configuration that can do that to it.
        let wins = vec![fake_win("shop", "@1", "acme-ms1-ship", "bash")];
        let root = built(wins, &Config::empty());
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].kind, Kind::SessionGroup);
        assert_eq!(root.children[0].title, "shop");
        assert_eq!(root.children[0].children.len(), 1);
        assert_eq!(root.children[0].children[0].kind, Kind::Window);
        assert_eq!(root.children[0].children[0].title, "acme-ms1-ship");
    }

    #[test]
    fn preview_and_jump_tmux_commands_remain_non_resizing() {
        let source = include_str!("tmux.rs");
        assert!(source.contains("capture-pane"));
        assert!(source.contains("switch-client"));
        assert!(source.contains("select-window"));
        assert!(!source.contains("resize-pane"));
    }

    struct TempConfig {
        root: PathBuf,
    }

    impl TempConfig {
        fn new() -> Self {
            let unique =
                SystemTime::now().duration_since(UNIX_EPOCH).expect("clock after epoch").as_nanos();
            let root =
                env::temp_dir().join(format!("factory-tui-config-{}-{unique}", std::process::id()));
            fs::create_dir_all(&root).expect("create temp config root");
            Self { root }
        }
    }

    impl Drop for TempConfig {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.root).expect("remove temp config root");
        }
    }

    struct EnvRestore {
        key: &'static str,
        previous: Option<OsString>,
    }

    impl EnvRestore {
        fn set(key: &'static str, value: Option<&std::ffi::OsStr>) -> Self {
            let previous = env::var_os(key);
            match value {
                Some(value) => env::set_var(key, value),
                None => env::remove_var(key),
            }
            Self { key, previous }
        }
    }

    impl Drop for EnvRestore {
        fn drop(&mut self) {
            match &self.previous {
                Some(value) => env::set_var(self.key, value),
                None => env::remove_var(self.key),
            }
        }
    }
}
