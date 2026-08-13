//! Cheap extra facts for the selected seat. Never block the tree on this.

use std::fs;
use std::path::Path;

use crate::tmux::Win;

/// A few lines of durable state, if any file is sitting next to the seat.
pub fn peek(win: &Win) -> Vec<String> {
    let mut lines = Vec::new();
    for pane in &win.panes {
        if let Some(found) = read_head(Path::new(&pane.path).join(".orch/window-brief.md")) {
            lines.push(format!("brief  {}", found));
            break;
        }
    }
    for pane in &win.panes {
        if let Some(found) = read_tail(Path::new(&pane.path).join("STATUS.md")) {
            lines.push(format!("STATUS {found}"));
            break;
        }
    }
    if lines.iter().all(|l| !l.starts_with("STATUS ")) {
        let ms = guess_ms(&win.name);
        let mut candidates =
            vec!["/tmp/machine/STATUS.md".to_string(), "/tmp/orch/STATUS.md".to_string()];
        if !ms.is_empty() {
            candidates.push(format!("/tmp/ms-{ms}/STATUS.md"));
        }
        for candidate in candidates {
            if let Some(found) = read_tail(Path::new(&candidate)) {
                lines.push(format!("STATUS {found}"));
                break;
            }
        }
    }
    lines
}

fn guess_ms(name: &str) -> String {
    if let Some(i) = name.find("-ms") {
        name[i + 3..].chars().take_while(|c| c.is_ascii_digit()).collect()
    } else if let Some(rest) = name.strip_prefix("ms") {
        rest.chars().take_while(|c| c.is_ascii_digit()).collect()
    } else {
        String::new()
    }
}

fn read_head(path: impl AsRef<Path>) -> Option<String> {
    let text = fs::read_to_string(path.as_ref()).ok()?;
    text.lines().map(str::trim).find(|l| !l.is_empty() && *l != "---").map(ToOwned::to_owned)
}

fn read_tail(path: impl AsRef<Path>) -> Option<String> {
    let text = fs::read_to_string(path.as_ref()).ok()?;
    text.lines().rev().map(str::trim).find(|l| !l.is_empty()).map(ToOwned::to_owned)
}
