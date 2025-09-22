use crossterm::event::MouseEvent;
#[derive(Debug, Default)]

pub struct BottomBar {}

impl BottomBar {
    pub fn render(
        &mut self,
        f: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
        _config: &crate::app::Config,
    ) {
        use ratatui::widgets::Paragraph;
        let widget = Paragraph::new("console-ide 0.1.0");
        f.render_widget(widget, area);
    }

    pub fn handle_mouse_event(&mut self, _mouse_event: MouseEvent) {
        // TODO: Implement specific mouse event handling for BottomBar
        // println!("Bottom Bar mouse event: {:?}", mouse_event);
    }
}
