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

pub mod bar;
pub mod panel;