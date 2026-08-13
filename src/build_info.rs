//! Immutable build identity: what this binary is and where it came from.
//!
//! This module is the single owner of the version-plus-revision value and of
//! its canonical text. The command line and the popup chrome are consumers:
//! neither discovers build metadata nor formats it a second way.
//!
//! The version is the compiler's package version, which comes from
//! `Cargo.toml`. The revision is supplied at build time by the Nix build,
//! which reads it from the flake source metadata — no Git process, clock, or
//! network is involved. A build without that metadata reports
//! [`UNKNOWN_REVISION`] rather than inventing a commit.

/// Reported when the build supplied no source revision.
///
/// It is deliberately not hex-shaped, so it can never be mistaken for a
/// commit id.
pub const UNKNOWN_REVISION: &str = "unknown";

/// What this binary is: a product version and the source it was built from.
///
/// Both fields are compiled in and never change during process execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BuildIdentity {
    /// Product version, sourced from `Cargo.toml`.
    pub version: &'static str,
    /// Exact source revision, or [`UNKNOWN_REVISION`] when the build supplied
    /// none. A dirty source carries its commit with a `-dirty` suffix.
    pub revision: &'static str,
}

/// The identity compiled into this binary. Performs no I/O.
pub fn current() -> BuildIdentity {
    BuildIdentity {
        version: env!("CARGO_PKG_VERSION"),
        revision: choose_revision(option_env!("FACTORY_TUI_REVISION")),
    }
}

/// The canonical identity text shared by the command line and the popup.
pub fn display(identity: BuildIdentity) -> String {
    let BuildIdentity { version, revision } = identity;
    format!("factory-tui {version} (revision {revision})")
}

/// Chooses between build-supplied provenance and the honest fallback.
///
/// A build that supplied nothing — or an empty string — reports
/// [`UNKNOWN_REVISION`]. Nothing here guesses a commit.
fn choose_revision(supplied: Option<&'static str>) -> &'static str {
    match supplied {
        Some(revision) if !revision.is_empty() => revision,
        _ => UNKNOWN_REVISION,
    }
}

#[cfg(test)]
mod tests {
    use super::{choose_revision, current, display, BuildIdentity, UNKNOWN_REVISION};

    /// The version parsed straight out of the manifest, independently of the
    /// compiler's own `CARGO_PKG_VERSION` substitution.
    fn manifest_version() -> String {
        let manifest = include_str!("../Cargo.toml");
        let found = manifest
            .lines()
            .filter_map(|line| line.trim_end().strip_prefix("version"))
            .filter_map(|rest| rest.trim_start().strip_prefix('='))
            .map(|rest| rest.trim().trim_matches('"').to_string())
            .next()
            .expect("Cargo.toml declares a package version");
        // Positive control: a parser that silently matched nothing useful
        // must not be able to satisfy the comparison below.
        assert!(found.contains('.'), "parsed a non-version string: {found}");
        found
    }

    #[test]
    fn current_version_equals_the_manifest_version() {
        assert_eq!(current().version, manifest_version());
    }

    #[test]
    fn current_revision_is_never_empty() {
        assert!(!current().revision.is_empty());
    }

    #[test]
    fn display_carries_both_identity_fields() {
        let text = display(BuildIdentity { version: "1.2.3", revision: "d3adb33f" });
        assert!(text.contains("1.2.3"), "version missing from {text}");
        assert!(text.contains("d3adb33f"), "revision missing from {text}");
    }

    #[test]
    fn display_leads_with_the_product_and_version() {
        let text = display(BuildIdentity { version: "1.2.3", revision: "d3adb33f" });
        assert!(text.starts_with("factory-tui 1.2.3"), "unexpected shape: {text}");
    }

    #[test]
    fn supplied_revision_is_used_verbatim() {
        assert_eq!(choose_revision(Some("abc123")), "abc123");
    }

    #[test]
    fn absent_or_empty_build_metadata_falls_back_to_the_label() {
        assert_eq!(choose_revision(None), UNKNOWN_REVISION);
        assert_eq!(choose_revision(Some("")), UNKNOWN_REVISION);
    }

    #[test]
    fn the_fallback_can_never_be_read_as_a_commit() {
        let fallback = choose_revision(None);
        assert!(!fallback.is_empty(), "the fallback must still say something");
        assert!(
            !fallback.chars().all(|c| c.is_ascii_hexdigit()),
            "the fallback must not be hex-shaped: {fallback}"
        );
    }
}
