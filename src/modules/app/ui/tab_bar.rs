use crate::modules::app::config::theme::Theme;
use ratatui::{Frame, layout::Rect, style::Style, widgets::Tabs};

pub fn draw_vertical_tab_bar(
    f: &mut Frame,
    area: Rect,
    theme: &Theme,
    tabs: &[&str],
    selected: usize,
) {
    let tabs_widget = Tabs::new(
        tabs.iter()
            .map(|s| (*s).to_string())
            .collect::<Vec<String>>(),
    )
    .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::NONE))
    .select(selected)
    .style(Style::default().fg(theme.foreground))
    .highlight_style(Style::default().fg(theme.primary));
    // 縦方向タブバーはratatui標準APIでは未サポートのため、必要ならカスタム描画が必要です。
    f.render_widget(tabs_widget, area);
}

pub fn draw_horizontal_tab_bar(
    f: &mut Frame,
    area: Rect,
    theme: &Theme,
    tabs: &[&str],
    selected: usize,
) {
    let tabs_widget = Tabs::new(
        tabs.iter()
            .map(|s| (*s).to_string())
            .collect::<Vec<String>>(),
    )
    .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::NONE))
    .select(selected)
    .style(Style::default().fg(theme.foreground))
    .highlight_style(Style::default().fg(theme.primary));
    f.render_widget(tabs_widget, area);
}
