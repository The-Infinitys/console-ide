use crate::app::ui::widget::PanelWidget;
use crate::assets;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::widgets::Borders;
#[derive(Debug, Default)]
pub struct MainPanel {
    pub widgets: Vec<PanelWidget>,
}

impl MainPanel {
    pub fn render(
        &mut self,
        f: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
        _config: &crate::app::Config,
    ) {
        use ratatui::prelude::*;
        use ratatui::widgets::{Block, Paragraph};
        if self.widgets.is_empty() {
            let recommend_keybindings = [
                ["パレットを起動", "ctrl+p"],
                ["アプリケーションを終了", "ctrl+q"],
            ];
            let logo = assets::logo::console_ide();

            let logo_height = logo.lines().count() as u16;
            let keybindings_height = recommend_keybindings.len() as u16;

            let total_content_height = logo_height + keybindings_height + 1; // +1 for a small gap

            let centered_vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(total_content_height),
                    Constraint::Min(0),
                ])
                .split(area);

            let logo_width = logo.lines().map(|s| s.len()).max().unwrap_or(0) as u16;

            // Calculate max_name_len for alignment
            let max_name_len = recommend_keybindings
                .iter()
                .map(|[name, _]| name.len())
                .max()
                .unwrap_or(0) as u16;

            // Calculate max_key_len
            let max_key_len = recommend_keybindings
                .iter()
                .map(|[_, key]| key.len())
                .max()
                .unwrap_or(0) as u16;

            let keybindings_block_width = max_name_len + 2 + max_key_len; // name + ": " + key

            let max_content_width = std::cmp::max(logo_width, keybindings_block_width);

            let centered_horizontal_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(max_content_width),
                    Constraint::Min(0),
                ])
                .split(centered_vertical_chunks[1]);

            let content_area = centered_horizontal_chunks[1];

            let content_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(logo_height),
                    Constraint::Length(1), // Gap
                    Constraint::Length(keybindings_height),
                ])
                .split(content_area);
            let logo_paragraph = Paragraph::new(logo)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::NONE));
            f.render_widget(logo_paragraph, content_chunks[0]); // Render logo

            // Render keybindings in content_chunks[2]
            let keybindings_area = content_chunks[2];

            let keybinding_rows = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    recommend_keybindings
                        .iter()
                        .map(|_| Constraint::Length(1))
                        .collect::<Vec<_>>(),
                )
                .split(keybindings_area);

            for (i, [name, key]) in recommend_keybindings.iter().enumerate() {
                let row_area = keybinding_rows[i];
                let row_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Min(0),               // Left padding
                        Constraint::Length(max_name_len), // Use max_name_len for alignment
                        Constraint::Length(2),            // ": "
                        Constraint::Length(key.len() as u16),
                        Constraint::Min(0), // Right padding
                    ])
                    .split(row_area);

                let name_paragraph = Paragraph::new(*name).alignment(Alignment::Right);
                let separator_paragraph = Paragraph::new(": ").alignment(Alignment::Center);
                let key_paragraph = Paragraph::new(*key).alignment(Alignment::Left);

                f.render_widget(name_paragraph, row_chunks[1]);
                f.render_widget(separator_paragraph, row_chunks[2]);
                f.render_widget(key_paragraph, row_chunks[3]);
            }
        }
    }

    pub fn handle_mouse_event(&mut self, _mouse_event: MouseEvent) {
        // TODO: Implement specific mouse event handling for MainPanel
        // println!("Main Panel mouse event: {:?}", mouse_event);
    }

    pub fn handle_key(&mut self, _key: KeyEvent) {
        // TODO: Implement specific key event handling for MainPanel
    }
}
