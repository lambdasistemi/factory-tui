//! Black-box proof that the version request is answered by the real binary
//! with no tmux server, no terminal, and no UI.

use std::process::Command;

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_factory-tui"))
        .args(args)
        .env_remove("TMUX")
        .env_remove("TMUX_PANE")
        .output()
        .expect("factory-tui runs")
}

#[test]
fn the_version_request_succeeds_outside_tmux() {
    let out = run(&["--version"]);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(out.status.success(), "exit {:?}, stderr: {stderr}", out.status.code());
    assert!(
        stdout.starts_with(&format!("factory-tui {}", env!("CARGO_PKG_VERSION"))),
        "unexpected stdout: {stdout}"
    );
    assert!(stdout.contains("revision"), "no provenance in stdout: {stdout}");
    assert!(stderr.is_empty(), "unexpected stderr: {stderr}");
}

/// Positive control for the harness: the same runner, with the same
/// environment, must be able to observe a refusal. Without it, a runner that
/// could not see failures would report the test above as a pass.
#[test]
fn without_a_version_request_the_binary_still_refuses_to_run_outside_tmux() {
    let out = run(&[]);
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(!out.status.success(), "expected a refusal outside tmux");
    assert!(stderr.contains("not inside tmux"), "unexpected stderr: {stderr}");
}
