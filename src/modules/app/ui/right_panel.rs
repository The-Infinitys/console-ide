use crate::modules::app::config::theme::Theme;
use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    widgets::{Block, BorderType, Borders},
};

pub fn draw_right_panel(f: &mut Frame, area: Rect, theme: &Theme) {
    let right_panel = Block::default()
        .borders(Borders::LEFT)
        .border_style(theme.primary)
        .border_type(BorderType::QuadrantOutside)
        .style(Style::default().bg(theme.background).fg(theme.foreground));
    f.render_widget(right_panel, area);
}
