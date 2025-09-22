mod bottom_bar;
mod top_bar;
#[derive(Debug, Default)]

pub struct BarUi {
    pub top: top_bar::TopBar,
    pub bottom: bottom_bar::BottomBar,
}
