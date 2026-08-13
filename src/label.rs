//! Display labels: one string in, one string out. This module owns every
//! string a row is drawn from — #22's safe pane identity label and #26's
//! configured reinterpretation. Nothing reachable from here can create,
//! remove, reparent or reorder a node, which is what makes INV-26-C8 provable.

use ratatui::text::Span;
use regex::Regex;

use crate::config::{Config, Scope};
use crate::tmux::{self, Pane};
use crate::tree::{status_field, Kind, Node, Status};

/// Terminal cells a string occupies, measured the way ratatui places it.
pub fn cells(text: &str) -> usize {
    Span::raw(text).width()
}

/// Drop escape sequences first, then every remaining control character, then
/// collapse the whitespace that leaves behind. A pane title is written by
/// whatever runs in the pane, so raw it can recolour, erase, or — where a label
/// is drawn from a plain string — split the frame it lands in.
///
/// The control pass does not assume what `strip_ansi` chooses to keep, so it
/// is redundant with today's keep-list and no input can distinguish it — see
/// the equivalent mutant M07 in the campaign log. It stays as this function's
/// own stated contract, not as a check anything relies on.
pub fn sanitize_label(raw: &str) -> String {
    let stripped = tmux::strip_ansi(raw);
    let plain: String = stripped.chars().map(|c| if c.is_control() { ' ' } else { c }).collect();
    plain.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Cut to `max` terminal cells, marking the cut so a shortened title still
/// reads as one.
pub fn fit_cells(text: &str, max: u16) -> String {
    let max = max as usize;
    if max == 0 {
        return String::new();
    }
    if cells(text) <= max {
        return text.to_string();
    }
    let budget = max - 1;
    let mut out = String::new();
    let mut used = 0;
    let mut buf = [0u8; 4];
    for ch in text.chars() {
        let w = cells(ch.encode_utf8(&mut buf));
        if used + w > budget {
            break;
        }
        out.push(ch);
        used += w;
    }
    out.push('…');
    out
}

/// One identity rule for every pane label site: the operator-authored tmux
/// title when it survives sanitation, otherwise the `{index}:{command}`
/// identity this view has always used. Never blank while a cell is free, never
/// wider than `max_width` terminal cells.
pub fn pane_label(pane: &Pane, max_width: u16) -> String {
    // Emptiness that matters here is *visible* emptiness, not string
    // emptiness. A combining mark, a zero-width space or a lone variation
    // selector survives sanitation as a nonempty String and paints nothing, so
    // testing `is_empty()` hands a blank box to the reader and calls it an
    // identity.
    let title = sanitize_label(&pane.title);
    let text = if cells(&title) > 0 {
        title
    } else {
        // The fallback is visible whatever the index and command are: the
        // colon is neither control nor whitespace, so it survives sanitation
        // and pays for at least one cell. There is no third case to guard.
        sanitize_label(&format!("{}:{}", pane.index, pane.cmd))
    };
    fit_cells(&text, max_width)
}

/// Reinterpret one raw row name into the string that row displays.
///
/// The first entry of `scope` whose pattern matches wins. The matched span is
/// replaced by that entry's label, and the sanitized raw name is kept beside
/// it in parentheses, so a rewritten row can always be read back to the name
/// tmux actually uses and two rows that reinterpret alike stay distinct.
///
/// Anything that would leave the row blank — no match, a replacement that
/// paints nothing, a rewrite that says exactly what the raw name already
/// said — renders raw instead.
pub fn reinterpret(scope: Scope, raw_name: &str, config: &Config) -> String {
    let raw = sanitize_label(raw_name);
    for entry in config.reinterpreter.iter().filter(|entry| entry.scope == scope) {
        // A pattern that does not compile was already rejected at load time;
        // skipping it here keeps one bad entry from swallowing the rest.
        let Ok(regex) = Regex::new(&entry.pattern) else {
            continue;
        };
        if !regex.is_match(&raw) {
            continue;
        }
        let replaced = sanitize_label(&regex.replace(&raw, entry.label.as_str()));
        if cells(&replaced) == 0 || replaced == raw {
            return raw;
        }
        return format!("{replaced} ({raw})");
    }
    raw
}

/// The annotation a session row carries when it matches `[sessions] infra`.
pub const INFRA_FIELD: &str = "[infra]";

/// What the *renderer* says on a row, never the row itself: the two tmux
/// identity markers, the status column, and the infra annotation. Each entry is
/// an opening shape, so a forgery is caught whatever it puts inside, and the
/// status half is derived rather than listed so a new status cannot arrive
/// unreserved.
fn reserved_fields() -> Vec<String> {
    let mut fields = vec!["[window=".to_string(), "[pane=".to_string(), INFRA_FIELD.to_string()];
    fields.extend(Status::ALL.iter().map(|status| status_field(*status)).filter(|f| !f.is_empty()));
    fields
}

/// Neutralise anything in a display string that only the renderer may say.
/// Sanitation stops a label from corrupting the frame; this stops it from
/// *claiming* something — a tmux id, a liveness, an infrastructure role. The
/// forgery is defanged rather than deleted, so the reader still sees what the
/// row called itself, and a name merely containing the word `idle` is untouched.
pub fn defang_reserved(label: &str) -> String {
    let mut out = label.to_string();
    for field in reserved_fields() {
        if let Some(rest) = field.strip_prefix('[') {
            out = out.replace(&field, &format!("({rest}"));
        }
    }
    out
}

/// The display string for one node. The tree already exists by the time this
/// runs, and the result cannot travel back into identity or parentage. Every
/// consumer goes through here, so the reserved-field rule is applied once
/// rather than remembered at each render site.
pub fn node_label(node: &Node, config: &Config) -> String {
    let shown = match node.kind {
        Kind::SessionGroup if node.id != "root" => reinterpret(Scope::Session, &node.title, config),
        Kind::Window => reinterpret(Scope::Window, &node.title, config),
        Kind::Pane => reinterpret(Scope::Pane, &node.title, config),
        _ => node.title.clone(),
    };
    let shown = defang_reserved(&shown);

    // Infrastructure is decided by the raw session name, not by whatever a
    // rule renamed it to, and is added after defanging — which is what makes
    // the genuine annotation distinguishable from a row that writes it itself.
    if node.kind == Kind::SessionGroup && node.id != "root" && config.is_infra(&node.title) {
        format!("{shown} {INFRA_FIELD}")
    } else {
        shown
    }
}

#[cfg(test)]
mod tests {
    use super::{node_label, reinterpret};
    use crate::config::{self, Config, Scope};
    use crate::tree::{Kind, Node, Status};

    /// A title any process in a pane can set with one escape sequence.
    const HOSTILE: &str = "\u{1b}[31m\u{1b}[2Jred\u{1b}]0;owned\u{7}\r\nsecond\tline\u{8}\u{9b}5m";

    /// Text only the renderer may produce. Stated as literals on purpose: an
    /// expectation computed from the production reserved list would agree with
    /// it however wrong both were.
    const RESERVED_DISPLAY: [&str; 6] =
        ["[window=@9]", "[pane=%9]", "[RUNNING]", "[PARKED]", "[idle]", "[infra]"];

    /// What `HOSTILE` must look like once it is safe to draw. Stated as a
    /// literal on purpose: an expectation computed by the sanitizer under
    /// proof would agree with it whatever either of them did.
    const HOSTILE_SAFE: &str = "red second line5m";

    fn parse(source: &str) -> Config {
        config::load_from_str(source).expect("test configuration parses")
    }

    /// A node carrying one raw name and no status of its own, so that any
    /// status word on its rendered row can only have been forged.
    fn node(kind: Kind, id: &str, raw_name: &str) -> Node {
        Node {
            id: id.to_string(),
            kind,
            title: raw_name.to_string(),
            status: Status::Unknown,
            win: None,
            pane: None,
            children: Vec::new(),
        }
    }

    #[test]
    fn raw_name_is_retained_after_reinterpretation() {
        let config = parse(
            r#"
[[reinterpreter]]
scope = "window"
pattern = "^(?P<project>.+)-t(?P<ticket>[0-9]+)-(?P<goal>.+)$"
label = "t$ticket $goal"
"#,
        );

        // A matched row is rewritten, and its raw source stays on the row: two
        // rows that reinterpret to the same text must still be tellable apart,
        // and an operator must be able to read back what tmux calls this.
        let raw = "factory-tui-t26-raw-tree";
        let shown = reinterpret(Scope::Window, raw, &config);
        assert_ne!(shown, raw, "the configured rule never applied");
        assert!(shown.contains("t26 raw-tree"), "replacement missing from {shown:?}");
        assert!(shown.contains(raw), "raw source {raw:?} is not reachable from {shown:?}");
        assert_eq!(shown.lines().count(), 1, "a label is one line: {shown:?}");

        // Unmatched input renders raw, byte for byte.
        assert_eq!(reinterpret(Scope::Window, "notes", &config), "notes");

        // Scope is closed: a window rule may not relabel a session or a pane.
        assert_eq!(reinterpret(Scope::Session, raw, &config), raw);
        assert_eq!(reinterpret(Scope::Pane, raw, &config), raw);

        // Ordered, first match wins.
        let ordered = parse(
            r#"
[[reinterpreter]]
scope = "window"
pattern = "^notes$"
label = "first"

[[reinterpreter]]
scope = "window"
pattern = "^notes$"
label = "second"
"#,
        );
        let shown = reinterpret(Scope::Window, "notes", &ordered);
        assert!(shown.starts_with("first"), "a later entry won: {shown:?}");

        // The raw half is sanitized before it is shown. A pane names itself,
        // so the name a rewrite discloses is hostile input.
        let hostile_config = parse(
            r#"
[[reinterpreter]]
scope = "pane"
pattern = "^(?P<all>.+)$"
label = "seat"
"#,
        );
        let shown = reinterpret(Scope::Pane, HOSTILE, &hostile_config);
        assert!(shown.starts_with("seat"), "{shown:?}");
        assert!(shown.contains(HOSTILE_SAFE), "sanitized source missing from {shown:?}");
        assert!(!shown.contains('\u{1b}'), "{shown:?}");
        assert!(!shown.chars().any(char::is_control), "{shown:?}");
        assert_eq!(shown.lines().count(), 1, "{shown:?}");

        // An output that would paint nothing interprets nothing: the row falls
        // back to its raw name rather than going blank.
        let blank = parse(
            r#"
[[reinterpreter]]
scope = "window"
pattern = "^(?P<kept>.+)$"
label = "$absent"
"#,
        );
        assert_eq!(reinterpret(Scope::Window, "notes", &blank), "notes");

        // Sanitation and marker defanging stop a row from corrupting the frame
        // or forging a tmux id. They do not stop it from *claiming* something:
        // a window named "deploy PARKED", or a rule that relabels one, paints
        // the same word the status column paints. Every node below has status
        // Unknown, so any status text on its row is forged by construction.
        let kinds = [
            (Kind::SessionGroup, "session/x", "session"),
            (Kind::Window, "session/x/@1", "window"),
        ];
        for reserved in RESERVED_DISPLAY {
            for (kind, id, scope) in kinds {
                // arriving as a raw tmux name
                let shown =
                    node_label(&node(kind, id, &format!("seat {reserved} tail")), &Config::empty());
                assert!(
                    !shown.contains(reserved),
                    "{kind:?} raw name painted {reserved:?}: {shown:?}"
                );
                assert!(shown.contains("seat"), "{kind:?} lost its raw name: {shown:?}");

                // arriving as a configured replacement
                let config = parse(&format!(
                    "[[reinterpreter]]\nscope = \"{scope}\"\npattern = \"^plain$\"\nlabel = \"{reserved} x\"\n"
                ));
                let shown = node_label(&node(kind, id, "plain"), &config);
                assert!(
                    !shown.contains(reserved),
                    "{kind:?} label painted {reserved:?}: {shown:?}"
                );
                assert!(shown.contains("plain"), "{kind:?} lost its raw source: {shown:?}");
            }
        }

        // Positive control: the annotation the renderer *does* own must still
        // render, or the rule above would be satisfied by emitting nothing.
        let infra = parse("[sessions]\ninfra = [\"ops-*\"]\n");
        let shown = node_label(&node(Kind::SessionGroup, "session/ops-cache", "ops-cache"), &infra);
        assert!(shown.ends_with(" [infra]"), "the genuine annotation was lost: {shown:?}");

        // And a session that only claims to be infrastructure does not get it.
        let liar = node(Kind::SessionGroup, "session/x", "ops-cache [infra]");
        assert!(
            !node_label(&liar, &Config::empty()).contains("[infra]"),
            "unlisted session claimed infra"
        );
    }
}
