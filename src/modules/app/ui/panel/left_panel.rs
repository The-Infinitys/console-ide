#[derive(Debug, Default)]

pub struct LeftPanel {}

impl LeftPanel {
    pub fn render(
        &mut self,
        f: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
        _config: &crate::app::Config,
    ) {
        use ratatui::widgets::Paragraph;
        let widget = Paragraph::new("Left Panel");
        f.render_widget(widget, area);
    }
}
