use config::Config;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use event::{Event, EventHandler};
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
pub mod features;
pub mod ui;

pub struct App {
    workspace_dir: Option<PathBuf>,
    config: Config,
    event_handler: EventHandler,
}

impl Default for App {
    fn default() -> Self {
        Self {
            workspace_dir: None,
            config: Config::default(),
            event_handler: EventHandler::new(Duration::from_millis(100)),
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
        Self {
            workspace_dir: None,
            config,
            event_handler: EventHandler::new(Duration::from_millis(250)),
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

            match self.event_handler.next() {
                Event::Input(key) => {
                    let bind_id: Option<String> = {
                        let detected_bindings = self.config.keybindings.detect_events(&key);

                        // 1. System KeyBindings
                        
                        if let Some(system_bind) = detected_bindings
                            .iter()
                            .find(|bind_id| bind_id.starts_with("system."))
                        {
                            Some(system_bind.to_string())
                        } else if let Some(focused_bind) = detected_bindings
                            .iter()
                            .find(|bind_id| bind_id.starts_with("focus."))
                        {
                            Some(focused_bind.to_string())
                        } else { detected_bindings.first().map(|bind_id| bind_id.to_string()) }
                    };
                    if let Some(bind_id) = bind_id
                        && bind_id.as_str() == "system.quit" { break }
                }
                Event::Tick => {}
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
            .border_style(self.config.theme.primary)
            .border_type(BorderType::QuadrantOutside)
            .style(
                Style::default()
                    .bg(self.config.theme.background)
                    .fg(self.config.theme.foreground),
            );
        f.render_widget(top_bar, chunks[0]);

        let bottom_bar = Block::default()
            .borders(Borders::TOP)
            .border_style(self.config.theme.primary)
            .border_type(BorderType::QuadrantOutside)
            .style(
                Style::default()
                    .bg(self.config.theme.background)
                    .fg(self.config.theme.foreground),
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
            .border_style(self.config.theme.primary)
            .border_type(BorderType::QuadrantOutside)
            .style(
                Style::default()
                    .bg(self.config.theme.background)
                    .fg(self.config.theme.foreground),
            );
        f.render_widget(left_panel, middle_chunks[0]);

        let right_panel = Block::default()
            .borders(Borders::LEFT)
            .border_style(self.config.theme.primary)
            .border_type(BorderType::QuadrantOutside)
            .style(
                Style::default()
                    .bg(self.config.theme.background)
                    .fg(self.config.theme.foreground),
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
                .bg(self.config.theme.background)
                .fg(self.config.theme.foreground),
        );
        f.render_widget(main_panel, center_chunks[0]);

        let sub_panel = Block::default()
            .borders(Borders::TOP)
            .border_style(self.config.theme.primary)
            .border_type(BorderType::QuadrantOutside)
            .style(
                Style::default()
                    .bg(self.config.theme.background)
                    .fg(self.config.theme.foreground),
            );
        f.render_widget(sub_panel, center_chunks[1]);
    }
}
