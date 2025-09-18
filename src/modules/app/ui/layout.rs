use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub fn split_middle_box(area: Rect) -> [Rect; 3] {
    let v = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(1),     // Left panel
            Constraint::Min(0),     // Center box
            Constraint::Length(20), // Right panel（仮幅）
        ])
        .split(area);
    [v[0], v[1], v[2]]
}

pub fn split_center_box(area: Rect) -> [Rect; 2] {
    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(70), // Main panel
            Constraint::Percentage(30), // Sub panel
        ])
        .split(area);
    [v[0], v[1]]
}
