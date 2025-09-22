#[derive(Debug, Default)]

pub struct TopBar {}

impl TopBar {
    pub fn render(
        &mut self,
        f: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
        _config: &crate::app::Config,
    ) {
        use ratatui::widgets::Paragraph;
        let widget = Paragraph::new("Top Bar");
        f.render_widget(widget, area);
    }
}
