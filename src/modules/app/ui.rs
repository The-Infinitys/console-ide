use crate::app::Config;
use ratatui::Frame;

#[derive(Debug, Clone)]
pub enum WidgetItem {
    Buildin(BuildinWidget),
    Extension(ExtensionWidget),
}

#[derive(Debug, Clone)]
pub struct BuildinWidget {
    pub id: String,
    // 必要に応じて追加
}

#[derive(Debug, Clone)]
pub struct ExtensionWidget {
    pub id: String,
    pub binary_path: String,
    // 必要に応じて追加
}

mod bar;
mod panel;
#[derive(Debug, Default)]
pub struct Ui {
    panel: panel::PanelUi,
    bar: bar::BarUi,
}

impl Ui {
    pub fn render(&mut self, f: &mut Frame, config: &Config) {
        use ratatui::{
            layout::{Constraint, Direction, Layout},
            style::Style,
            widgets::{Block, BorderType, Borders},
        };
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
            .border_style(config.theme.primary)
            .border_type(BorderType::QuadrantOutside)
            .style(
                Style::default()
                    .bg(config.theme.background)
                    .fg(config.theme.foreground),
            );
        f.render_widget(top_bar, chunks[0]);
        self.bar.top.render(f, chunks[0], config);

        let bottom_bar = Block::default()
            .borders(Borders::TOP)
            .border_style(config.theme.primary)
            .border_type(BorderType::QuadrantOutside)
            .style(
                Style::default()
                    .bg(config.theme.background)
                    .fg(config.theme.foreground),
            );
        f.render_widget(bottom_bar, chunks[2]);
        self.bar.bottom.render(f, chunks[2], config);

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
            .border_style(config.theme.primary)
            .border_type(BorderType::QuadrantOutside)
            .style(
                Style::default()
                    .bg(config.theme.background)
                    .fg(config.theme.foreground),
            );
        f.render_widget(left_panel, middle_chunks[0]);
        self.panel.left.render(f, middle_chunks[0], config);

        let right_panel = Block::default()
            .borders(Borders::LEFT)
            .border_style(config.theme.primary)
            .border_type(BorderType::QuadrantOutside)
            .style(
                Style::default()
                    .bg(config.theme.background)
                    .fg(config.theme.foreground),
            );
        f.render_widget(right_panel, middle_chunks[2]);
        self.panel.right.render(f, middle_chunks[2], config);

        let center_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(70), // Main panel
                Constraint::Percentage(30), // Sub panel
            ])
            .split(middle_chunks[1]);

        let main_panel = Block::default().borders(Borders::NONE).style(
            Style::default()
                .bg(config.theme.background)
                .fg(config.theme.foreground),
        );
        f.render_widget(main_panel, center_chunks[0]);
        self.panel.main.render(f, center_chunks[0], config);

        let sub_panel = Block::default()
            .borders(Borders::TOP)
            .border_style(config.theme.primary)
            .border_type(BorderType::QuadrantOutside)
            .style(
                Style::default()
                    .bg(config.theme.background)
                    .fg(config.theme.foreground),
            );
        f.render_widget(sub_panel, center_chunks[1]);
        self.panel.sub.render(f, center_chunks[1], config);
    }
}
