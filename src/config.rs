//! Optional, host-local configuration for the raw tmux tree.

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

/// All configuration currently understood by `factory-tui`. None of it can
/// change the shape of the tree: the tmux census decides that alone, and
/// everything here supplies display strings and status vocabulary.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
#[serde(default)]
pub struct Config {
    /// Infrastructure patterns.
    pub sessions: SessionsConfig,
    /// Pane-command rules used to classify window status.
    pub status: StatusConfig,
    /// Ordered label-only rules. First match in a scope wins.
    pub reinterpreter: Vec<Reinterpreter>,
}

/// Which kind of row a reinterpreter may relabel. Closed on purpose: an
/// unrecognised scope is a configuration error, not a rule that never fires.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Scope {
    /// A tmux session row.
    Session,
    /// A tmux window row.
    Window,
    /// A tmux pane row.
    Pane,
}

/// One `[[reinterpreter]]` table: a scope, a regex, and the text that replaces
/// what the regex matched. Nothing here reaches the tree — it rewrites one
/// row's displayed text and can do nothing else, because the tree it would
/// have to reach already exists by the time any of this is read.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Reinterpreter {
    /// Which kind of row this entry may relabel.
    pub scope: Scope,
    /// Regex matched against the raw tmux name.
    pub pattern: String,
    /// Replacement for the matched span. `$name` refers to a named capture.
    pub label: String,
}

impl Config {
    /// Return a configuration whose tables have no effect on the tmux census.
    pub fn empty() -> Self {
        Self::default()
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
    /// Literal or simple-glob (`*` and `?`) patterns tagged as infrastructure.
    pub infra: Vec<String>,
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
    validate_reinterpreters(&config)?;
    Ok(config)
}

fn validate_reinterpreters(config: &Config) -> Result<(), ConfigError> {
    let invalid = |index: usize, what: &str| ConfigError::Toml {
        path: None,
        span: None,
        message: format!("reinterpreter {index} {what}"),
    };
    for (index, entry) in config.reinterpreter.iter().enumerate() {
        if entry.pattern.is_empty() {
            return Err(invalid(index, "has an empty pattern"));
        }
        // A label that paints nothing is caught here rather than at render
        // time, where the row would silently fall back and look unmatched.
        if entry.label.trim().is_empty() {
            return Err(invalid(index, "has a label that displays nothing"));
        }
        Regex::new(&entry.pattern).map_err(|error| invalid(index, &format!("pattern: {error}")))?;
    }
    Ok(())
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
    use std::fs;
    use std::path::Path;

    use super::{glob_matches, load_from_path, load_from_str, Config, Scope};
    use crate::label::reinterpret;

    /// The example this repository ships and publishes, read from the working
    /// tree. A published example that no longer exists, or that the real
    /// parser no longer accepts, fails here rather than on an operator's host.
    fn shipped_example() -> String {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/config.toml");
        fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("shipped example {}: {error}", path.display()))
    }

    /// A line of the shipped example, and what a seeded break replaces it
    /// with. Each case checks that its own substitution applied: a break that
    /// silently failed to match would report "rejected" while testing nothing.
    fn seeded_breaks(example: &str) -> Vec<(&'static str, String)> {
        let cases = [
            (
                "sampler field outside the supported set",
                "field = \"pane_title\"",
                "field = \"pane_mood\"",
            ),
            (
                "sampler status outside the closed set",
                "status = \"running\"",
                "status = \"paused\"",
            ),
            (
                "sampler regex that does not compile",
                "regex = \"^[\\u2800-\\u28ff]\"",
                "regex = \"[\"",
            ),
            ("scope outside the closed set", "scope = \"session\"", "scope = \"galaxy\""),
            (
                "pattern that is not a regex",
                "pattern = \"^ops-(?P<rest>.+)$\"",
                "pattern = \"^ops-(?P<rest>.+$\"",
            ),
            ("label that displays nothing", "label = \"operations $rest\"", "label = \"\""),
            ("entry with no pattern at all", "pattern = \"^ops-(?P<rest>.+)$\"\n", ""),
        ];
        cases
            .into_iter()
            .map(|(name, from, to)| {
                assert!(example.contains(from), "{name}: anchor {from:?} is not in the example");
                let broken = example.replacen(from, to, 1);
                assert_ne!(broken, example, "{name}: the seeded break did not apply");
                (name, broken)
            })
            .collect()
    }

