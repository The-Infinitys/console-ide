use crate::app::Config;
use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use std::collections::VecDeque;

#[derive(Debug)]
pub enum NotificationType {
    Warn,
    Error,
    Info,
    Notify,
}

#[derive(Debug)]
pub struct Notification {
    pub notification_type: NotificationType,
    pub title: String,
    pub content: String,
    pub height: u16, // Dynamic height based on content
}

#[derive(Debug)]
pub struct NotifyWidget {
    pub notification: Notification,
}

impl NotifyWidget {
    pub fn new(notification_type: NotificationType, title: String, content: String) -> Self {
        let height = (content.lines().count() + 2) as u16; // Content lines + border
        Self {
            notification: Notification {
                notification_type,
                title,
                content,
                height,
            },
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect, config: &Config) {
        let border_style = match self.notification.notification_type {
            NotificationType::Warn => Style::default().fg(config.theme.warning),
            NotificationType::Error => Style::default().fg(config.theme.error),
            NotificationType::Info => Style::default().fg(config.theme.info),
            NotificationType::Notify => Style::default().fg(config.theme.primary),
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style)
            .title(self.notification.title.as_str())
            .style(
                Style::default()
                    .bg(config.theme.background)
                    .fg(config.theme.foreground),
            );

        let paragraph = Paragraph::new(self.notification.content.as_str()).block(block);

        f.render_widget(paragraph, area);
    }
}

#[derive(Debug)]
pub struct NotifyManager {
    pub widgets: VecDeque<NotifyWidget>,
}

impl Default for NotifyManager {
    fn default() -> Self {
        Self {
            widgets: VecDeque::new(),
        }
    }
}

impl NotifyManager {
    pub fn add(&mut self, notification_type: NotificationType, title: String, content: String) {
        self.widgets
            .push_back(NotifyWidget::new(notification_type, title, content));
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect, config: &Config) {
        if self.widgets.is_empty() {
            return;
        }

        let max_height = area.height * 75 / 100;
        let notification_width = area.width * 40 / 100;
        let margin = 1;

        let mut current_y = area.height - margin;
        let mut rendered_height = 0;

        for widget in self.widgets.iter().rev() {
            let notification_height = widget.notification.height;
            if rendered_height + notification_height + margin > max_height {
                break; // Don't render if it exceeds 75% height
            }

            current_y = current_y.saturating_sub(notification_height + margin);
            let notification_area = Rect::new(
                area.width.saturating_sub(notification_width + margin),
                current_y,
                notification_width,
                notification_height,
            );

            widget.render(f, notification_area, config);
            rendered_height += notification_height + margin;
        }
    }
}
