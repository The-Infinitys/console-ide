use crate::app::Config;
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
mod panel;
#[derive(Debug, Default)]
pub struct Ui {
    panel: panel::PanelUi,
    bar: bar::BarUi,
    pub focused_on: FocusedElement,
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
            _ => None,
        };

        if let Some(new_focus) = new_focus {
            // If the new focus is the same as the current focus, close the panel and go to MainPanel
            if self.focused_on == new_focus {
                match self.focused_on {
                    FocusedElement::LeftPanel => self.panel.left.is_closed = true,
                    FocusedElement::RightPanel => self.panel.right.is_closed = true,
                    FocusedElement::SubPanel => self.panel.sub.is_closed = true,
                    _ => {} // MainPanel, Notification, Pallete don't have is_closed
                }
                self.focused_on = FocusedElement::MainPanel;
            } else {
                // Then, open the new panel (if it's a panel that can be opened)
                match new_focus {
                    FocusedElement::LeftPanel => self.panel.left.is_closed = false,
                    FocusedElement::RightPanel => self.panel.right.is_closed = false,
                    FocusedElement::SubPanel => self.panel.sub.is_closed = false,
                    _ => {}
                }
                self.focused_on = new_focus;
            }
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
        let inner_top_bar_area = chunks[0].shrink(ShrinkDirection::Bottom, 1);
        self.bar.top.render(f, inner_top_bar_area, config);

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
        let inner_bottom_bar_area = chunks[2].shrink(ShrinkDirection::Top, 1);
        self.bar.bottom.render(f, inner_bottom_bar_area, config);

        let mut middle_constraints = vec![];
        let mut has_left_panel = false;
        if !self.panel.left.is_closed {
            let left_panel_width;
            has_left_panel = true;
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

        let mut has_right_panel = false;
        if !self.panel.right.is_closed {
            has_right_panel = true;
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

        if has_left_panel {
            let left_panel_block = Block::default()
                .borders(Borders::RIGHT)
                .border_style(config.theme.primary)
                .border_type(BorderType::QuadrantOutside)
                .style(
                    Style::default()
                        .bg(config.theme.background)
                        .fg(config.theme.foreground),
                );
            f.render_widget(left_panel_block, middle_chunks[current_chunk_idx]);
            let inner_left_panel_area =
                middle_chunks[current_chunk_idx].shrink(ShrinkDirection::Right, 1);
            self.panel.left.render(f, inner_left_panel_area, config);
            current_chunk_idx += 1;
        }

        let center_area_for_panels = middle_chunks[current_chunk_idx];
        current_chunk_idx += 1; // Move past the center panel's chunk

        if has_right_panel {
            let right_panel_block = Block::default()
                .borders(Borders::LEFT)
                .border_style(config.theme.primary)
                .border_type(BorderType::QuadrantOutside)
                .style(
                    Style::default()
                        .bg(config.theme.background)
                        .fg(config.theme.foreground),
                );
            f.render_widget(right_panel_block, middle_chunks[current_chunk_idx]);
            let inner_right_panel_area =
                middle_chunks[current_chunk_idx].shrink(ShrinkDirection::Left, 1);
            self.panel.right.render(f, inner_right_panel_area, config);
        }

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

        let main_panel = Block::default().borders(Borders::NONE).style(
            Style::default()
                .bg(config.theme.background)
                .fg(config.theme.foreground),
        );
        f.render_widget(main_panel, center_chunks[0]);
        self.panel.main.render(f, center_chunks[0], config);

        if !self.panel.sub.is_closed {
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
            let inner_sub_panel_area = center_chunks[1].shrink(ShrinkDirection::Top, 1);
            self.panel.sub.render(f, inner_sub_panel_area, config);
        }
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
