//! Window-name grammar used by the factory skills. Best-effort: unknown
//! names stay visible under unscoped rather than being dropped.

/// What kind of seat a window is trying to be.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Role {
    Machine,
    Crew,
    Orch,
    Desk,
    Epic,
    Ticket,
    Unknown,
}

/// Fields decoded from a window name plus the session that holds it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parsed {
    pub role: Role,
    pub project: Option<String>,
    pub milestone: Option<String>,
    pub epic: Option<String>,
    pub ticket: Option<String>,
    pub goal: String,
    pub parked: bool,
}

/// Decode a window name. `session` fills in a missing project / milestone.
pub fn parse_window(name: &str, session: &str) -> Parsed {
    let parked = name.to_ascii_uppercase().contains("PARKED");
    let mut parsed = parse_name(name);
    parsed.parked = parked || parsed.parked;

    if parsed.project.is_none() {
        parsed.project = project_from_session(session);
    } else if let Some(p) = parsed.project.take() {
        parsed.project = Some(canonicalize_project(&p, session));
    }

    if parsed.milestone.is_none() {
        parsed.milestone = milestone_from_session(session);
    }
    parsed
}

fn parse_name(name: &str) -> Parsed {
    match name {
        "machine" => {
            return blank(Role::Machine, "machine");
        }
        "machine-crew" | "machine-bootstrap" | "machine-orphans" => {
            return blank(Role::Crew, name);
        }
        "orch" => return blank(Role::Orch, "orch"),
        _ => {}
    }

    if let Some(p) = parse_ms(name) {
        return p;
    }
    if let Some(p) = parse_no_epic(name) {
        return p;
    }
    if let Some(p) = parse_epic(name) {
        return p;
    }
    blank(Role::Unknown, name)
}

fn blank(role: Role, goal: &str) -> Parsed {
    Parsed {
        role,
        project: None,
        milestone: None,
        epic: None,
        ticket: None,
        goal: goal.to_string(),
        parked: false,
    }
}

fn parse_ms(name: &str) -> Option<Parsed> {
    let (prefix, rest) = split_ms(name)?;
    let (ms, after) = take_digits(rest)?;
    let after = after.strip_prefix('-').unwrap_or(after);
    let project = nonempty_prefix(prefix);

    if let Some(trest) = after.strip_prefix('t') {
        if let Some(rest) = trest.strip_prefix('-') {
            return Some(Parsed {
                role: Role::Ticket,
                project,
                milestone: Some(ms),
                epic: None,
                ticket: Some(rest.to_string()),
                goal: rest.to_string(),
                parked: false,
            });
        }
        if let Some((tid, goal)) = take_digits(trest) {
            return Some(Parsed {
                role: Role::Ticket,
                project,
                milestone: Some(ms),
                epic: None,
                ticket: Some(tid),
                goal: strip_dash(goal).to_string(),
                parked: false,
            });
        }
    }

    if let Some(erest) = after.strip_prefix("e-") {
        let (epic, goal) = split_once_dash(erest);
        return Some(Parsed {
            role: Role::Epic,
            project,
            milestone: Some(ms),
            epic: Some(epic.to_string()),
            ticket: None,
            goal: goal.unwrap_or(epic).to_string(),
            parked: false,
        });
    }

    Some(Parsed {
        role: Role::Desk,
        project,
        milestone: Some(ms),
        epic: None,
        ticket: None,
        goal: after.to_string(),
        parked: false,
    })
}

fn parse_no_epic(name: &str) -> Option<Parsed> {
    let idx = name.find("-no-epic-t")?;
    let prefix = &name[..idx];
    let rest = &name[idx + "-no-epic-t".len()..];
    let project = nonempty_prefix(prefix);
    if let Some(rest) = rest.strip_prefix('-') {
        return Some(Parsed {
            role: Role::Ticket,
            project,
            milestone: None,
            epic: None,
            ticket: Some(rest.to_string()),
            goal: rest.to_string(),
            parked: false,
        });
    }
    if let Some((tid, goal)) = take_digits(rest) {
        return Some(Parsed {
            role: Role::Ticket,
            project,
            milestone: None,
            epic: None,
            ticket: Some(tid),
            goal: strip_dash(goal).to_string(),
            parked: false,
        });
    }
    None
}

