//! factory-tui — a tree browser over live tmux.
//!
//! The tree is tmux itself: session → window → pane. An optional config file
//! may rewrite what a row *says*; nothing may change what the tree *is*.
//! Right: a text snapshot of the selected pane (`tmux capture-pane`).
//! Enter jumps the attached client there and exits.
//!
//! `--dump` prints the tree and quits (no TUI).
//! `--version` prints the build identity and quits (no TUI, no tmux).

mod ansi;
mod app;
mod build_info;
mod config;
mod geometry;
mod label;
mod peek;
mod tmux;
mod tree;
mod ui;

use std::env;
use std::io::{self, stdout, Stdout};
use std::time::Duration;

use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::Terminal;

use app::App;

/// True only for the standalone version request. Reads arguments only: it
/// never consults tmux or the terminal.
fn is_version_request(args: &[String]) -> bool {
    args.len() == 1 && args[0] == "--version"
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if is_version_request(&args) {
        println!("{}", build_info::display(build_info::current()));
        return Ok(());
    }

    if env::args().any(|a| a == "--dump") {
        let config = config::load();
        let wins = tmux::query_all()?;
        let root = tree::build(wins, &config.sampler);
        print!("{}", tree::dump(&root, &config));
        return Ok(());
    }

    if env::var_os("TMUX").is_none() {
        eprintln!("factory-tui: not inside tmux ($TMUX unset). Use --dump or run from tmux.");
        std::process::exit(1);
    }

    let config = config::load();
    enable_raw_mode()?;
    let mut out = stdout();
    execute!(out, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(out);
    let mut terminal = Terminal::new(backend)?;

    let res = run(&mut terminal, config);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    res
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    config: config::Config,
) -> io::Result<()> {
    let mut app = App::new(config)?;
    loop {
        terminal.draw(|f| app.draw(f))?;
        if event::poll(Duration::from_millis(800))? {
            match event::read()? {
                Event::Key(k) => app.on_key(k)?,
                Event::Mouse(m) => app.on_mouse(m)?,
                _ => {}
            }
        } else {
            app.refresh_preview();
        }
        if app.should_quit {
            return Ok(());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::is_version_request;

    fn args(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| (*s).to_string()).collect()
    }

    #[test]
    fn the_standalone_version_flag_is_a_version_request() {
        assert!(is_version_request(&args(&["--version"])));
    }

    #[test]
    fn no_arguments_is_not_a_version_request() {
        assert!(!is_version_request(&args(&[])));
    }

    #[test]
    fn other_commands_are_not_version_requests() {
        assert!(!is_version_request(&args(&["--dump"])));
        assert!(!is_version_request(&args(&["-V"])));
        assert!(!is_version_request(&args(&["version"])));
        assert!(!is_version_request(&args(&["--versions"])));
    }

    #[test]
    fn a_version_flag_among_other_arguments_is_not_the_supported_request() {
        assert!(!is_version_request(&args(&["--version", "--dump"])));
        assert!(!is_version_request(&args(&["--dump", "--version"])));
    }
}