    #[test]
    fn shipped_config_parses() {
        let example = shipped_example();
        assert!(example.contains("[[sampler]]"), "the shipped example has no sampler table");
        let config = load_from_str(&example).expect("the shipped example parses");

        // Parsing alone proves little: a schema that ignores what it does not
        // recognise accepts an example made entirely of typos. Require the
        // example to be live through the real display path instead.
        for (scope, raw, expected) in [
            (Scope::Session, "ops-cache", "operations cache"),
            (Scope::Window, "api-deploy-staging", "api to staging"),
            (Scope::Pane, "0:bash", "pane 0 running bash"),
        ] {
            let shown = reinterpret(scope, raw, &config);
            assert_ne!(shown, raw, "the {scope:?} example rule never fires on {raw:?}");
            assert!(shown.contains(expected), "{scope:?}: {shown:?} lacks {expected:?}");
            assert!(shown.contains(raw), "{scope:?}: {shown:?} hides its raw source {raw:?}");
        }

        // The tables the example also publishes must survive the same parse.
        assert!(config.is_infra("ops-cache"), "the example's infra glob was dropped");
        assert!(!config.is_infra("work"));
        let retained = format!("{config:?}");
        assert!(retained.contains("busy-title"), "the example sampler was ignored: {retained}");
        assert!(retained.contains("pane_title"), "the example field was ignored: {retained}");
    }

    #[test]
    fn shipped_config_rejects_seeded_break() {
        let example = shipped_example();
        // Positive control: the instrument is pointed at something it accepts,
        // so a rejection below is about the break and not about the method.
        load_from_str(&example).expect("the unbroken example parses");

        for (name, broken) in seeded_breaks(&example) {
            let error = load_from_str(&broken)
                .err()
                .unwrap_or_else(|| panic!("{name}: the broken example was accepted"));
            assert!(!error.to_string().is_empty(), "{name}: rejected with no diagnostic");
        }
    }

    #[test]
    fn empty_and_devnull_paths_load_a_neutral_config() {
        // The immutable gate reads the live tree with FACTORY_TUI_CONFIG
        // pointed at /dev/null. That only means "raw tmux" if an empty file
        // loads as a configuration which changes nothing.
        assert_eq!(load_from_str("").expect("empty text parses"), Config::empty());
        let dev_null = Path::new("/dev/null");
        if dev_null.exists() {
            assert_eq!(load_from_path(dev_null).expect("/dev/null parses"), Config::empty());
        }
    }

    #[test]
    fn simple_globs_match_whole_session_names() {
        assert!(glob_matches("ops-*", "ops-cache"));
        assert!(glob_matches("node-?", "node-a"));
        assert!(glob_matches("literal", "literal"));
        assert!(!glob_matches("ops-*", "shop-ops-cache"));
        assert!(!glob_matches("node-?", "node-ab"));
    }

    #[test]
    fn sampler_validation_rejects_every_invalid_shape_and_the_removed_table() {
        let cases = [
            (
                "unsupported field",
                "[[sampler]]\nname = \"bad-field\"\nfield = \"pane_mood\"\nregex = \".*\"\nstatus = \"running\"\n",
                "pane_mood",
            ),
            (
                "unsupported status",
                "[[sampler]]\nname = \"bad-status\"\nfield = \"pane_title\"\nregex = \".*\"\nstatus = \"paused\"\n",
                "bad-status",
            ),
            (
                "invalid regex",
                "[[sampler]]\nname = \"bad-regex\"\nfield = \"pane_title\"\nregex = \"[\"\nstatus = \"idle\"\n",
                "bad-regex",
            ),
            (
                "empty name",
                "[[sampler]]\nname = \" \"\nfield = \"pane_title\"\nregex = \".*\"\nstatus = \"idle\"\n",
                "name",
            ),
            (
                "duplicate name",
                "[[sampler]]\nname = \"same\"\nfield = \"pane_title\"\nregex = \"one\"\nstatus = \"idle\"\n\n[[sampler]]\nname = \"same\"\nfield = \"pane_title\"\nregex = \"two\"\nstatus = \"running\"\n",
                "same",
            ),
            (
                "removed status table",
                "[status]\nrunning = [\"occupied\"]\n",
                "status",
            ),
        ];

        for (case, source, diagnostic) in cases {
            let error = load_from_str(source)
                .err()
                .unwrap_or_else(|| panic!("{case}: invalid configuration was accepted"));
            assert!(
                error.to_string().contains(diagnostic),
                "{case}: {error} does not name {diagnostic:?}"
            );
        }
    }

    #[test]
    fn c3_published_schema_names_the_real_sampler_contract() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let schema = fs::read_to_string(root.join("skills/factory-tui/references/config.md"))
            .expect("published schema is present");
        let config_source = include_str!("config.rs");

        assert!(schema.contains("[[sampler]]"), "published schema omits the sampler table");
        assert!(
            config_source.contains("pub const SUPPORTED_SAMPLER_FIELDS"),
            "the real Config module has no supported-field declaration"
        );
        for field in ["pane_current_command", "pane_current_path", "pane_title", "window_name"] {
            assert!(schema.contains(field), "published schema omits {field}");
            assert!(config_source.contains(&format!("\"{field}\"")), "Config omits {field}");

            let source = format!(
                "[[sampler]]\nname = \"schema-{field}\"\nfield = \"{field}\"\nregex = \".*\"\nstatus = \"idle\"\n"
            );
            let parsed = load_from_str(&source).expect("schema-shaped sampler parses");
            let retained = format!("{parsed:?}");
            assert!(retained.contains(field), "real Config ignored schema field {field}");
        }
    }
}
