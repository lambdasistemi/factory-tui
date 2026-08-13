//! factory-tui — a factory-tree browser over live tmux.
//!
//! Left: machine → project → milestone → epic → ticket.
//! Right: a text snapshot of the selected pane (`tmux capture-pane`).
//! Enter jumps the attached client there and exits.
//!
//! `--dump` prints the tree and quits (no TUI).

mod ansi;
mod app;
mod geometry;
mod parse;
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

fn main() -> io::Result<()> {
    if env::args().any(|a| a == "--dump") {
        let wins = tmux::query_all()?;
        print!("{}", tree::dump(&tree::build(wins)));
        return Ok(());
    }

    if env::var_os("TMUX").is_none() {
        eprintln!("factory-tui: not inside tmux ($TMUX unset). Use --dump or run from tmux.");
        std::process::exit(1);
    }

    enable_raw_mode()?;
    let mut out = stdout();
    execute!(out, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(out);
    let mut terminal = Terminal::new(backend)?;

    let res = run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    res
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    let mut app = App::new()?;
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
