use crate::modules::app::config::theme::Theme;
use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    widgets::{Block, BorderType, Borders},
};

use crate::modules::app::ui::WidgetItem;
pub fn draw_bottom_bar(f: &mut Frame, area: Rect, theme: &Theme, widgets: &[WidgetItem]) {
    let bottom_bar = Block::default()
        .borders(Borders::TOP)
        .border_style(theme.primary)
        .border_type(BorderType::QuadrantOutside)
        .style(Style::default().bg(theme.background).fg(theme.foreground));
    f.render_widget(bottom_bar, area);
    for widget in widgets {
        match widget {
            WidgetItem::Buildin(w) => {/* ... */},
            WidgetItem::Extension(e) => {/* ... */},
        }
    }
}
