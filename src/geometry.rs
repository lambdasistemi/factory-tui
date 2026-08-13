//! Scale tmux cell geometry into on-screen rectangles and hit-test them.

use ratatui::layout::Rect;

use crate::tmux::Pane;

/// Scale a pane's tmux cell-geometry into the on-screen `area`.
pub fn map_rect(p: &Pane, area: Rect, win_w: u16, win_h: u16) -> Rect {
    let sx = area.width as f32 / win_w.max(1) as f32;
    let sy = area.height as f32 / win_h.max(1) as f32;
    let x = (area.x + (p.left as f32 * sx).round() as u16).min(area.right().saturating_sub(1));
    let y = (area.y + (p.top as f32 * sy).round() as u16).min(area.bottom().saturating_sub(1));
    let max_w = area.right().saturating_sub(x).max(1);
    let max_h = area.bottom().saturating_sub(y).max(1);
    let w = ((p.width as f32 * sx).round() as u16).clamp(1, max_w);
    let h = ((p.height as f32 * sy).round() as u16).clamp(1, max_h);
    Rect { x, y, width: w, height: h }
}

/// `(index, rect)` for every pane, scaled into `canvas`.
pub fn rects_for(panes: &[Pane], canvas: Rect, win_w: u16, win_h: u16) -> Vec<(usize, Rect)> {
    panes.iter().enumerate().map(|(i, p)| (i, map_rect(p, canvas, win_w, win_h))).collect()
}

/// Half-open hit test.
pub fn contains(r: Rect, col: u16, row: u16) -> bool {
    col >= r.x && col < r.right() && row >= r.y && row < r.bottom()
}

/// Smallest containing pane wins so nested borders do not steal the click.
pub fn hit(rects: &[(usize, Rect)], col: u16, row: u16) -> Option<usize> {
    rects
        .iter()
        .filter(|(_, r)| contains(*r, col, row))
        .min_by_key(|(_, r)| r.width as u32 * r.height as u32)
        .map(|(i, _)| *i)
}
