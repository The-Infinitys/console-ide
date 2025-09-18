use crate::modules::app::config::theme::Theme;
use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Paragraph},
};

pub fn draw_palette(f: &mut Frame, area: Rect, theme: &Theme, input: &str, visible: bool) {
    if !visible {
        return;
    }
    let width = area.width.min(50);
    let rect = Rect::new(
        area.left() + (area.width - width) / 2,
        area.top() + 1,
        width,
        3,
    );
    let para = Paragraph::new(input)
        .block(
            Block::default()
                .title("Command Palette")
                .borders(Borders::ALL),
        )
        .style(Style::default().bg(theme.background).fg(theme.primary));
    f.render_widget(para, rect);
}