fn parse_epic(name: &str) -> Option<Parsed> {
    let idx = find_epic_marker(name)?;
    let prefix = &name[..idx];
    let rest = &name[idx + 2..]; // skip "-e"
    let project = nonempty_prefix(prefix);

    if let Some(rest) = rest.strip_prefix('-') {
        // e-rfq-t-research-usdm-otc
        if let Some(tidx) = rest.find("-t-") {
            let epic = &rest[..tidx];
            let goal = &rest[tidx + 3..];
            return Some(Parsed {
                role: Role::Ticket,
                project,
                milestone: None,
                epic: Some(epic.to_string()),
                ticket: Some(goal.to_string()),
                goal: goal.to_string(),
                parked: false,
            });
        }
        return Some(Parsed {
            role: Role::Epic,
            project,
            milestone: None,
            epic: Some(rest.to_string()),
            ticket: None,
            goal: rest.to_string(),
            parked: false,
        });
    }

    let (epic, after) = take_digits(rest)?;
    let after = after.strip_prefix('-').unwrap_or(after);
    if let Some(trest) = after.strip_prefix('t') {
        if let Some(rest) = trest.strip_prefix('-') {
            return Some(Parsed {
                role: Role::Ticket,
                project,
                milestone: None,
                epic: Some(epic),
                ticket: Some(rest.to_string()),
                goal: rest.to_string(),
                parked: false,
            });
        }
        if let Some((tid, goal)) = take_digits(trest) {
            return Some(Parsed {
                role: Role::Ticket,
                project,
                milestone: None,
                epic: Some(epic),
                ticket: Some(tid),
                goal: strip_dash(goal).to_string(),
                parked: false,
            });
        }
    }
    Some(Parsed {
        role: Role::Epic,
        project,
        milestone: None,
        epic: Some(epic),
        ticket: None,
        goal: after.to_string(),
        parked: false,
    })
}

fn split_ms(name: &str) -> Option<(&str, &str)> {
    if let Some(rest) = name.strip_prefix("ms") {
        if rest.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            return Some(("", rest));
        }
    }
    let idx = name.rfind("-ms")?;
    let rest = &name[idx + 3..];
    if rest.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        Some((&name[..idx], rest))
    } else {
        None
    }
}

fn find_epic_marker(name: &str) -> Option<usize> {
    let bytes = name.as_bytes();
    for i in 0..bytes.len().saturating_sub(2) {
        if bytes[i] == b'-' && bytes[i + 1] == b'e' {
            let next = bytes.get(i + 2).copied();
            if next.is_some_and(|c| c == b'-' || c.is_ascii_digit()) {
                return Some(i);
            }
        }
    }
    None
}

fn take_digits(s: &str) -> Option<(String, &str)> {
    let n = s.chars().take_while(|c| c.is_ascii_digit()).count();
    if n == 0 {
        return None;
    }
    Some((s[..n].to_string(), &s[n..]))
}

fn strip_dash(s: &str) -> &str {
    s.strip_prefix('-').unwrap_or(s)
}

fn split_once_dash(s: &str) -> (&str, Option<&str>) {
    match s.split_once('-') {
        Some((a, b)) => (a, Some(b)),
        None => (s, None),
    }
}

fn nonempty_prefix(prefix: &str) -> Option<String> {
    if prefix.is_empty() {
        None
    } else {
        Some(prefix.to_string())
    }
}

