//! Build a raw session/window tree from a live tmux census.

use std::collections::BTreeMap;

use crate::config::Config;
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
    SessionGroup,
    Folder,
    Window,
}

/// One node in the tmux tree.
#[derive(Clone, Debug)]
pub struct Node {
    pub id: String,
    pub kind: Kind,
    pub title: String,
    pub status: Status,
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

struct SessionBucket {
    name: String,
    is_infra: bool,
    windows: Vec<Win>,
}

/// Group every observed window under its configured session name,
/// or fold matching windows when projection rules are present.
pub fn build(wins: Vec<Win>, config: &Config) -> Node {
    if config.projects() {
        return build_projected(wins, config);
    }
    build_raw(wins, config)
}

fn build_raw(wins: Vec<Win>, config: &Config) -> Node {
    let mut sessions: Vec<SessionBucket> = Vec::new();

    for win in wins {
        let name = config.session_alias(&win.session).unwrap_or(&win.session).to_string();
        let is_infra = config.is_infra(&win.session);
        if let Some(session) = sessions.iter_mut().find(|session| session.name == name) {
            session.is_infra |= is_infra;
            session.windows.push(win);
        } else {
            sessions.push(SessionBucket { name, is_infra, windows: vec![win] });
        }
    }

    let children =
        sessions.into_iter().map(|session| session_node(session, config)).collect::<Vec<_>>();
    let status = rollup(&children);

    Node {
        id: "root".to_string(),
        kind: Kind::SessionGroup,
        title: "tmux".to_string(),
        status,
        win: None,
        children,
    }
}

fn session_node(session: SessionBucket, config: &Config) -> Node {
    let id = format!("session/{}", session.name);
    let title = if session.is_infra { format!("{} [infra]", session.name) } else { session.name };
    let children =
        session.windows.into_iter().map(|win| window_node(&id, win, config)).collect::<Vec<_>>();
    let status = rollup(&children);

    Node { id, kind: Kind::SessionGroup, title, status, win: None, children }
}

fn window_node(parent: &str, win: Win, config: &Config) -> Node {
    let status = status_of(&win, config);
    Node {
        id: format!("{parent}/{}", win.id),
        kind: Kind::Window,
        title: win.name.clone(),
        status,
        win: Some(win),
        children: Vec::new(),
    }
}

fn build_projected(wins: Vec<Win>, config: &Config) -> Node {
    let mut rows: Vec<(Win, Option<crate::config::Classified>)> = wins
        .into_iter()
        .map(|win| {
            let classified = crate::config::classify(&win, config);
            (win, classified)
        })
        .collect();
    inherit_milestones(&mut rows, config);

    let mut folders = FolderAcc::default();
    let mut unmatched: Vec<Win> = Vec::new();
    for (win, classified) in rows {
        match classified {
            Some(classified) => {
                let path = folder_path(config, &classified);
                if path.is_empty() {
                    unmatched.push(win);
                } else {
                    folders.insert(path, win, classified, config);
                }
            }
            None => unmatched.push(win),
        }
    }

    let mut children = folders.into_nodes("project", config);
    if !unmatched.is_empty() {
        children.extend(build_raw(unmatched, config).children);
    }
    let status = rollup(&children);
    Node {
        id: "root".to_string(),
        kind: Kind::SessionGroup,
        title: "tmux".to_string(),
        status,
        win: None,
        children,
    }
}

fn inherit_milestones(rows: &mut [(Win, Option<crate::config::Classified>)], config: &Config) {
    if !config.tree.inherit_milestone_from_desk {
        return;
    }
    let mut session_ms: BTreeMap<String, String> = BTreeMap::new();
    for (win, classified) in rows.iter() {
        let Some(classified) = classified else {
            continue;
        };
        if classified
            .role
            .as_deref()
            .is_some_and(|role| config.tree.desk_roles.iter().any(|desk| desk == role))
        {
            if let Some(milestone) = &classified.milestone {
                session_ms.insert(win.session.clone(), milestone.clone());
            }
        }
    }
    for (win, classified) in rows.iter_mut() {
        let Some(classified) = classified else {
            continue;
        };
        if classified.milestone.is_none() {
            if let Some(milestone) = session_ms.get(&win.session) {
                classified.milestone = Some(milestone.clone());
            }
        }
    }
}

fn folder_path(config: &Config, classified: &crate::config::Classified) -> Vec<(String, String)> {
    let mut path = Vec::new();
    for field in &config.tree.folders {
        match folder_step(field, classified) {
            Some((id, title)) => path.push((id, title)),
            None => break,
        }
    }
    path
}

fn folder_step(field: &str, classified: &crate::config::Classified) -> Option<(String, String)> {
    match field {
        "project" => classified.project.clone().map(|value| (format!("p/{value}"), value)),
        "milestone" => {
            classified.milestone.clone().map(|value| (format!("m/{value}"), format!("M{value}")))
        }
        "epic" => classified.epic.clone().map(|value| {
            let title = if value.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                format!("e{value}")
            } else {
                format!("e-{value}")
            };
            (format!("e/{value}"), title)
        }),
        "ticket" => {
            classified.ticket.clone().map(|value| (format!("t/{value}"), format!("t{value}")))
        }
        "role" => classified.role.clone().map(|value| (format!("r/{value}"), value)),
        "goal" => classified.goal.clone().map(|value| (format!("g/{value}"), value)),
        _ => None,
    }
}

