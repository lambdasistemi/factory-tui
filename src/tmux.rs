//! Live tmux census. Cross-session on purpose — this is the factory, not one
//! session's window list.

use std::io;
use std::process::Command;

/// A pane as reported by tmux, with cell geometry inside its window.
#[derive(Clone, Debug)]
pub struct Pane {
    pub id: String,
    pub index: String,
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16,
    pub active: bool,
    pub cmd: String,
    pub path: String,
    /// Whatever the pane set as its tmux title. Any process in the pane can
    /// write this with one escape sequence, so it stays untrusted until a
    /// display label is composed from it.
    pub title: String,
}

/// One window, every pane, plus the session that holds it.
#[derive(Clone, Debug)]
pub struct Win {
    pub session: String,
    pub id: String,
    pub index: String,
    pub name: String,
    pub w: u16,
    pub h: u16,
    pub panes: Vec<Pane>,
}

fn out(args: &[&str]) -> io::Result<String> {
    let o = Command::new("tmux").args(args).output()?;
    if !o.status.success() {
        return Err(io::Error::other(String::from_utf8_lossy(&o.stderr).trim().to_string()));
    }
    Ok(String::from_utf8_lossy(&o.stdout).to_string())
}

fn fire(args: &[&str]) -> io::Result<()> {
    let st = Command::new("tmux").args(args).status()?;
    if !st.success() {
        return Err(io::Error::other(format!("tmux {args:?} failed")));
    }
    Ok(())
}

fn parse_pane(f: &[&str]) -> Option<Pane> {
    if f.len() < 10 {
        return None;
    }
    Some(Pane {
        id: f[0].to_string(),
        index: f[1].to_string(),
        left: f[2].parse().unwrap_or(0),
        top: f[3].parse().unwrap_or(0),
        width: f[4].parse().unwrap_or(1),
        height: f[5].parse().unwrap_or(1),
        active: f[6].trim() == "1",
        cmd: f[7].to_string(),
        path: f[8].to_string(),
        title: f[9].to_string(),
    })
}

/// One row per pane. `#{pane_title}` is requested last on purpose: a pane can
/// put anything in its own title, including a tab, and a trailing field can
/// only ever add fields — never shift the ones parsed before it.
const QUERY_FORMAT: &str = "#{session_name}\t#{session_attached}\t#{window_id}\t#{window_index}\t#{window_name}\t#{window_active}\t#{window_width}\t#{window_height}\t#{pane_id}\t#{pane_index}\t#{pane_left}\t#{pane_top}\t#{pane_width}\t#{pane_height}\t#{pane_active}\t#{pane_current_command}\t#{pane_current_path}\t#{pane_title}";

/// Fields `QUERY_FORMAT` produces. The row guard and the pane slice below both
/// read it, so they cannot drift apart.
const QUERY_FIELDS: usize = 18;

/// Group one `list-panes` census into windows, dropping any row that is not in
/// the format above.
fn parse_query(raw: &str) -> Vec<Win> {
    let mut wins: Vec<Win> = Vec::new();
    for line in raw.lines() {
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() < QUERY_FIELDS {
            continue;
        }
        let Some(pane) = parse_pane(&f[8..QUERY_FIELDS]) else {
            continue;
        };
        let wid = f[2].to_string();
        if let Some(w) = wins.iter_mut().find(|w| w.id == wid) {
            w.panes.push(pane);
        } else {
            wins.push(Win {
                session: f[0].to_string(),
                id: wid,
                index: f[3].to_string(),
                name: f[4].to_string(),
                w: f[6].parse().unwrap_or(80),
                h: f[7].parse().unwrap_or(24),
                panes: vec![pane],
            });
        }
    }
    wins.sort_by(|a, b| {
        a.session.cmp(&b.session).then_with(|| {
            a.index.parse::<i64>().unwrap_or(0).cmp(&b.index.parse::<i64>().unwrap_or(0))
        })
    });
    wins
}

/// Every window on the host, each with its panes.
pub fn query_all() -> io::Result<Vec<Win>> {
    Ok(parse_query(&out(&["list-panes", "-a", "-F", QUERY_FORMAT])?))
}

/// Visible pane text plus a short scrollback tail. This is a snapshot, not a
/// live embed — tmux will not composite another pane into this one.
pub fn capture_pane(id: &str) -> io::Result<String> {
    out(&["capture-pane", "-ept", id, "-J", "-S", "-80"])
}

/// Strip CSI/OSC so a TUI snapshot is readable as plain text.
pub fn strip_ansi(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\u{1b}' {
            match chars.peek() {
                Some('[') => {
                    chars.next();
                    for x in chars.by_ref() {
                        if x.is_ascii_alphabetic() || x == '~' {
                            break;
                        }
                    }
                }
                Some(']') => {
                    chars.next();
                    for x in chars.by_ref() {
                        if x == '\u{7}' {
                            break;
                        }
                    }
                }
                Some(_) => {
                    chars.next();
                }
                None => {}
            }
        } else if c == '\r' || c == '\u{8}' {
            // drop CR / backspace; keep the rest of the snapshot
        } else if c == '\t' || c == '\n' || !c.is_control() {
            out.push(c);
        }
    }
    out
}

