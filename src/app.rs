//! Selection, expand/collapse, and the jump-to-seat action.

use std::collections::HashSet;
use std::io;
use std::time::{Duration, Instant};

use ratatui::crossterm::event::{KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

use crate::config::Config;
use crate::geometry::{contains, hit, rects_for};
use crate::peek;
use crate::tmux::{self, Win};
use crate::tree::{self, Kind, Node};

/// A visible tree row.
#[derive(Clone, Debug)]
pub struct Row {
    pub id: String,
    pub depth: usize,
    pub title: String,
    pub kind: Kind,
    pub status: tree::Status,
    pub has_children: bool,
    pub expanded: bool,
    pub window_count: usize,
}

pub struct App {
    config: Config,
    pub root: Node,
    pub expanded: HashSet<String>,
    pub selected: usize,
    pub scroll: usize,
    pub rows: Vec<Row>,
    pub status: String,
    pub peek: Vec<String>,
    pub preview: String,
    pub preview_pane: Option<String>,
    pub preview_from_bottom: usize,
    pub should_quit: bool,
    pub cursor: (u16, u16),
    pub tree_area: Rect,
    pub schematic_area: Rect,
    pub preview_area: Rect,
    last_wheel: Instant,
    last_click: Option<(Instant, ClickTarget)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ClickTarget {
    Tree(usize),
    Schematic(String),
    Preview,
}

impl App {
    pub fn new(config: Config) -> io::Result<Self> {
        let wins = tmux::query_all()?;
        let root = tree::build(wins, &config);
        let mut expanded = HashSet::new();
        expand_defaults(&root, &mut expanded);
        let mut app = Self {
            config,
            root,
            expanded,
            selected: 0,
            scroll: 0,
            rows: Vec::new(),
            status: "snapshot preview · Enter jumps · q quits".into(),
            peek: Vec::new(),
            preview: String::new(),
            preview_pane: None,
            preview_from_bottom: 0,
            should_quit: false,
            cursor: (0, 0),
            tree_area: Rect::default(),
            schematic_area: Rect::default(),
            preview_area: Rect::default(),
            last_wheel: Instant::now() - Duration::from_secs(1),
            last_click: None,
        };
        app.rebuild_rows();
        app.select_first_window();
        app.refresh_peek();
        Ok(app)
    }

    pub fn refresh(&mut self) -> io::Result<()> {
        let keep = self.current().map(|r| r.id.clone());
        self.root = tree::build(tmux::query_all()?, &self.config);
        expand_defaults(&self.root, &mut self.expanded);
        self.rebuild_rows();
        if let Some(id) = keep {
            if let Some(i) = self.rows.iter().position(|r| r.id == id) {
                self.selected = i;
            }
        }
        self.clamp();
        self.refresh_peek();
        self.status = "refreshed".into();
        Ok(())
    }

    fn rebuild_rows(&mut self) {
        self.rows.clear();
        flatten(&self.root, 0, &self.expanded, &mut self.rows);
        self.clamp();
    }

    fn clamp(&mut self) {
        if self.rows.is_empty() {
            self.selected = 0;
            return;
        }
        if self.selected >= self.rows.len() {
            self.selected = self.rows.len() - 1;
        }
    }

    fn select_first_window(&mut self) {
        if let Some(i) = self.rows.iter().position(|r| r.kind == Kind::Window) {
            self.selected = i;
        }
    }

    pub fn current(&self) -> Option<&Row> {
        self.rows.get(self.selected)
    }

    pub fn selected_node(&self) -> Option<&Node> {
        let id = &self.current()?.id;
        find_node(&self.root, id)
    }

    pub fn selected_win(&self) -> Option<&Win> {
        self.selected_node()?.win.as_ref()
    }

    fn refresh_peek(&mut self) {
        self.peek = self.selected_win().map(peek::peek).unwrap_or_default();
        self.sync_preview_pane();
        self.refresh_preview();
    }

    fn sync_preview_pane(&mut self) {
        let Some(win) = self.selected_win() else {
            self.preview_pane = None;
            return;
        };
        let still_there =
            self.preview_pane.as_ref().is_some_and(|id| win.panes.iter().any(|p| &p.id == id));
        if !still_there {
            self.preview_pane = win
                .panes
                .iter()
                .find(|p| p.active)
                .or_else(|| win.panes.first())
                .map(|p| p.id.clone());
            self.preview_from_bottom = 0;
        }
    }

    /// Recapture the selected pane. Cheap enough to run on a timer.
    pub fn refresh_preview(&mut self) {
        let Some(id) = self.preview_pane.clone() else {
            self.preview.clear();
            return;
        };
        match tmux::capture_pane(&id) {
            Ok(text) => self.preview = text,
            Err(e) => self.preview = format!("(no capture: {e})"),
        }
    }

    pub fn preview_pane_meta(&self) -> Option<&crate::tmux::Pane> {
        let id = self.preview_pane.as_ref()?;
        self.selected_win()?.panes.iter().find(|p| &p.id == id)
    }

    fn cycle_pane(&mut self, dir: isize) {
        let Some(win) = self.selected_win() else {
            return;
        };
        if win.panes.is_empty() {
            return;
        }
        let cur = self
            .preview_pane
            .as_ref()
            .and_then(|id| win.panes.iter().position(|p| &p.id == id))
            .unwrap_or(0);
        let n = win.panes.len() as isize;
        let next = (cur as isize + dir).rem_euclid(n) as usize;
        self.preview_pane = Some(win.panes[next].id.clone());
        self.preview_from_bottom = 0;
        self.refresh_preview();
    }

    pub fn move_sel(&mut self, delta: isize) {
        if self.rows.is_empty() {
            return;
        }
        let next = self.selected as isize + delta;
        self.selected = next.clamp(0, self.rows.len() as isize - 1) as usize;
        self.ensure_visible();
        self.refresh_peek();
    }

    pub fn ensure_visible(&mut self) {
        let h = self.tree_area.height as usize;
        if h == 0 {
            return;
        }
        if self.selected < self.scroll {
            self.scroll = self.selected;
        } else if self.selected >= self.scroll + h {
            self.scroll = self.selected + 1 - h;
        }
    }

    pub fn toggle(&mut self) {
        let Some(row) = self.current() else {
            return;
        };
        if !row.has_children {
            return;
        }
        let id = row.id.clone();
        if !self.expanded.remove(&id) {
            self.expanded.insert(id);
        }
        self.rebuild_rows();
        self.refresh_peek();
    }

    pub fn jump(&mut self, pane: Option<&str>) -> io::Result<()> {
        let Some(win) = self.selected_win().cloned() else {
            self.status = "no window on this node".into();
            return Ok(());
        };
        tmux::focus(&win.session, &win.id, pane)?;
        self.should_quit = true;
        Ok(())
    }

    pub fn on_key(&mut self, k: KeyEvent) -> io::Result<()> {
        match k.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Char('r') => self.refresh()?,
            KeyCode::Down | KeyCode::Char('j') => self.move_sel(1),
            KeyCode::Up | KeyCode::Char('k') => self.move_sel(-1),
            KeyCode::PageDown => self.move_sel(10),
            KeyCode::PageUp => self.move_sel(-10),
            KeyCode::Home => {
                self.selected = 0;
                self.ensure_visible();
                self.refresh_peek();
            }
            KeyCode::End => {
                self.selected = self.rows.len().saturating_sub(1);
                self.ensure_visible();
                self.refresh_peek();
            }
            KeyCode::Left | KeyCode::Char('h') => {
                if self.current().is_some_and(|r| r.expanded) {
                    self.toggle();
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if self.current().is_some_and(|r| r.has_children && !r.expanded) {
                    self.toggle();
                }
            }
            KeyCode::Char(' ') => self.toggle(),
            KeyCode::Tab => self.cycle_pane(1),
            KeyCode::BackTab => self.cycle_pane(-1),
            KeyCode::Char('[') => self.cycle_pane(-1),
            KeyCode::Char(']') => self.cycle_pane(1),
            KeyCode::Enter => self.go()?,
            _ => {}
        }
        Ok(())
    }

    pub fn on_mouse(&mut self, m: MouseEvent) -> io::Result<()> {
        self.cursor = (m.column, m.row);
        match m.kind {
            MouseEventKind::Down(MouseButton::Left) => self.on_left_down(m.column, m.row)?,
            MouseEventKind::ScrollDown => self.on_wheel(1),
            MouseEventKind::ScrollUp => self.on_wheel(-1),
            _ => {}
        }
        Ok(())
    }

    fn go(&mut self) -> io::Result<()> {
        self.jump(self.preview_pane.clone().as_deref())
    }

    fn on_left_down(&mut self, col: u16, row: u16) -> io::Result<()> {
        let target = if contains(self.tree_area, col, row) {
            let i = self.scroll + (row.saturating_sub(self.tree_area.y)) as usize;
            if i < self.rows.len() {
                self.selected = i;
                self.refresh_peek();
                Some(ClickTarget::Tree(i))
            } else {
                None
            }
        } else if contains(self.schematic_area, col, row) {
            let pane = self.selected_win().and_then(|win| {
                let rects = rects_for(&win.panes, self.schematic_area, win.w, win.h);
                hit(&rects, col, row).map(|pi| win.panes[pi].id.clone())
            });
            if let Some(pane) = pane {
                self.preview_pane = Some(pane.clone());
                self.preview_from_bottom = 0;
                self.refresh_preview();
                Some(ClickTarget::Schematic(pane))
            } else {
                None
            }
        } else if contains(self.preview_area, col, row) {
            Some(ClickTarget::Preview)
        } else {
            None
        };

        let Some(target) = target else {
            self.last_click = None;
            return Ok(());
        };
        let now = Instant::now();
        let doubled = self.last_click.as_ref().is_some_and(|(t, prev)| {
            *prev == target && now.duration_since(*t) < Duration::from_millis(400)
        });
        self.last_click = Some((now, target));
        if doubled {
            self.go()?;
        }
        Ok(())
    }

    /// One accepted notch = one row / one preview line. High-res wheels
    /// fire a burst per physical click; collapse those.
    fn on_wheel(&mut self, dir: isize) {
        let now = Instant::now();
        if now.duration_since(self.last_wheel) < Duration::from_millis(50) {
            return;
        }
        self.last_wheel = now;
        if contains(self.preview_area, self.cursor.0, self.cursor.1) {
            if dir > 0 {
                self.preview_from_bottom = self.preview_from_bottom.saturating_add(1);
            } else {
                self.preview_from_bottom = self.preview_from_bottom.saturating_sub(1);
            }
        } else {
            self.move_sel(dir);
        }
    }
}

fn expand_defaults(node: &Node, set: &mut HashSet<String>) {
    if node.kind == Kind::SessionGroup || node.kind == Kind::Folder {
        set.insert(node.id.clone());
    }
    for c in &node.children {
        expand_defaults(c, set);
    }
}

fn flatten(node: &Node, depth: usize, expanded: &HashSet<String>, rows: &mut Vec<Row>) {
    if node.id != "root" {
        rows.push(Row {
            id: node.id.clone(),
            depth,
            title: node.title.clone(),
            kind: node.kind,
            status: node.status,
            has_children: !node.children.is_empty(),
            expanded: expanded.contains(&node.id),
            window_count: node.window_count(),
        });
    }
    if node.id == "root" || expanded.contains(&node.id) {
        let next = if node.id == "root" { depth } else { depth + 1 };
        for c in &node.children {
            flatten(c, next, expanded, rows);
        }
    }
}

fn find_node<'a>(node: &'a Node, id: &str) -> Option<&'a Node> {
    if node.id == id {
        return Some(node);
    }
    node.children.iter().find_map(|c| find_node(c, id))
}
