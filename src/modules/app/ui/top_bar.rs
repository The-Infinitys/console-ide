use crate::modules::app::config::theme::Theme;
use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    widgets::{Block, BorderType, Borders},
};

pub fn draw_top_bar(f: &mut Frame, area: Rect, theme: &Theme) {
    let top_bar = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(theme.primary)
        .border_type(BorderType::QuadrantOutside)
        .style(Style::default().bg(theme.background).fg(theme.foreground));
    f.render_widget(top_bar, area);
}