/// Jump the attached client to this session/window, optionally a pane.
pub fn focus(session: &str, window_id: &str, pane: Option<&str>) -> io::Result<()> {
    fire(&["switch-client", "-t", session])?;
    fire(&["select-window", "-t", window_id])?;
    if let Some(pane) = pane {
        fire(&["select-pane", "-t", pane])?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{parse_query, strip_ansi};

    /// The eight window fields the census asks for, in order.
    const WIN: [&str; 8] = ["work", "1", "@7", "3", "editor", "1", "180", "48"];

    /// One census row in the exact shape `query_all` requests from tmux.
    fn row(window: &[&str], pane: &[&str]) -> String {
        window.iter().chain(pane.iter()).copied().collect::<Vec<_>>().join("\t")
    }

    #[test]
    fn strip_csi_and_cr() {
        assert_eq!(strip_ansi("\u{1b}[32mhello\u{1b}[0m\r\nworld"), "hello\nworld");
    }

    #[test]
    fn query_parser_preserves_all_fields_after_title_extension() {
        // The rows below are hand-built at the positions the parser reads, so
        // bind those positions to the format string tmux is actually given.
        // Without this the two halves drift apart with every test still green.
        assert_eq!(
            super::QUERY_FORMAT.split('\t').collect::<Vec<_>>(),
            [
                "#{session_name}",
                "#{session_attached}",
                "#{window_id}",
                "#{window_index}",
                "#{window_name}",
                "#{window_active}",
                "#{window_width}",
                "#{window_height}",
                "#{pane_id}",
                "#{pane_index}",
                "#{pane_left}",
                "#{pane_top}",
                "#{pane_width}",
                "#{pane_height}",
                "#{pane_active}",
                "#{pane_current_command}",
                "#{pane_current_path}",
                "#{pane_title}",
            ],
            "the title must be requested last, after every pre-existing field"
        );
        assert_eq!(super::QUERY_FORMAT.split('\t').count(), super::QUERY_FIELDS);

        let raw = format!(
            "{}\n{}\n",
            row(
                &WIN,
                &["%11", "0", "0", "0", "90", "48", "1", "claude", "/code/factory-tui", "owner-1"]
            ),
            row(
                &WIN,
                &[
                    "%12",
                    "1",
                    "90",
                    "0",
                    "90",
                    "48",
                    "0",
                    "claude",
                    "/code/factory-tui",
                    "auditor-1"
                ]
            ),
        );

        let wins = parse_query(&raw);
        assert_eq!(wins.len(), 1, "both rows name window @7");

        let w = &wins[0];
        assert_eq!(w.session, "work");
        assert_eq!(w.id, "@7");
        assert_eq!(w.index, "3");
        assert_eq!(w.name, "editor");
        assert_eq!((w.w, w.h), (180, 48));
        assert_eq!(w.panes.len(), 2);

        let p = &w.panes[0];
        assert_eq!(p.id, "%11");
        assert_eq!(p.index, "0");
        assert_eq!((p.left, p.top, p.width, p.height), (0, 0, 90, 48));
        assert!(p.active);
        assert_eq!(p.cmd, "claude");
        assert_eq!(p.path, "/code/factory-tui");
        assert_eq!(p.title, "owner-1");

        // Same command, different title: the distinction the ticket exists for.
        let q = &w.panes[1];
        assert_eq!(q.cmd, p.cmd);
        assert_eq!(q.title, "auditor-1");
        assert_eq!((q.left, q.top), (90, 0));
        assert!(!q.active);
    }

    #[test]
    fn query_parser_rejects_pre_extension_field_count() {
        // The pre-title census shape. The row-length guard and the pane slice
        // must agree about it: a guard that still admits 17 fields would slice
        // out of range, and a slice that still stops at 17 would drop the title
        // the test above requires.
        let short =
            row(&WIN, &["%11", "0", "0", "0", "90", "48", "1", "claude", "/code/factory-tui"]);
        assert_eq!(short.split('\t').count(), 17);
        assert!(parse_query(&short).is_empty(), "a titleless row carries no pane");

        // One malformed row must not take the rest of the census with it.
        let good =
            row(&WIN, &["%12", "1", "90", "0", "90", "48", "0", "bash", "/tmp", "auditor-1"]);
        let wins = parse_query(&format!("{short}\n{good}\n"));
        assert_eq!(wins.len(), 1);
        assert_eq!(wins[0].panes.len(), 1);
        assert_eq!(wins[0].panes[0].id, "%12");
        assert_eq!(wins[0].panes[0].title, "auditor-1");
    }

    #[test]
    fn query_parser_keeps_earlier_fields_when_title_contains_tab() {
        // Titles are attacker-controlled and may contain the field separator.
        // The title is requested last precisely so that can only add trailing
        // fields — never shift an earlier one.
        let raw = row(
            &WIN,
            &["%11", "0", "0", "0", "90", "48", "1", "claude", "/code/factory-tui", "split\there"],
        );

        let wins = parse_query(&raw);
        assert_eq!(wins.len(), 1);
        let p = &wins[0].panes[0];
        assert_eq!(p.cmd, "claude");
        assert_eq!(p.path, "/code/factory-tui", "the path must not absorb the title");
        assert_eq!(p.title, "split", "an embedded tab truncates the title, it never shifts");
    }
}
