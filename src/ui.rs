//! Left: factory tree. Right: selected seat schematic.

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::ansi;
use crate::app::App;
use crate::geometry::rects_for;
use crate::tmux::{self, Pane};
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

        let shown_pane = self.preview_pane_meta().cloned();
        let mut info =
            vec![preview_meta_line(shown_pane.as_ref(), split[0].width, win.panes.len())];
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
                        .title(box_title(Some(p), r.width)),
                    r,
                );
            }
        }

        let preview_block = Block::default()
            .borders(Borders::ALL)
            .title(box_title(shown_pane.as_ref(), self.preview_area.width))
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

/// Fixed lead-in of the preview metadata line.
const META_PREFIX: &str = "preview ";

/// Terminal cells a string occupies, measured the way ratatui places it.
fn cells(text: &str) -> usize {
    Span::raw(text).width()
}

/// Drop escape sequences first, then every remaining control character, then
/// collapse the whitespace that leaves behind. A pane title is written by
/// whatever runs in the pane, so raw it can recolour, erase, or — where a label
/// is drawn from a plain string — split the frame it lands in.
///
/// The control pass does not assume what `strip_ansi` chooses to keep, so it
/// is redundant with today's keep-list and no input can distinguish it — see
/// the equivalent mutant M07 in the campaign log. It stays as this function's
/// own stated contract, not as a check anything relies on.
fn sanitize_label(raw: &str) -> String {
    let stripped = tmux::strip_ansi(raw);
    let plain: String = stripped.chars().map(|c| if c.is_control() { ' ' } else { c }).collect();
    plain.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Cut to `max` terminal cells, marking the cut so a shortened title still
/// reads as one.
fn fit_cells(text: &str, max: u16) -> String {
    let max = max as usize;
    if max == 0 {
        return String::new();
    }
    if cells(text) <= max {
        return text.to_string();
    }
    let budget = max - 1;
    let mut out = String::new();
    let mut used = 0;
    let mut buf = [0u8; 4];
    for ch in text.chars() {
        let w = cells(ch.encode_utf8(&mut buf));
        if used + w > budget {
            break;
        }
        out.push(ch);
        used += w;
    }
    out.push('…');
    out
}

/// One identity rule for every pane label site: the operator-authored tmux
/// title when it survives sanitation, otherwise the `{index}:{command}`
/// identity this view has always used. Never blank while a cell is free, never
/// wider than `max_width` terminal cells.
fn pane_label(pane: &Pane, max_width: u16) -> String {
    let title = sanitize_label(&pane.title);
    let text = if title.is_empty() {
        let fallback = sanitize_label(&format!("{}:{}", pane.index, pane.cmd));
        if fallback.is_empty() {
            "?".to_string()
        } else {
            fallback
        }
    } else {
        title
    };
    fit_cells(&text, max_width)
}

fn label_or_unknown(pane: Option<&Pane>, budget: u16) -> String {
    match pane {
        Some(pane) => pane_label(pane, budget),
        None => fit_cells("?", budget),
    }
}

/// Title for a bordered pane box. A box of `box_width` offers `box_width - 2`
/// title cells, and the padding below costs two more.
fn box_title(pane: Option<&Pane>, box_width: u16) -> String {
    if box_width < 4 {
        return String::new();
    }
    format!(" {} ", label_or_unknown(pane, box_width - 4))
}

/// The metadata line above the preview. The label gets exactly the cells this
/// line has left once its fixed decoration is accounted for.
fn preview_meta_line(pane: Option<&Pane>, width: u16, pane_count: usize) -> String {
    let suffix = format!("  ·  {pane_count} pane(s)  ·  snapshot, not a live embed");
    let fixed = u16::try_from(cells(META_PREFIX) + cells(&suffix)).unwrap_or(u16::MAX);
    let label = label_or_unknown(pane, width.saturating_sub(fixed));
    format!("{META_PREFIX}{label}{suffix}")
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
        Kind::SessionGroup | Kind::Folder => {
            Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)
        }
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

#[cfg(test)]
mod tests {
    use ratatui::backend::TestBackend;
    use ratatui::buffer::Buffer;
    use ratatui::Terminal;

    use super::*;

    /// A title a hostile pane can set with a single escape sequence: colour,
    /// screen erase, an OSC window-title write, a carriage return, a newline,
    /// a tab, a backspace, and a raw C1 CSI byte.
    const HOSTILE: &str = "\u{1b}[31m\u{1b}[2Jred\u{1b}]0;owned\u{7}\r\nsecond\tline\u{8}\u{9b}5m";

    /// What `HOSTILE` must look like once it is safe to draw.
    const HOSTILE_SAFE: &str = "red second line5m";

    fn pane(index: &str, cmd: &str, title: &str) -> Pane {
        Pane {
            id: format!("%{index}"),
            index: index.to_string(),
            left: 0,
            top: 0,
            width: 80,
            height: 24,
            active: true,
            cmd: cmd.to_string(),
            path: "/code".to_string(),
            title: title.to_string(),
        }
    }

    /// One rendered row, one character per terminal cell.
    fn row_text(buf: &Buffer, y: u16) -> String {
        (0..buf.area.width).map(|x| buf.cell((x, y)).unwrap().symbol()).collect()
    }

    fn render(width: u16, height: u16, draw: impl Fn(&mut Frame)) -> Buffer {
        let mut term = Terminal::new(TestBackend::new(width, height)).unwrap();
        term.draw(|f| draw(f)).unwrap();
        term.backend().buffer().clone()
    }

    #[test]
    fn pane_label_falls_back_for_empty_title() {
        assert_eq!(pane_label(&pane("2", "bash", ""), 40), "2:bash");
        assert_eq!(pane_label(&pane("2", "bash", "   "), 40), "2:bash");
        // A title made only of controls sanitizes to nothing, and a pane box
        // with no text in it identifies nothing.
        assert_eq!(pane_label(&pane("2", "bash", "\u{1b}[31m\u{1b}[2J\r\n\t\u{8}"), 40), "2:bash");
        // A usable title wins over the command it would otherwise share.
        assert_eq!(pane_label(&pane("2", "bash", "owner-1"), 40), "owner-1");

        // No budget above zero may produce a blank label.
        for width in 1..=40u16 {
            for p in [pane("2", "bash", ""), pane("2", "bash", "\u{1b}[2J")] {
                assert!(!pane_label(&p, width).is_empty(), "blank label at width {width}");
            }
        }
    }

    #[test]
    fn pane_title_escape_cannot_corrupt_frame() {
        let p = pane("0", "bash", HOSTILE);

        let label = pane_label(&p, 30);
        assert_eq!(label, HOSTILE_SAFE);
        assert!(!label.contains('\u{1b}'));
        assert!(!label.chars().any(char::is_control));

        // Rendered proof, pane box: the title must not erase or restyle the
        // frame it sits in, and the border must survive it.
        let buf = render(40, 3, |f| {
            f.render_widget(
                Block::default().borders(Borders::ALL).title(box_title(Some(&p), 40)),
                Rect { x: 0, y: 0, width: 40, height: 3 },
            );
        });
        let top = row_text(&buf, 0);
        assert_eq!(top.chars().count(), 40);
        assert!(top.starts_with(&format!("┌ {HOSTILE_SAFE} ─")), "top row was {top:?}");
        assert!(top.ends_with('┐'));
        assert_eq!(row_text(&buf, 2), format!("└{}┘", "─".repeat(38)));
        for cell in buf.content() {
            assert_eq!(cell.fg, Color::Reset, "a pane title must not colour the frame");
            assert_eq!(cell.bg, Color::Reset);
            assert_eq!(cell.modifier, Modifier::empty());
        }

        // Rendered proof, metadata line: this line is drawn from a plain
        // string, so an unsanitized newline in the title would split it into a
        // second row and push the frame down.
        let buf = render(100, 2, |f| {
            f.render_widget(
                Paragraph::new(preview_meta_line(Some(&p), 100, 2)),
                Rect { x: 0, y: 0, width: 100, height: 2 },
            );
        });
        assert!(row_text(&buf, 0).contains(HOSTILE_SAFE), "row 0 was {:?}", row_text(&buf, 0));
        assert_eq!(row_text(&buf, 1).trim(), "", "the title split the metadata line");
    }

    #[test]
    fn pane_label_respects_terminal_cell_width() {
        let long = pane("0", "cargo", "observability dashboard rebuild · phase two");
        for width in 0..=48u16 {
            let label = pane_label(&long, width);
            assert!(cells(&label) <= width as usize, "width {width} produced {label:?}");
        }
        assert_eq!(pane_label(&long, 0), "");

        // A narrow box keeps the operator's title rather than the command, and
        // says that it was cut.
        let narrow = pane_label(&long, 12);
        assert_eq!(cells(&narrow), 12);
        assert!(narrow.starts_with("observabili"), "narrow label was {narrow:?}");
        assert!(narrow.ends_with('…'));
        assert!(!narrow.contains("0:cargo"));

        // A leading glyph that occupies two cells must be charged two cells.
        let cjk = pane("0", "cargo", "日本語のタイトル");
        assert_eq!(cells("日本語のタイトル"), 16);
        for width in 0..=20u16 {
            let label = pane_label(&cjk, width);
            assert!(cells(&label) <= width as usize, "wide label {label:?} at width {width}");
        }

        // Each site is bounded by its own real budget, not by `pane_label`
        // alone: a bordered box spends two cells on its border, and the
        // metadata line spends the rest on fixed decoration.
        for width in 0..=48u16 {
            let title = box_title(Some(&long), width);
            assert!(
                cells(&title) <= width.saturating_sub(2) as usize,
                "box title {title:?} does not fit a {width}-cell box"
            );
        }
        let meta = preview_meta_line(Some(&long), 60, 2);
        assert!(cells(&meta) <= 60, "metadata line took {} cells: {meta:?}", cells(&meta));

        // The measure must agree with what ratatui paints: a label budgeted at
        // ten cells, drawn into a twenty-four cell row, must leave the rest of
        // the row untouched.
        let label = pane_label(&cjk, 10);
        let buf = render(24, 1, |f| {
            f.render_widget(
                Paragraph::new(label.clone()),
                Rect { x: 0, y: 0, width: 24, height: 1 },
            );
        });
        let painted = row_text(&buf, 0);
        assert_eq!(
            painted.chars().skip(10).collect::<String>().trim(),
            "",
            "label {label:?} painted past its ten-cell budget: {painted:?}"
        );
    }

    #[test]
    fn every_pane_label_site_uses_one_identity_rule() {
        let titled = pane("1", "claude", "owner-1");
        assert_eq!(box_title(Some(&titled), 40), " owner-1 ");
        assert!(preview_meta_line(Some(&titled), 100, 3).starts_with("preview owner-1  ·  3 pane"));

        let blank = pane("1", "claude", "");
        assert_eq!(box_title(Some(&blank), 40), " 1:claude ");
        assert!(preview_meta_line(Some(&blank), 100, 3).starts_with("preview 1:claude  ·  3 pane"));

        // Two panes running the same command are distinguishable.
        let other = pane("2", "claude", "auditor-1");
        assert_ne!(box_title(Some(&titled), 40), box_title(Some(&other), 40));

        // No pane bound: the site still renders something.
        assert_eq!(box_title(None, 40), " ? ");
    }
}
