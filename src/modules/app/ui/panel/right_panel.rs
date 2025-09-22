#[derive(Debug,Default)]

pub struct RightPanel{}

impl RightPanel {
    pub fn render(&mut self, f: &mut ratatui::Frame, area: ratatui::layout::Rect, _config: &crate::app::Config) {
        use ratatui::widgets::Paragraph;
        let widget = Paragraph::new("Right Panel");
        f.render_widget(widget, area);
    }
}