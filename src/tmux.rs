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
    if f.len() < 9 {
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
    })
}

/// Every window on the host, each with its panes.
pub fn query_all() -> io::Result<Vec<Win>> {
    let fmt = "#{session_name}\t#{session_attached}\t#{window_id}\t#{window_index}\t#{window_name}\t#{window_active}\t#{window_width}\t#{window_height}\t#{pane_id}\t#{pane_index}\t#{pane_left}\t#{pane_top}\t#{pane_width}\t#{pane_height}\t#{pane_active}\t#{pane_current_command}\t#{pane_current_path}";
    let raw = out(&["list-panes", "-a", "-F", fmt])?;
    let mut wins: Vec<Win> = Vec::new();
    for line in raw.lines() {
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() < 17 {
            continue;
        }
        let Some(pane) = parse_pane(&f[8..17]) else {
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
    Ok(wins)
}

/// Visible pane text plus a short scrollback tail. This is a snapshot, not a
/// live embed — tmux will not composite another pane into this one.
pub fn capture_pane(id: &str) -> io::Result<String> {
    out(&["capture-pane", "-ept", id, "-J", "-S", "-80"])
}

/// Strip CSI/OSC so a TUI snapshot is readable as plain text.
#[allow(dead_code)]
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
    use super::strip_ansi;

    #[test]
    fn strip_csi_and_cr() {
        assert_eq!(strip_ansi("\u{1b}[32mhello\u{1b}[0m\r\nworld"), "hello\nworld");
    }
}
