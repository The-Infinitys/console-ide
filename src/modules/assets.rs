pub mod logo {
    pub fn console_ide() -> &'static str {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/logo/console-ide.asc"
        ))
    }
    pub fn shell() -> &'static str {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/logo/shell.asc"
        ))
    }

    pub fn files() -> &'static str {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/logo/files.asc"
        ))
    }

    pub fn chat() -> &'static str {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/logo/chat.asc"))
    }
}
