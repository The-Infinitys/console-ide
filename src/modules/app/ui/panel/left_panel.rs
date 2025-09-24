use crossterm::event::{KeyEvent, MouseEvent};

use crate::app::ui::widget::PanelWidget;
#[derive(Debug, Default)]
pub struct LeftPanel {
    pub is_closed: bool,
    pub _widgets: Vec<PanelWidget>,
}

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

    pub fn handle_mouse_event(&mut self, _mouse_event: MouseEvent) {
        // TODO: Implement specific mouse event handling for LeftPanel
        // println!("Left Panel mouse event: {:?}", mouse_event);
    }

    pub fn handle_key(&mut self, _key: KeyEvent) {
        // TODO: Implement specific key event handling for LeftPanel
    }
}