#[derive(Default)]
struct FolderAcc {
    children: BTreeMap<String, FolderAcc>,
    titles: BTreeMap<String, String>,
    desk: Option<Win>,
    leaves: Vec<(Win, crate::config::Classified)>,
}

impl FolderAcc {
    fn insert(
        &mut self,
        path: Vec<(String, String)>,
        win: Win,
        classified: crate::config::Classified,
        config: &Config,
    ) {
        if path.is_empty() {
            self.leaves.push((win, classified));
            return;
        }
        let ((id, title), rest) = path.split_first().expect("path not empty");
        self.titles.insert(id.clone(), title.clone());
        let child = self.children.entry(id.clone()).or_default();
        if rest.is_empty() {
            let is_desk = classified
                .role
                .as_deref()
                .is_some_and(|role| config.tree.desk_roles.iter().any(|desk| desk == role));
            if is_desk && child.desk.is_none() {
                child.desk = Some(win);
                if let Some(goal) = &classified.goal {
                    self.titles.insert(id.clone(), format!("{title} {goal}"));
                }
            } else {
                child.leaves.push((win, classified));
            }
        } else {
            child.insert(rest.to_vec(), win, classified, config);
        }
    }

    fn into_nodes(self, parent: &str, config: &Config) -> Vec<Node> {
        let mut nodes = Vec::new();
        for (id, child) in self.children {
            let title = self.titles.get(&id).cloned().unwrap_or_else(|| id.clone());
            let node_id = format!("{parent}/{id}");
            let desk = child.desk.clone();
            let kids = child.into_nodes(&node_id, config);
            let status = match &desk {
                Some(win) => worse(status_of(win, config), rollup(&kids)),
                None => rollup(&kids),
            };
            nodes.push(Node {
                id: node_id,
                kind: Kind::Folder,
                title,
                status,
                win: desk,
                children: kids,
            });
        }
        for (win, classified) in self.leaves {
            let title = leaf_title(&classified, &win);
            let mut node = window_node(parent, win, config);
            node.title = title;
            nodes.push(node);
        }
        nodes
    }
}

fn leaf_title(classified: &crate::config::Classified, win: &Win) -> String {
    if let Some(ticket) = &classified.ticket {
        let label = if ticket.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            format!("t{ticket}")
        } else {
            format!("t-{ticket}")
        };
        if let Some(goal) = &classified.goal {
            if goal != ticket {
                return format!("{label} {goal}");
            }
        }
        return label;
    }
    classified.goal.clone().unwrap_or_else(|| win.name.clone())
}

