//! SGR (color/attribute) from `tmux capture-pane -e` → ratatui `Text`.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};

/// Convert a capture that still contains escape sequences into styled lines.
pub fn to_text(input: &str) -> Text<'static> {
    let mut lines: Vec<Line<'static>> = Vec::new();
    let mut spans: Vec<Span<'static>> = Vec::new();
    let mut buf = String::new();
    let mut style = Style::default();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\u{1b}' {
            match chars.peek() {
                Some('[') => {
                    chars.next();
                    let mut seq = String::new();
                    let mut fin = 'm';
                    for x in chars.by_ref() {
                        if x.is_ascii_alphabetic() || x == '~' {
                            fin = x;
                            break;
                        }
                        seq.push(x);
                    }
                    if fin == 'm' {
                        flush_span(&mut spans, &mut buf, style);
                        style = apply_sgr(style, &seq);
                    }
                }
                Some(']') => {
                    chars.next();
                    for x in chars.by_ref() {
                        if x == '\u{7}' {
                            break;
                        }
                    }
                }
                Some(_) => {
                    chars.next();
                }
                None => {}
            }
        } else if c == '\n' {
            flush_span(&mut spans, &mut buf, style);
            lines.push(Line::from(std::mem::take(&mut spans)));
        } else if c == '\r' || c == '\u{8}' {
            // ignore
        } else if c == '\t' || !c.is_control() {
            buf.push(c);
        }
    }
    flush_span(&mut spans, &mut buf, style);
    if !spans.is_empty() || lines.is_empty() {
        lines.push(Line::from(spans));
    }
    Text::from(lines)
}

fn flush_span(spans: &mut Vec<Span<'static>>, buf: &mut String, style: Style) {
    if buf.is_empty() {
        return;
    }
    spans.push(Span::styled(std::mem::take(buf), style));
}

fn apply_sgr(mut style: Style, seq: &str) -> Style {
    if seq.is_empty() {
        return Style::default();
    }
    let params: Vec<u8> =
        seq.split(';').map(|p| if p.is_empty() { 0 } else { p.parse().unwrap_or(0) }).collect();
    let mut i = 0;
    while i < params.len() {
        match params[i] {
            0 => style = Style::default(),
            1 => style = style.add_modifier(Modifier::BOLD),
            2 => style = style.add_modifier(Modifier::DIM),
            3 => style = style.add_modifier(Modifier::ITALIC),
            4 => style = style.add_modifier(Modifier::UNDERLINED),
            7 => style = style.add_modifier(Modifier::REVERSED),
            22 => style = style.remove_modifier(Modifier::BOLD | Modifier::DIM),
            23 => style = style.remove_modifier(Modifier::ITALIC),
            24 => style = style.remove_modifier(Modifier::UNDERLINED),
            27 => style = style.remove_modifier(Modifier::REVERSED),
            39 => style = style.fg(Color::Reset),
            49 => style = style.bg(Color::Reset),
            n @ 30..=37 => style = style.fg(basic(n - 30, false)),
            n @ 90..=97 => style = style.fg(basic(n - 90, true)),
            n @ 40..=47 => style = style.bg(basic(n - 40, false)),
            n @ 100..=107 => style = style.bg(basic(n - 100, true)),
            38 | 48 => {
                let is_fg = params[i] == 38;
                if let Some((color, used)) = ext_color(&params[i + 1..]) {
                    style = if is_fg { style.fg(color) } else { style.bg(color) };
                    i += used;
                }
            }
            _ => {}
        }
        i += 1;
    }
    style
}

fn ext_color(rest: &[u8]) -> Option<(Color, usize)> {
    match rest {
        [5, n, ..] => Some((Color::Indexed(*n), 2)),
        [2, r, g, b, ..] => Some((Color::Rgb(*r, *g, *b), 4)),
        _ => None,
    }
}

fn basic(n: u8, bright: bool) -> Color {
    match (n, bright) {
        (0, false) => Color::Black,
        (1, false) => Color::Red,
        (2, false) => Color::Green,
        (3, false) => Color::Yellow,
        (4, false) => Color::Blue,
        (5, false) => Color::Magenta,
        (6, false) => Color::Cyan,
        (7, false) => Color::White,
        (0, true) => Color::DarkGray,
        (1, true) => Color::LightRed,
        (2, true) => Color::LightGreen,
        (3, true) => Color::LightYellow,
        (4, true) => Color::LightBlue,
        (5, true) => Color::LightMagenta,
        (6, true) => Color::LightCyan,
        (7, true) => Color::Gray,
        _ => Color::Reset,
    }
}

#[cfg(test)]
mod tests {
    use super::to_text;
    use ratatui::style::Color;

    #[test]
    fn keeps_green_span() {
        let text = to_text("\u{1b}[32mhello\u{1b}[0m world");
        assert_eq!(text.lines.len(), 1);
        assert_eq!(text.lines[0].spans.len(), 2);
        assert_eq!(text.lines[0].spans[0].content, "hello");
        assert_eq!(text.lines[0].spans[0].style.fg, Some(Color::Green));
        assert_eq!(text.lines[0].spans[1].content, " world");
    }

    #[test]
    fn indexed_and_rgb() {
        let text = to_text("\u{1b}[38;5;81mcyan\u{1b}[38;2;255;80;0morange");
        assert_eq!(text.lines[0].spans[0].style.fg, Some(Color::Indexed(81)));
        assert_eq!(text.lines[0].spans[1].style.fg, Some(Color::Rgb(255, 80, 0)));
    }
}
