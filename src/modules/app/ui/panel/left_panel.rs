use crossterm::event::{KeyEvent, MouseEvent};

use crate::{app::ui::widget::PanelWidget, assets};
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
        use ratatui::{
            prelude::*,
            widgets::{Block, Borders, Paragraph},
        };

        let logo = assets::logo::files();
        let logo_height = logo.lines().count() as u16;
        let logo_width = logo.lines().map(|s| s.len()).max().unwrap_or(0) as u16;

        let centered_vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(logo_height),
                Constraint::Min(0),
            ])
            .split(area);

        let centered_horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(logo_width),
                Constraint::Min(0),
            ])
            .split(centered_vertical_chunks[1]);

        let logo_paragraph = Paragraph::new(logo)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::NONE));
        f.render_widget(logo_paragraph, centered_horizontal_chunks[1]);
    }

    pub fn handle_mouse_event(&mut self, _mouse_event: MouseEvent) {
        // TODO: Implement specific mouse event handling for LeftPanel
        // println!("Left Panel mouse event: {:?}", mouse_event);
    }

    pub fn handle_key(&mut self, _key: KeyEvent) {
        // TODO: Implement specific key event handling for LeftPanel
    }
}
