//! Optional, host-local configuration for the raw tmux tree.

use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::ops::Range;
use std::path::{Path, PathBuf};

use regex::Regex;
use serde::Deserialize;

const CONFIG_ENV: &str = "FACTORY_TUI_CONFIG";

/// All configuration currently understood by `factory-tui`.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
#[serde(default)]
pub struct Config {
    /// Session aliases, infrastructure patterns, and future transport buckets.
    pub sessions: SessionsConfig,
    /// Pane-command rules used to classify window status.
    pub status: StatusConfig,
    /// Ordered window classifiers. First match wins.
    pub rule: Vec<Rule>,
    /// How classified windows are folded into folders.
    pub tree: TreeConfig,
}

/// One `[[rule]]` table: a window regex plus optional session regex and role.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
#[serde(default)]
pub struct Rule {
    /// Regex matched against the tmux window name. Named captures become fields.
    pub window: String,
    /// Optional regex matched against the tmux session name.
    pub session: Option<String>,
    /// Role assigned when this rule matches (`desk`, `ticket`, …).
    pub role: Option<String>,
}

/// The `[tree]` table.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
#[serde(default)]
pub struct TreeConfig {
    /// Ordered field names used as folder levels (`project`, `milestone`, `epic`).
    pub folders: Vec<String>,
    /// Roles whose window is the jump target of its folder.
    pub desk_roles: Vec<String>,
    /// Copy a desk's milestone onto other windows in the same session.
    pub inherit_milestone_from_desk: bool,
}

/// Fields decoded from the first matching rule.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Classified {
    /// Rule or capture `role`.
    pub role: Option<String>,
    /// Capture `project`, else session alias / session name.
    pub project: Option<String>,
    /// Capture `milestone`.
    pub milestone: Option<String>,
    /// Capture `epic`.
    pub epic: Option<String>,
    /// Capture `ticket`.
    pub ticket: Option<String>,
    /// Capture `goal`.
    pub goal: Option<String>,
    /// Window name contains the parked substring.
    pub parked: bool,
}

impl Config {
    /// Return a configuration whose tables have no effect on the tmux census.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Whether this file asks for a folded tree.
    pub fn projects(&self) -> bool {
        !self.rule.is_empty() && !self.tree.folders.is_empty()
    }

    /// Return the display alias for `session`, when one is configured.
    pub fn session_alias<'a>(&'a self, session: &str) -> Option<&'a str> {
        self.sessions.alias.get(session).map(String::as_str)
    }

    /// Whether `session` matches an infrastructure pattern.
    pub fn is_infra(&self, session: &str) -> bool {
        self.sessions.infra.iter().any(|pattern| glob_matches(pattern, session))
    }
}

/// Configuration under the `[sessions]` table.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
#[serde(default)]
pub struct SessionsConfig {
    /// Raw tmux session name to displayed session name.
    pub alias: BTreeMap<String, String>,
    /// Literal or simple-glob (`*` and `?`) patterns tagged as infrastructure.
    pub infra: Vec<String>,
    /// Future transport buckets retained for the projection ticket.
    pub machine: Vec<String>,
}

/// Configuration under the `[status]` table.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
#[serde(default)]
pub struct StatusConfig {
    /// Pane command names classified as running.
    pub running: Vec<String>,
    /// Pane command names classified as idle.
    pub idle: Vec<String>,
    /// Non-empty substring in a pane command classified as parked.
    pub parked_substring: String,
}

/// A configuration file could not be read or parsed.
#[derive(Debug)]
pub enum ConfigError {
    /// An existing configuration path could not be read.
    Io {
        /// The path that failed.
        path: PathBuf,
        /// The underlying filesystem error.
        source: io::Error,
    },
    /// TOML text did not match the supported schema.
    Toml {
        /// The source path, when parsing a file rather than an in-memory string.
        path: Option<PathBuf>,
        /// Byte span reported by the TOML parser, when available.
        span: Option<Range<usize>>,
        /// Short parser diagnostic.
        message: String,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => {
                write!(formatter, "cannot read config {}: {source}", path.display())
            }
            Self::Toml { path, span, message } => {
                write!(formatter, "invalid config")?;
                if let Some(path) = path {
                    write!(formatter, " {}", path.display())?;
                }
                if let Some(span) = span {
                    write!(formatter, " at bytes {}..{}", span.start, span.end)?;
                }
                write!(formatter, ": {message}")
            }
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Toml { .. } => None,
        }
    }
}

/// Load the selected config, exiting with a diagnostic if an existing file is invalid.
pub fn load() -> Config {
    let Some(path) = selected_path() else {
        return Config::empty();
    };
    let loaded = match path.try_exists() {
        Ok(false) => return Config::empty(),
        Ok(true) => load_from_path(&path),
        Err(source) => Err(ConfigError::Io { path: path.clone(), source }),
    };
    match loaded {
        Ok(config) => config,
        Err(error) => {
            eprintln!("factory-tui: {error}");
            std::process::exit(2);
        }
    }
}