fn project_from_session(session: &str) -> Option<String> {
    Some(
        match session {
            "0-machine" | "0-probe" | "0-projects" | "warden" | "grok-seat" => return None,
            "keri" | "keri-ms8-blaster" => "cardano-keri",
            "csk" => "cardano-swiss-knife",
            "wallet" | "cw" => "cardano-wallet",
            "treasury-ms1" => "amaru-treasury-tx",
            "amaru" => "amaru",
            "trenitalia" => "trenitalia-maps",
            "cip113" => "cip113",
            "cna-214" => "cardano-node-antithesis",
            other => other,
        }
        .to_string(),
    )
}

fn milestone_from_session(session: &str) -> Option<String> {
    if let Some(rest) = session.split("-ms").nth(1) {
        let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        if !digits.is_empty() {
            return Some(digits);
        }
    }
    None
}

fn canonicalize_project(raw: &str, session: &str) -> String {
    match raw {
        "cna" => "cardano-node-antithesis".into(),
        other => {
            if let Some(from_sess) = project_from_session(session) {
                if other == session {
                    return from_sess;
                }
            }
            other.to_string()
        }
    }
}

/// Sessions that are host infrastructure, not a product.
pub fn is_infra_session(session: &str) -> bool {
    session.starts_with("0-") || matches!(session, "warden" | "grok-seat" | "project-role")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, session: &str) -> Parsed {
        parse_window(name, session)
    }

    #[test]
    fn live_keri_names() {
        let d = p("cardano-keri-ms1-identity-core", "keri");
        assert_eq!(d.role, Role::Desk);
        assert_eq!(d.project.as_deref(), Some("cardano-keri"));
        assert_eq!(d.milestone.as_deref(), Some("1"));
        assert_eq!(d.goal, "identity-core");

        let e = p("cardano-keri-e274-onchain", "keri");
        assert_eq!(e.role, Role::Epic);
        assert_eq!(e.epic.as_deref(), Some("274"));

        let t = p("cardano-keri-e274-t271-payee-auth", "keri");
        assert_eq!(t.role, Role::Ticket);
        assert_eq!(t.epic.as_deref(), Some("274"));
        assert_eq!(t.ticket.as_deref(), Some("271"));

        let parked = p("cardano-keri-e156-t220-PARKED", "keri");
        assert!(parked.parked);
        assert_eq!(parked.ticket.as_deref(), Some("220"));
    }

    #[test]
    fn live_ms8_and_standalone() {
        let d = p("cardano-keri-ms8-blaster", "keri-ms8-blaster");
        assert_eq!(d.role, Role::Desk);
        assert_eq!(d.milestone.as_deref(), Some("8"));

        let t = p("cardano-keri-ms8-t-blaster-aiken-skill", "keri-ms8-blaster");
        assert_eq!(t.role, Role::Ticket);
        assert_eq!(t.milestone.as_deref(), Some("8"));

        let s = p("cardano-swiss-knife-ms1-t129-release-pipeline", "csk");
        assert_eq!(s.role, Role::Ticket);
        assert_eq!(s.ticket.as_deref(), Some("129"));
        assert_eq!(s.milestone.as_deref(), Some("1"));
    }

    #[test]
    fn live_oddities() {
        let a = p("ms2-amaru-routine", "amaru");
        assert_eq!(a.role, Role::Desk);
        assert_eq!(a.milestone.as_deref(), Some("2"));
        assert_eq!(a.project.as_deref(), Some("amaru"));

        let n = p("cip113-programmable-tokens-no-epic-t-unknown-pr101-review", "cip113");
        assert_eq!(n.role, Role::Ticket);
        assert_eq!(n.project.as_deref(), Some("cip113-programmable-tokens"));

        let rfq = p("amaru-treasury-tx-e-rfq-t-research-usdm-otc", "treasury-ms1");
        assert_eq!(rfq.role, Role::Ticket);
        assert_eq!(rfq.epic.as_deref(), Some("rfq"));

        let unk = p("trenitalia-maps-ms1-e-unknown-p2p-caching", "trenitalia");
        assert_eq!(unk.role, Role::Epic);
        assert_eq!(unk.milestone.as_deref(), Some("1"));
        assert_eq!(unk.epic.as_deref(), Some("unknown"));
    }
}
