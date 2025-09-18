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
pub mod bottom_bar;
pub mod command_palette;
pub mod layout;
pub mod left_panel;
pub mod main_panel;
pub mod notifications;
pub mod right_panel;
pub mod sub_panel;
pub mod tab_bar;
pub mod top_bar;

pub use bottom_bar::draw_bottom_bar;
pub use command_palette::draw_command_palette;
pub use layout::{split_center_box, split_middle_box};
pub use left_panel::draw_left_panel;
pub use main_panel::draw_main_panel;
pub use notifications::draw_notifications;
pub use right_panel::draw_right_panel;
pub use sub_panel::draw_sub_panel;
pub use tab_bar::{draw_horizontal_tab_bar, draw_vertical_tab_bar};
pub use top_bar::draw_top_bar;