fn status_of(win: &Win, config: &Config) -> Status {
    if !config.status.parked_substring.is_empty()
        && (win.name.contains(&config.status.parked_substring)
            || win.panes.iter().any(|pane| pane.cmd.contains(&config.status.parked_substring)))
    {
        return Status::Parked;
    }
    if win.panes.iter().any(|pane| config.status.running.iter().any(|command| command == &pane.cmd))
    {
        return Status::Running;
    }
    if win.panes.iter().any(|pane| config.status.idle.iter().any(|command| command == &pane.cmd)) {
        return Status::Idle;
    }
    Status::Unknown
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

/// Print a plain-text tree for `--dump` and tests.
pub fn dump(node: &Node) -> String {
    let mut output = String::new();
    dump_into(node, 0, &mut output);
    output
}

fn dump_into(node: &Node, depth: usize, output: &mut String) {
    if node.id != "root" {
        output.push_str(&"  ".repeat(depth));
        output.push_str("• ");
        output.push_str(&node.title);
        let status = status_label(node.status);
        if !status.is_empty() {
            output.push_str("  ");
            output.push_str(status);
        }
        if let Some(win) = &node.win {
            output.push_str(&format!("  [{}:{}]", win.session, win.name));
        }
        output.push('\n');
    }

    let child_depth = if node.id == "root" { depth } else { depth + 1 };
    for child in &node.children {
        dump_into(child, child_depth, output);
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

#[cfg(test)]
mod tests {
    use std::env;
    use std::ffi::OsString;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::Mutex;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{build, dump, Kind, Status};
    use crate::config::{self, Config};
    use crate::tmux::{Pane, Win};

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn fake_win(session: &str, id: &str, name: &str, command: &str) -> Win {
        Win {
            session: session.to_string(),
            id: id.to_string(),
            index: id.trim_start_matches('@').to_string(),
            name: name.to_string(),
            w: 80,
            h: 24,
            panes: vec![Pane {
                id: format!("%{id}"),
                index: "0".to_string(),
                left: 0,
                top: 0,
                width: 80,
                height: 24,
                active: true,
                cmd: command.to_string(),
                path: "/tmp".to_string(),
                // Tree shape does not read pane titles.
                title: String::new(),
            }],
        }
    }

    #[test]
    fn empty_config_dumps_every_window_under_its_session() {
        let wins = vec![
            fake_win("alpha", "@1", "editor", "bash"),
            fake_win("alpha", "@2", "tests", "cargo"),
            fake_win("beta", "@3", "shell", "zsh"),
            fake_win("beta", "@4", "logs", "tail"),
        ];

        let root = build(wins, &Config::empty());

        assert_eq!(root.children.len(), 2);
        assert!(root.children.iter().all(|node| node.kind == Kind::SessionGroup));
        assert_eq!(root.children[0].title, "alpha");
        assert_eq!(root.children[1].title, "beta");
        assert_eq!(root.children[0].children.len(), 2);
        assert_eq!(root.children[1].children.len(), 2);
        assert_eq!(root.window_count(), 4);
        assert_eq!(
            dump(&root),
            concat!(
                "• alpha\n",
                "  • editor  [alpha:editor]\n",
                "  • tests  [alpha:tests]\n",
                "• beta\n",
                "  • shell  [beta:shell]\n",
                "  • logs  [beta:logs]\n",
            )
        );
    }

    #[test]
    fn configured_tables_change_alias_infra_and_status_output() {
        let config = config::load_from_str(
            r#"
[sessions]
infra = ["ops-*"]

[sessions.alias]
shop = "acme"

[status]
running = ["worker"]
idle = ["bash"]
parked_substring = "PARKED"
"#,
        )
        .expect("config parses");
        let wins = vec![
            fake_win("shop", "@1", "storefront", "worker"),
            fake_win("ops-cache", "@2", "maintenance", "bash-PARKED"),
        ];

        let root = build(wins, &config);

        assert_eq!(root.children[0].title, "acme");
        assert_eq!(root.children[0].status, Status::Running);
        assert_eq!(root.children[1].title, "ops-cache [infra]");
        assert_eq!(root.children[1].status, Status::Parked);
        assert_eq!(
            dump(&root),
            concat!(
                "• acme  RUNNING\n",
                "  • storefront  RUNNING  [shop:storefront]\n",
                "• ops-cache [infra]  PARKED\n",
                "  • maintenance  PARKED  [ops-cache:maintenance]\n",
            )
        );
    }

    #[test]
    fn every_configured_status_bucket_reaches_status_and_dump() {
        let cases = [
            ("running", "[status]\nrunning = [\"worker\"]\n", "worker", Status::Running, "RUNNING"),
            ("idle", "[status]\nidle = [\"bash\"]\n", "bash", Status::Idle, "idle"),
            (
                "parked_substring",
                "[status]\nparked_substring = \"PARKED\"\n",
                "bash-PARKED",
                Status::Parked,
                "PARKED",
            ),
        ];

        for (bucket, source, command, expected_status, expected_label) in cases {
            let config = config::load_from_str(source).expect("exclusive status config parses");
            let root = build(vec![fake_win("alpha", "@1", "window", command)], &config);
            let window = &root.children[0].children[0];

            assert_eq!(window.status, expected_status, "status bucket {bucket}");
            assert_eq!(
                dump(&root),
                format!(
                    "• alpha  {expected_label}\n  • window  {expected_label}  [alpha:window]\n"
                ),
                "dump label for status bucket {bucket}"
            );
        }
    }

    #[test]
    fn unknown_top_level_config_table_is_forward_compatible() {
        let config = config::load_from_str(
            r#"
[fabricated]
future = true

[sessions.alias]
shop = "acme"
"#,
        )
        .expect("unknown top-level tables are ignored");

        assert_eq!(config.sessions.alias.get("shop").map(String::as_str), Some("acme"));
    }

    #[test]
    fn config_load_precedence_is_env_then_xdg_then_empty() {
        let _lock = ENV_LOCK.lock().expect("environment lock");
        let temp = TempConfig::new();
        let env_path = temp.root.join("explicit.toml");
        let xdg_path = temp.root.join("xdg/factory-tui/config.toml");
        fs::create_dir_all(xdg_path.parent().expect("XDG config parent")).expect("create XDG");
        fs::write(&env_path, "[sessions.alias]\nsource = \"env\"\n").expect("write env config");
        fs::write(&xdg_path, "[sessions.alias]\nsource = \"xdg\"\n").expect("write XDG config");

        let factory = EnvRestore::set("FACTORY_TUI_CONFIG", Some(env_path.as_os_str()));
        let xdg = EnvRestore::set("XDG_CONFIG_HOME", Some(temp.root.join("xdg").as_os_str()));

        let loaded = config::load();
        assert_eq!(loaded.sessions.alias.get("source").map(String::as_str), Some("env"));

        env::remove_var("FACTORY_TUI_CONFIG");
        let loaded = config::load();
        assert_eq!(loaded.sessions.alias.get("source").map(String::as_str), Some("xdg"));

        env::set_var("XDG_CONFIG_HOME", temp.root.join("missing"));
        assert_eq!(config::load(), Config::empty());

        drop(xdg);
        drop(factory);
    }

    #[test]
    fn example_projection_folds_and_leaves_unmatched_under_session() {
        let config = config::load_from_str(
            r#"
[sessions]
infra = ["0-*"]

[[rule]]
window = "^(?:(?P<project>.+)-)?ms(?P<milestone>[0-9]+)-(?P<goal>.+)$"
role = "desk"

[[rule]]
window = "^(?:(?P<project>.+)-)?e(?P<epic>[0-9]+)-t(?P<ticket>[0-9]+)-(?P<goal>.+)$"
role = "ticket"

[tree]
folders = ["project", "milestone", "epic"]
desk_roles = ["desk"]
inherit_milestone_from_desk = true
"#,
        )
        .expect("example config parses");
        let wins = vec![
            fake_win("shop", "@1", "acme-ms1-ship", "claude"),
            fake_win("shop", "@2", "acme-e4-t18-rename", "codex"),
            fake_win("shop", "@3", "notes", "bash"),
        ];
        let root = build(wins, &config);
        let dump = dump(&root);
        assert!(dump.contains("• acme"), "{dump}");
        assert!(dump.contains("M1"), "{dump}");
        assert!(dump.contains("t18 rename"), "{dump}");
        assert!(dump.contains("notes"), "{dump}");
        assert_eq!(root.window_count(), 3);
    }

    #[test]
    fn no_rules_keeps_session_tree_even_if_names_look_folded() {
        let wins = vec![fake_win("shop", "@1", "acme-ms1-ship", "bash")];
        let root = build(wins, &Config::empty());
        assert_eq!(root.children[0].kind, Kind::SessionGroup);
        assert_eq!(root.children[0].title, "shop");
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
