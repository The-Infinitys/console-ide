use crate::app::Config;
use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

#[derive(Debug, Default)]
pub struct Pallete {
    pub is_open: bool,
    pub input: String,
    pub cursor_position: usize,
}

impl Pallete {
    pub fn render(&mut self, f: &mut Frame, area: Rect, config: &Config) {
        if !self.is_open {
            return;
        }

        let width = area.width / 2;
        let x = area.width / 2 - width / 2;
        let pallete_area = Rect::new(x, 1, width, 3); // 1 character from top, 3 lines height for border + input
        f.render_widget(Clear, pallete_area);

        let input_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(config.theme.primary)
            .title("Pallete")
            .style(
                Style::default()
                    .bg(config.theme.background)
                    .fg(config.theme.foreground),
            );

        let input_text = Paragraph::new(self.input.as_str()).block(input_block);

        f.render_widget(input_text, pallete_area);
        f.set_cursor_position((
            pallete_area.x + 1 + self.cursor_position as u16,
            pallete_area.y + 1,
        ));
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if !self.is_open {
            return;
        }
        match key.code {
            crossterm::event::KeyCode::Char(c) => {
                self.input.insert(self.cursor_position, c);
                self.cursor_position += 1;
            }
            crossterm::event::KeyCode::Backspace => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                    self.input.remove(self.cursor_position);
                }
            }
            crossterm::event::KeyCode::Left => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                }
            }
            crossterm::event::KeyCode::Right => {
                if self.cursor_position < self.input.len() {
                    self.cursor_position += 1;
                }
            }
            crossterm::event::KeyCode::Esc => {
                self.is_open = false;
                self.input.clear();
                self.cursor_position = 0;
            }
            _ => {}
        }
    }
}
