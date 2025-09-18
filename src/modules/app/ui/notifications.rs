use crate::modules::app::config::theme::Theme;
use ratatui::{Frame, layout::Rect, style::Style, widgets::Paragraph};

pub fn draw_notifications(f: &mut Frame, area: Rect, theme: &Theme, notifications: &[&str]) {
    let height = notifications.len() as u16;
    let mut y = area.bottom() - height;
    for msg in notifications.iter().rev() {
        let rect = Rect::new(area.right() - 30, y, 30, 1);
        let para =
            Paragraph::new(*msg).style(Style::default().bg(theme.primary).fg(theme.background));
        f.render_widget(para, rect);
        y += 1;
    }
}
