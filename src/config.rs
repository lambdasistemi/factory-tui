//! Optional, host-local configuration for the raw tmux tree.

use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::ops::Range;
use std::path::{Path, PathBuf};

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
}

impl Config {
    /// Return a configuration whose tables have no effect on the tmux census.
    pub fn empty() -> Self {
        Self::default()
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
    toml::from_str(text).map_err(|source: toml::de::Error| ConfigError::Toml {
        path: None,
        span: source.span(),
        message: source.message().to_string(),
    })
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