/// Read and parse one explicit configuration path.
pub fn load_from_path(path: &Path) -> Result<Config, ConfigError> {
    let text = fs::read_to_string(path)
        .map_err(|source| ConfigError::Io { path: path.to_path_buf(), source })?;
    load_from_str(&text).map_err(|error| match error {
        ConfigError::Toml { span, message, .. } => {
            ConfigError::Toml { path: Some(path.to_path_buf()), span, message }
        }
        other => other,
    })
}

/// Parse TOML configuration from an in-memory string.
pub fn load_from_str(text: &str) -> Result<Config, ConfigError> {
    let config: Config = toml::from_str(text).map_err(|source: toml::de::Error| {
        ConfigError::Toml { path: None, span: source.span(), message: source.message().to_string() }
    })?;
    validate_rules(&config)?;
    Ok(config)
}

fn validate_rules(config: &Config) -> Result<(), ConfigError> {
    for (index, rule) in config.rule.iter().enumerate() {
        if rule.window.is_empty() {
            return Err(ConfigError::Toml {
                path: None,
                span: None,
                message: format!("rule {index} is missing window"),
            });
        }
        Regex::new(&rule.window).map_err(|error| ConfigError::Toml {
            path: None,
            span: None,
            message: format!("rule {index} window: {error}"),
        })?;
        if let Some(session) = &rule.session {
            Regex::new(session).map_err(|error| ConfigError::Toml {
                path: None,
                span: None,
                message: format!("rule {index} session: {error}"),
            })?;
        }
    }
    Ok(())
}

/// Classify `win` with the first matching rule. `None` means unmatched.
pub fn classify(win: &crate::tmux::Win, config: &Config) -> Option<Classified> {
    for rule in &config.rule {
        if let Some(session) = &rule.session {
            let Ok(regex) = Regex::new(session) else {
                continue;
            };
            if !regex.is_match(&win.session) {
                continue;
            }
        }
        let Ok(regex) = Regex::new(&rule.window) else {
            continue;
        };
        let Some(captures) = regex.captures(&win.name) else {
            continue;
        };
        let mut classified = Classified {
            role: named(&captures, "role").or_else(|| rule.role.clone()),
            project: named(&captures, "project"),
            milestone: named(&captures, "milestone"),
            epic: named(&captures, "epic"),
            ticket: named(&captures, "ticket"),
            goal: named(&captures, "goal"),
            parked: parked_name(&win.name, config),
        };
        if classified.project.is_none() {
            classified.project = config
                .session_alias(&win.session)
                .map(str::to_string)
                .or_else(|| Some(win.session.clone()));
        }
        if classified.milestone.is_none() {
            classified.milestone = milestone_from_session(&win.session);
        }
        return Some(classified);
    }
    None
}

fn named(captures: &regex::Captures<'_>, name: &str) -> Option<String> {
    captures.name(name).map(|value| value.as_str().to_string()).filter(|value| !value.is_empty())
}

fn parked_name(name: &str, config: &Config) -> bool {
    !config.status.parked_substring.is_empty() && name.contains(&config.status.parked_substring)
}

fn milestone_from_session(session: &str) -> Option<String> {
    let rest = session.split("-ms").nth(1)?;
    let digits: String = rest.chars().take_while(|character| character.is_ascii_digit()).collect();
    if digits.is_empty() {
        None
    } else {
        Some(digits)
    }
}

fn selected_path() -> Option<PathBuf> {
    if let Some(path) = nonempty_env(CONFIG_ENV) {
        return Some(PathBuf::from(path));
    }
    if let Some(root) = nonempty_env("XDG_CONFIG_HOME") {
        return Some(PathBuf::from(root).join("factory-tui/config.toml"));
    }
    nonempty_env("HOME").map(PathBuf::from).map(|home| home.join(".config/factory-tui/config.toml"))
}

fn nonempty_env(name: &str) -> Option<std::ffi::OsString> {
    env::var_os(name).filter(|value| !value.is_empty())
}

fn glob_matches(pattern: &str, value: &str) -> bool {
    let pattern: Vec<char> = pattern.chars().collect();
    let value: Vec<char> = value.chars().collect();
    let mut matched = vec![vec![false; value.len() + 1]; pattern.len() + 1];
    matched[0][0] = true;

    for pattern_index in 1..=pattern.len() {
        if pattern[pattern_index - 1] == '*' {
            matched[pattern_index][0] = matched[pattern_index - 1][0];
        }
        for value_index in 1..=value.len() {
            matched[pattern_index][value_index] = match pattern[pattern_index - 1] {
                '*' => {
                    matched[pattern_index - 1][value_index]
                        || matched[pattern_index][value_index - 1]
                }
                '?' => matched[pattern_index - 1][value_index - 1],
                character => {
                    character == value[value_index - 1]
                        && matched[pattern_index - 1][value_index - 1]
                }
            };
        }
    }

    matched[pattern.len()][value.len()]
}

#[cfg(test)]
mod tests {
    use super::glob_matches;

    #[test]
    fn simple_globs_match_whole_session_names() {
        assert!(glob_matches("ops-*", "ops-cache"));
        assert!(glob_matches("node-?", "node-a"));
        assert!(glob_matches("literal", "literal"));
        assert!(!glob_matches("ops-*", "shop-ops-cache"));
        assert!(!glob_matches("node-?", "node-ab"));
    }
}
