use crate::modules::app::config::{Config, theme::Theme};
use crossterm::{
    event::{Event as CEvent, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, BorderType, Borders},
};
use std::{io, path::PathBuf, time::Duration};

pub mod config;
pub mod event;
pub mod ui;

pub struct App {
    workspace_dir: Option<PathBuf>,
    config: Config,
    theme: Theme,
}

impl Default for App {
    fn default() -> Self {
        Self {
            workspace_dir: None,
            config: Config::default(),
            theme: Theme::default(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        let config_path = PathBuf::from("~/.config")
            .join("console-ide")
            .join("config");
        let config = Config::load(&config_path.join("main.yaml")).unwrap_or_else(|e| {
            eprintln!("Failed to load main config from {:?}: {}", config_path, e);
            Config::default()
        });

        let theme = Theme::load(&config_path.join("theme.yaml")).unwrap_or_else(|e| {
            eprintln!("Failed to load theme config from {:?}: {}", config_path, e);
            Theme::default()
        });

        Self {
            workspace_dir: None,
            config,
            theme,
        }
    }
    pub fn set_workspace(&mut self, path: PathBuf) {
        self.workspace_dir = Some(path);
    }
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|f| {
                self.render(f);
            })?;

            if crossterm::event::poll(Duration::from_millis(250))? {
                if let CEvent::Key(key) = crossterm::event::read()? {
                    match key.code {
                        KeyCode::Char('q') => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        Ok(())
    }

    fn render(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2), // Top bar
                Constraint::Min(0),    // Middle box
                Constraint::Length(2), // Bottom bar
            ])
            .split(f.area());

        let top_bar = Block::default()
            .borders(Borders::BOTTOM)
            .border_style(self.theme.primary)
            .border_type(BorderType::QuadrantOutside)
            .style(
                Style::default()
                    .bg(self.theme.background)
                    .fg(self.theme.foreground),
            );
        f.render_widget(top_bar, chunks[0]);

        let bottom_bar = Block::default()
            .borders(Borders::TOP)
            .border_style(self.theme.primary)
            .border_type(BorderType::QuadrantOutside)
            .style(
                Style::default()
                    .bg(self.theme.background)
                    .fg(self.theme.foreground),
            );
        f.render_widget(bottom_bar, chunks[2]);

        let middle_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(1),     // Left panel
                Constraint::Min(0),     // Center box
                Constraint::Length(20), // Right panel (placeholder width)
            ])
            .split(chunks[1]);

        let left_panel = Block::default()
            .borders(Borders::RIGHT)
            .border_style(self.theme.primary)
            .border_type(BorderType::QuadrantOutside)
            .style(
                Style::default()
                    .bg(self.theme.background)
                    .fg(self.theme.foreground),
            );
        f.render_widget(left_panel, middle_chunks[0]);

        let right_panel = Block::default()
            .borders(Borders::LEFT)
            .border_style(self.theme.primary)
            .border_type(BorderType::QuadrantOutside)
            .style(
                Style::default()
                    .bg(self.theme.background)
                    .fg(self.theme.foreground),
            );
        f.render_widget(right_panel, middle_chunks[2]);

        let center_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(70), // Main panel
                Constraint::Percentage(30), // Sub panel
            ])
            .split(middle_chunks[1]);

        let main_panel = Block::default().borders(Borders::NONE).style(
            Style::default()
                .bg(self.theme.background)
                .fg(self.theme.foreground),
        );
        f.render_widget(main_panel, center_chunks[0]);

        let sub_panel = Block::default()
            .borders(Borders::TOP)
            .border_style(self.theme.primary)
            .border_type(BorderType::QuadrantOutside)
            .style(
                Style::default()
                    .bg(self.theme.background)
                    .fg(self.theme.foreground),
            );
        f.render_widget(sub_panel, center_chunks[1]);
    }
}
