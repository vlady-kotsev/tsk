use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
};

/// Classic Windows 95 title-bar blue.
pub const WIN95_BLUE: Color = Color::Rgb(0, 0, 168);
pub const CYAN: Color = Color::Cyan;
pub const MAGENTA: Color = Color::Magenta;

pub fn title_bar_style() -> Style {
    Style::default()
        .fg(Color::White)
        .bg(CYAN)
        .add_modifier(Modifier::BOLD)
}

pub fn footer_style() -> Style {
    Style::default().fg(MAGENTA)
}

pub fn selection_style() -> Style {
    Style::default()
        .bg(WIN95_BLUE)
        .fg(Color::White)
        .add_modifier(Modifier::BOLD)
}

/// Splits an area into a title bar, a body, and a footer hint line.
pub fn header_body_footer(area: Rect) -> (Rect, Rect, Rect) {
    let [header, body, footer] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(area);
    (header, body, footer)
}

/// Centers a `percent_x` x `percent_y` rect within `area`, for dialog-style popups.
pub fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let [_, vertical, _] = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .areas(area);
    let [_, horizontal, _] = Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .areas(vertical);
    horizontal
}

/// Centers a `percent_x` wide, fixed-`height` rect within `area` — a Windows 95-style
/// modal dialog box that doesn't grow with the terminal.
pub fn centered_dialog(percent_x: u16, height: u16, area: Rect) -> Rect {
    let remaining = area.height.saturating_sub(height);
    let [_, vertical, _] = Layout::vertical([
        Constraint::Length(remaining / 2),
        Constraint::Length(height),
        Constraint::Length(remaining - remaining / 2),
    ])
    .areas(area);
    let [_, horizontal, _] = Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .areas(vertical);
    horizontal
}
