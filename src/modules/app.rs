use config::Config;
use crossterm::{
    event::{
        DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyEvent as CKeyEvent,
        MouseButton, MouseEventKind,
    },
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use event::{Event, EventHandler};
use features::FeatureManager;
use ratatui::{Frame, Terminal, backend::CrosstermBackend};
use std::{io, path::PathBuf, time::Duration};

pub mod config;
pub mod event;
pub mod features;
pub mod ui;

pub struct App {
    workspace_dir: Option<PathBuf>,
    config: Config,
    event_handler: EventHandler,
    #[allow(unused)]
    feature_manager: FeatureManager,
    ui: ui::Ui,
}

impl Default for App {
    fn default() -> Self {
        Self {
            workspace_dir: None,
            config: Config::default(),
            event_handler: EventHandler::new(Duration::from_millis(100)),
            feature_manager: FeatureManager::default(),
            ui: ui::Ui::default(),
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

        let feature_manager = FeatureManager::new();

        Self {
            workspace_dir: None,
            config,
            event_handler: EventHandler::new(Duration::from_millis(250)),
            feature_manager,
            ui: ui::Ui::default(),
        }
    }
    pub fn set_workspace(&mut self, path: PathBuf) {
        self.workspace_dir = Some(path);
    }
    pub fn process_system_bind(&mut self, system_bind: &str) {
        if let Some(focus_id) = system_bind.strip_prefix("system.focus.") {
            self.ui.focus_bind(focus_id);
        }
        // match system_bind {
        //     _ => {}
        // }
    }
    pub fn process_binding(&mut self, _bind: &str) {}
    pub fn process_key(&mut self, key: CKeyEvent) {
        self.ui.handle_key(key);
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        self.ui
            .add_notification(ui::NotificationType::Error, "Hello, Wolrd", "Console IDE");
        loop {
            terminal.draw(|f| {
                self.render(f);
            })?;

            match self.event_handler.next() {
                Event::Input(event) => match event {
                    CEvent::Key(key) => {
                        let detected_bindings = self.config.keybindings.detect_events(&key);

                        // 1. System KeyBindings

                        if let Some(system_bind) = detected_bindings
                            .iter()
                            .find(|bind_id| bind_id.starts_with("system."))
                        {
                            if system_bind == "system.quit" {
                                break;
                            } else {
                                self.process_system_bind(system_bind);
                            }
                        } else if let Some(bind) =
                            detected_bindings.first().map(|bind_id| bind_id.to_string())
                        {
                            self.process_binding(&bind)
                        } else {
                            self.process_key(key);
                        }
                    }
                    CEvent::Mouse(mouse_event) => {
                        if let MouseEventKind::Down(MouseButton::Left) = mouse_event.kind {
                            self.ui.handle_mouse_event(mouse_event);
                        }
                    }
                    _ => {}
                },
                Event::Tick => {}
            }
        }
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }

    fn render(&mut self, f: &mut Frame) {
        self.ui.render(f, &self.config);
    }
}
