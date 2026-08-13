//! Left: factory tree. Right: selected seat schematic.

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::ansi;
use crate::app::App;
use crate::geometry::rects_for;
use crate::tree::{status_label, Kind, Status};

impl App {
    pub fn draw(&mut self, f: &mut Frame) {
        let full = f.area();
        // Use the terminal/tmux default background. A painted black fill
        // fights the pane snapshot, which was drawn against that default.
        f.render_widget(
            Block::default().style(Style::default().bg(Color::Reset).fg(Color::Reset)),
            full,
        );

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(5), Constraint::Length(2)])
            .split(full);

        let body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(46), Constraint::Percentage(54)])
            .split(chunks[1]);

        self.tree_area = inner(body[0]);
        self.ensure_visible();

        self.draw_top(f, chunks[0]);
        self.draw_tree(f, body[0]);
        self.draw_seat(f, body[1]);
        self.draw_bottom(f, chunks[2]);
    }

    fn draw_top(&self, f: &mut Frame, area: Rect) {
        let title = match self.current() {
            Some(r) => format!(" factory  ·  {}", r.title),
            None => " factory".into(),
        };
        f.render_widget(
            Paragraph::new(title)
                .style(Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
            area,
        );
    }

    fn draw_tree(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" tree ")
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        f.render_widget(block, area);

        let h = inner.height as usize;
        let start = self.scroll;
        let end = (start + h).min(self.rows.len());
        for (vis, row) in self.rows[start..end].iter().enumerate() {
            let y = inner.y + vis as u16;
            let selected = start + vis == self.selected;
            let marker = if !row.has_children {
                " "
            } else if row.expanded {
                "▾"
            } else {
                "▸"
            };
            let indent = "  ".repeat(row.depth);
            let count = if row.has_children && !row.expanded {
                format!(" ({})", row.window_count)
            } else {
                String::new()
            };
            let status = status_label(row.status);
            let line = format!("{indent}{marker} {}{count}", row.title);
            let mut spans = vec![Span::styled(line, row_style(row.kind, row.status, selected))];
            if !status.is_empty() {
                spans.push(Span::raw("  "));
                spans.push(Span::styled(status, status_style(row.status)));
            }
            f.render_widget(
                Paragraph::new(Line::from(spans)),
                Rect { x: inner.x, y, width: inner.width, height: 1 },
            );
        }
    }

    fn draw_seat(&mut self, f: &mut Frame, area: Rect) {
        let title = self
            .selected_win()
            .map(|w| format!(" {}:{} ", w.session, w.name))
            .unwrap_or_else(|| " seat ".into());
        let block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        f.render_widget(block, area);

        let Some(win) = self.selected_win().cloned() else {
            self.schematic_area = Rect::default();
            self.preview_area = inner;
            f.render_widget(Paragraph::new("no window bound to this node"), inner);
            return;
        };

        let info_h = (1 + self.peek.len() as u16).min(inner.height.saturating_sub(6)).max(1);
        let multi = win.panes.len() > 1;
        let schema_h = if multi { 5.min(inner.height.saturating_sub(info_h + 4)) } else { 0 };
        let split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(info_h),
                Constraint::Length(schema_h),
                Constraint::Min(3),
            ])
            .split(inner);
        self.schematic_area = split[1];
        self.preview_area = split[2];

        let pane_label = self
            .preview_pane_meta()
            .map(|p| format!("{}:{}", p.index, p.cmd))
            .unwrap_or_else(|| "?".into());
        let mut info = vec![format!(
            "preview {pane_label}  ·  {} pane(s)  ·  snapshot, not a live embed",
            win.panes.len(),
        )];
        info.extend(self.peek.iter().cloned());
        f.render_widget(
            Paragraph::new(info.join("\n")).style(Style::default().add_modifier(Modifier::DIM)),
            split[0],
        );

        if multi {
            let rects = rects_for(&win.panes, self.schematic_area, win.w, win.h);
            for (i, r) in rects {
                let p = &win.panes[i];
                let watching = self.preview_pane.as_deref() == Some(p.id.as_str());
                let style = if watching {
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::DarkGray)
                };
                f.render_widget(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(style)
                        .title(format!(" {}:{} ", p.index, p.cmd)),
                    r,
                );
            }
        }

        let preview_block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" {pane_label} "))
            .border_style(Style::default().fg(Color::DarkGray));
        let text_area = preview_block.inner(self.preview_area);
        f.render_widget(preview_block, self.preview_area);

        let shown = tail_text(&self.preview, text_area.height as usize, self.preview_from_bottom);
        f.render_widget(Paragraph::new(shown), text_area);
    }

    fn draw_bottom(&self, f: &mut Frame, area: Rect) {
        let help =
            "j/k move  Tab pane  Enter / double-click go  click pane box to preview  r refresh  q quit";
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Length(1)])
            .split(area);
        f.render_widget(
            Paragraph::new(self.status.as_str()).style(Style::default().fg(Color::Blue)),
            chunks[0],
        );
        f.render_widget(
            Paragraph::new(help).style(Style::default().add_modifier(Modifier::DIM)),
            chunks[1],
        );
    }
}

fn tail_text(raw: &str, height: usize, from_bottom: usize) -> Text<'static> {
    if height == 0 {
        return Text::default();
    }
    let text = ansi::to_text(raw);
    if text.lines.is_empty() || (text.lines.len() == 1 && text.lines[0].spans.is_empty()) {
        return Text::from("(empty pane)");
    }
    let end = text.lines.len().saturating_sub(from_bottom).max(1);
    let start = end.saturating_sub(height);
    Text::from(text.lines[start..end].to_vec())
}

fn inner(area: Rect) -> Rect {
    Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    }
}

fn row_style(kind: Kind, status: Status, selected: bool) -> Style {
    // Dark ANSI accents only — White/Cyan/Yellow/Gray vanish on a light
    // terminal background.
    let mut s = match kind {
        Kind::SessionGroup => Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
        Kind::Window => Style::default(),
    };
    if status == Status::Parked {
        s = s.fg(Color::Red);
    }
    if selected {
        s = s.add_modifier(Modifier::REVERSED | Modifier::BOLD);
    }
    s
}

fn status_style(status: Status) -> Style {
    match status {
        Status::Running => Style::default().fg(Color::Blue),
        Status::Parked => Style::default().fg(Color::Red),
        Status::Idle | Status::Unknown => Style::default().add_modifier(Modifier::DIM),
    }
}
