use crate::modules::app::config::theme::Theme;
use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    widgets::{Block, BorderType, Borders},
};

pub fn draw_left_panel(f: &mut Frame, area: Rect, theme: &Theme) {
    let left_panel = Block::default()
        .borders(Borders::RIGHT)
        .border_style(theme.primary)
        .border_type(BorderType::QuadrantOutside)
        .style(Style::default().bg(theme.background).fg(theme.foreground));
    f.render_widget(left_panel, area);
}
