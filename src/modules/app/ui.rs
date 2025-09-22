use crate::app::Config;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{
    Frame,
    layout::{Direction, Rect},
};
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
mod notification;
mod pallete;
mod panel;
#[derive(Debug, Default)]
pub struct Ui {
    panel: panel::PanelUi,
    bar: bar::BarUi,
    pallete: pallete::Pallete,
    notification_manager: notification::NotifyManager,
    pub focused_on: FocusedElement,
    top_bar_area: Rect,
    bottom_bar_area: Rect,
    left_panel_area: Rect,
    right_panel_area: Rect,
    main_panel_area: Rect,
    sub_panel_area: Rect,
}
#[derive(Default, Debug, PartialEq, Eq)]
pub enum FocusedElement {
    #[default]
    MainPanel,
    SubPanel,
    LeftPanel,
    RightPanel,
    Notification,
    Pallete,
}

impl Ui {
    pub fn focus_bind(&mut self, focus_id: &str) {
        let new_focus = match focus_id {
            "mainpanel" => Some(FocusedElement::MainPanel),
            "subpanel" => Some(FocusedElement::SubPanel),
            "leftpanel" => Some(FocusedElement::LeftPanel),
            "rightpanel" => Some(FocusedElement::RightPanel),
            "pallete" => Some(FocusedElement::Pallete),
            _ => None,
        };

        if let Some(new_focus) = new_focus {
            // If the new focus is the same as the current focus, close the panel and go to MainPanel
            if self.focused_on == new_focus {
                match self.focused_on {
                    FocusedElement::LeftPanel => self.panel.left.is_closed = true,
                    FocusedElement::RightPanel => self.panel.right.is_closed = true,
                    FocusedElement::SubPanel => self.panel.sub.is_closed = true,
                    FocusedElement::Pallete => self.pallete.is_open = false,
                    _ => {} // MainPanel, Notification don't have is_closed
                }
                self.focused_on = FocusedElement::MainPanel;
            } else {
                // Close all panels and pallete first
                self.panel.left.is_closed = true;
                self.panel.right.is_closed = true;
                self.panel.sub.is_closed = true;
                self.pallete.is_open = false;

                // Then, open the new panel (if it's a panel that can be opened)
                match new_focus {
                    FocusedElement::LeftPanel => self.panel.left.is_closed = false,
                    FocusedElement::RightPanel => self.panel.right.is_closed = false,
                    FocusedElement::SubPanel => self.panel.sub.is_closed = false,
                    FocusedElement::Pallete => self.pallete.is_open = true,
                    _ => {}
                }
                self.focused_on = new_focus;
            }
        }
    }
    pub fn handle_mouse_event(&mut self, mouse_event: MouseEvent) {
        let mouse_pos = (mouse_event.column, mouse_event.row);

        if self
            .top_bar_area
            .contains(ratatui::prelude::Position::new(mouse_pos.0, mouse_pos.1))
        {
            self.bar.top.handle_mouse_event(mouse_event);
        } else if self
            .bottom_bar_area
            .contains(ratatui::prelude::Position::new(mouse_pos.0, mouse_pos.1))
        {
            self.bar.bottom.handle_mouse_event(mouse_event);
        } else if self
            .left_panel_area
            .contains(ratatui::prelude::Position::new(mouse_pos.0, mouse_pos.1))
        {
            self.focused_on = FocusedElement::LeftPanel;
            self.panel.left.handle_mouse_event(mouse_event);
        } else if self
            .right_panel_area
            .contains(ratatui::prelude::Position::new(mouse_pos.0, mouse_pos.1))
        {
            self.focused_on = FocusedElement::RightPanel;
            self.panel.right.handle_mouse_event(mouse_event);
        } else if self
            .sub_panel_area
            .contains(ratatui::prelude::Position::new(mouse_pos.0, mouse_pos.1))
        {
            self.focused_on = FocusedElement::SubPanel;
            self.panel.sub.handle_mouse_event(mouse_event);
        } else if self
            .main_panel_area
            .contains(ratatui::prelude::Position::new(mouse_pos.0, mouse_pos.1))
        {
            self.focused_on = FocusedElement::MainPanel;
            self.panel.main.handle_mouse_event(mouse_event);
        }
    }
    pub fn render(&mut self, f: &mut Frame, config: &Config) {
        use ratatui::{
            layout::{Constraint, Layout},
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

        self.top_bar_area = chunks[0];
        let top_bar = Block::default()
            .borders(Borders::BOTTOM)
            .border_style(config.theme.primary)
            .border_type(BorderType::QuadrantOutside)
            .style(
                Style::default()
                    .bg(config.theme.background)
                    .fg(config.theme.foreground),
            );
        f.render_widget(top_bar, self.top_bar_area);
        let inner_top_bar_area = self.top_bar_area.shrink(ShrinkDirection::Bottom, 1);
        self.bar.top.render(f, inner_top_bar_area, config);

        self.bottom_bar_area = chunks[2];
        let bottom_bar = Block::default()
            .borders(Borders::TOP)
            .border_style(config.theme.primary)
            .border_type(BorderType::QuadrantOutside)
            .style(
                Style::default()
                    .bg(config.theme.background)
                    .fg(config.theme.foreground),
            );
        f.render_widget(bottom_bar, self.bottom_bar_area);
        let inner_bottom_bar_area = self.bottom_bar_area.shrink(ShrinkDirection::Top, 1);
        self.bar.bottom.render(f, inner_bottom_bar_area, config);

        let mut middle_constraints = vec![];
        if self.panel.left.is_closed {
            middle_constraints.push(Constraint::Max(5));
        } else {
            let left_panel_width;
            if self.focused_on == FocusedElement::LeftPanel {
                if self.panel.right.is_closed {
                    left_panel_width = 75;
                } else {
                    left_panel_width = 50;
                }
            } else {
                left_panel_width = 25;
            }
            middle_constraints.push(Constraint::Percentage(left_panel_width));
        }

        middle_constraints.push(Constraint::Min(0)); // Center box, will take remaining space

        if self.panel.right.is_closed {
            middle_constraints.push(Constraint::Max(5));
        } else {
            let right_panel_width;
            if self.focused_on == FocusedElement::RightPanel {
                if self.panel.left.is_closed {
                    right_panel_width = 75;
                } else {
                    right_panel_width = 50;
                }
            } else {
                right_panel_width = 25;
            }
            middle_constraints.push(Constraint::Percentage(right_panel_width));
        }

        let middle_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(middle_constraints)
            .split(chunks[1]);

        let mut current_chunk_idx = 0;

        self.left_panel_area = middle_chunks[current_chunk_idx];
        let left_panel_block = Block::default()
            .borders(Borders::RIGHT)
            .border_style(config.theme.primary)
            .border_type(BorderType::QuadrantOutside)
            .style(
                Style::default()
                    .bg(config.theme.background)
                    .fg(config.theme.foreground),
            );
        f.render_widget(left_panel_block, self.left_panel_area);
        let inner_left_panel_area = self.left_panel_area.shrink(ShrinkDirection::Right, 1);
        self.panel.left.render(f, inner_left_panel_area, config);
        current_chunk_idx += 1;

        let center_area_for_panels = middle_chunks[current_chunk_idx];
        current_chunk_idx += 1; // Move past the center panel's chunk

        self.right_panel_area = middle_chunks[current_chunk_idx];
        let right_panel_block = Block::default()
            .borders(Borders::LEFT)
            .border_style(config.theme.primary)
            .border_type(BorderType::QuadrantOutside)
            .style(
                Style::default()
                    .bg(config.theme.background)
                    .fg(config.theme.foreground),
            );
        f.render_widget(right_panel_block, self.right_panel_area);
        let inner_right_panel_area = self.right_panel_area.shrink(ShrinkDirection::Left, 1);
        self.panel.right.render(f, inner_right_panel_area, config);

        let center_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(match self.focused_on {
                FocusedElement::SubPanel => [
                    Constraint::Percentage(25), // Main panel
                    Constraint::Percentage(75), // Sub panel
                ],
                _ => [
                    Constraint::Percentage(75), // Main panel
                    Constraint::Percentage(25), // Sub panel
                ],
            })
            .split(center_area_for_panels);

        self.main_panel_area = center_chunks[0];
        let main_panel = Block::default().borders(Borders::NONE).style(
            Style::default()
                .bg(config.theme.background)
                .fg(config.theme.foreground),
        );
        f.render_widget(main_panel, self.main_panel_area);
        self.panel.main.render(f, self.main_panel_area, config);

        if !self.panel.sub.is_closed {
            self.sub_panel_area = center_chunks[1];
            let sub_panel = Block::default()
                .borders(Borders::TOP)
                .border_style(config.theme.primary)
                .border_type(BorderType::QuadrantOutside)
                .style(
                    Style::default()
                        .bg(config.theme.background)
                        .fg(config.theme.foreground),
                );
            f.render_widget(sub_panel, self.sub_panel_area);
            let inner_sub_panel_area = self.sub_panel_area.shrink(ShrinkDirection::Top, 1);
            self.panel.sub.render(f, inner_sub_panel_area, config);
        }

        // Render Pallete last to ensure it's on top
        self.pallete.render(f, f.area(), config);

        // Render Notifications last to ensure they are on top
        self.notification_manager.render(f, f.area(), config);
    }
    pub fn handle_key(&mut self, key: KeyEvent) {
        if self.pallete.is_open {
            self.pallete.handle_key(key);
            return;
        }
        match self.focused_on {
            FocusedElement::MainPanel => self.panel.main.handle_key(key),
            FocusedElement::SubPanel => self.panel.sub.handle_key(key),
            FocusedElement::LeftPanel => self.panel.left.handle_key(key),
            FocusedElement::RightPanel => self.panel.right.handle_key(key),
            _ => {}
        }
    }

    pub fn add_notification(
        &mut self,
        notification_type: notification::NotificationType,
        title: String,
        content: String,
    ) {
        self.notification_manager
            .add(notification_type, title, content);
    }
}

#[derive(Clone, Copy)]
pub enum ShrinkDirection {
    Top,
    Bottom,
    Left,
    Right,
}
pub trait ShrinkRect {
    fn shrink(&self, direction: ShrinkDirection, length: u16) -> Self;
}
impl ShrinkRect for Rect {
    fn shrink(&self, direction: ShrinkDirection, length: u16) -> Self {
        let mut rect = *self;
        match direction {
            ShrinkDirection::Bottom => rect.height = rect.height.saturating_sub(length),
            ShrinkDirection::Left => {
                rect.x = rect.x.saturating_add(length);
                rect.width = rect.width.saturating_sub(length)
            }
            ShrinkDirection::Right => rect.width = rect.width.saturating_sub(length),
            ShrinkDirection::Top => {
                rect.y = rect.y.saturating_add(length);
                rect.height = rect.height.saturating_sub(length);
            }
        }
        rect
    }
}
