use std::collections::HashMap;

pub use ipak::utils::version;
pub struct Extension {
    id: String,
    pub about: ExtensionAboutInfo,
    pub relation: ExtensionRelationInfo,
}
pub struct ExtensionAboutInfo {
    pub name: String,
    pub author: ExtensionAuthorInfo,
}
pub struct ExtensionAuthorInfo {
    pub name: String,
    pub email: Option<String>,
}
pub struct ExtensionRelationInfo {
    host_version: version::VersionRange,
    depend: HashMap<String, version::VersionRange>,
    conflict:Vec<String>
}
