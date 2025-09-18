use crate::modules::app::config::theme::Theme;
use ratatui::{Frame, layout::Rect, style::Style, widgets::Block};

use crate::modules::app::ui::WidgetItem;
pub fn draw_main_panel(f: &mut Frame, area: Rect, theme: &Theme, widgets: &[WidgetItem]) {
    let main_panel = Block::default()
        .borders(ratatui::widgets::Borders::NONE)
        .style(Style::default().bg(theme.background).fg(theme.foreground));
    f.render_widget(main_panel, area);
    for widget in widgets {
        match widget {
            WidgetItem::Buildin(w) => {/* ... */},
            WidgetItem::Extension(e) => {/* ... */},
        }
    }
}
