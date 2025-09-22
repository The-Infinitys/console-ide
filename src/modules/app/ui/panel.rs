pub mod left_panel;
pub mod main_panel;
pub mod right_panel;
pub mod sub_panel;

#[derive(Debug, Default)]
pub struct PanelUi {
    pub main: main_panel::MainPanel,
    pub sub: sub_panel::SubPanel,
    pub left: left_panel::LeftPanel,
    pub right: right_panel::RightPanel,
}
