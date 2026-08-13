//! Build a raw session/window tree from a live tmux census.

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

/// Group every observed window under its configured session name.
pub fn build(wins: Vec<Win>, config: &Config) -> Node {
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

fn status_of(win: &Win, config: &Config) -> Status {
    if !config.status.parked_substring.is_empty()
        && win.panes.iter().any(|pane| pane.cmd.contains(&config.status.parked_substring))
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
