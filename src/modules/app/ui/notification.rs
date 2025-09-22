use crate::app::Config;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, LineGauge, Paragraph},
};

use std::collections::VecDeque;
use std::time::{Duration, Instant};

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
    pub created_at: Instant,
    pub duration: Duration,
}

#[derive(Debug)]
pub struct NotifyWidget {
    pub notification: Notification,
}

impl NotifyWidget {
    pub fn new(notification_type: NotificationType, title: String, content: String) -> Self {
        let height = (content.lines().count() + 2) as u16; // Content lines + border
        let created_at = Instant::now();
        let min_duration = Duration::from_secs(5);
        let content_duration = Duration::from_millis((content.len() * 100) as u64); // 10 chars per second
        let duration = min_duration.max(content_duration);

        Self {
            notification: Notification {
                notification_type,
                title,
                content,
                height,
                created_at,
                duration,
            },
        }
    }

    pub fn render(
        &self,
        f: &mut Frame,
        area: Rect,
        config: &Config,
        remaining_time: Duration,
        total_duration: Duration,
    ) {
        let border_color = match self.notification.notification_type {
            NotificationType::Warn => config.theme.warning,
            NotificationType::Error => config.theme.error,
            NotificationType::Info => config.theme.info,
            NotificationType::Notify => config.theme.primary,
        };
        let border_style = Style::default().fg(border_color);

        // Create the block with the title
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style)
            .title(Line::from(Span::styled(
                self.notification.title.as_str(),
                Style::default().bg(Color::Reset),
            )))
            .style(
                Style::default()
                    .bg(config.theme.background)
                    .fg(config.theme.foreground),
            );

        // Render the block (this will draw borders and title)
        f.render_widget(&block, area);

        // Get the inner area for the paragraph content
        let inner_area = block.inner(area);
        let paragraph = Paragraph::new(self.notification.content.as_str());

        // Render the paragraph in the inner area
        f.render_widget(paragraph, inner_area);

        // Calculate the progress for the gauge
        let progress = if total_duration.as_secs() > 0 {
            remaining_time.as_secs_f64() / total_duration.as_secs_f64()
        } else {
            0.0
        };

        // Create the gauge widget
        let gauge = LineGauge::default()
            .filled_style(Style::default().fg(config.theme.secondary))
            .unfilled_style(Style::default().fg(border_color))
            .ratio(progress)
            .label(Span::raw("")); // Remove percentage display

        // Calculate the area for the gauge at the bottom of the notification
        let gauge_area = Rect::new(
            area.x + 1,
            area.bottom() - 1,
            area.width.saturating_sub(2),
            1,
        );

        // Render the gauge
        f.render_widget(gauge, gauge_area);
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

        let now = Instant::now();
        // Remove expired notifications
        self.widgets
            .retain(|widget| widget.notification.created_at + widget.notification.duration > now);

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

            let elapsed = now.duration_since(widget.notification.created_at);
            let remaining_time = widget.notification.duration.saturating_sub(elapsed);
            let total_duration = widget.notification.duration;

            current_y = current_y.saturating_sub(notification_height + margin);
            let notification_area = Rect::new(
                area.width.saturating_sub(notification_width + margin),
                current_y,
                notification_width,
                notification_height,
            );
            f.render_widget(Clear, notification_area.into());

            widget.render(f, notification_area, config, remaining_time, total_duration);
            rendered_height += notification_height + margin;
        }
    }
}
