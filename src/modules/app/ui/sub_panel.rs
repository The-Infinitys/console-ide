use crate::modules::app::config::theme::Theme;
use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    widgets::{Block, BorderType, Borders},
};

pub fn draw_sub_panel(f: &mut Frame, area: Rect, theme: &Theme) {
    let sub_panel = Block::default()
        .borders(Borders::TOP)
        .border_style(theme.primary)
        .border_type(BorderType::QuadrantOutside)
        .style(Style::default().bg(theme.background).fg(theme.foreground));
    f.render_widget(sub_panel, area);
}
