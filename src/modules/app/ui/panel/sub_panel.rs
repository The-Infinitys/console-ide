use crossterm::event::MouseEvent;
#[derive(Debug, Default)]
pub struct SubPanel {
    pub is_closed: bool,
}

impl SubPanel {
    pub fn render(
        &mut self,
        f: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
        _config: &crate::app::Config,
    ) {
        use ratatui::widgets::Paragraph;
        let widget = Paragraph::new("Sub Panel");
        f.render_widget(widget, area);
    }

    pub fn handle_mouse_event(&mut self, _mouse_event: MouseEvent) {
        // TODO: Implement specific mouse event handling for SubPanel
        // println!("Sub Panel mouse event: {:?}", mouse_event);
    }
}
