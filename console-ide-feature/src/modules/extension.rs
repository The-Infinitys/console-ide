use std::collections::HashMap;

pub use ipak::utils::version::{self, Version};

#[derive(Clone, Debug)]
pub struct Extension {
    pub id: String,
    pub about: ExtensionAboutInfo,
    pub relation: ExtensionRelationInfo,
}

#[derive(Clone, Debug)]
pub struct ExtensionAboutInfo {
    pub name: String,
    pub version: Version,
    pub author: ExtensionAuthorInfo,
}

#[derive(Clone, Debug)]
pub struct ExtensionAuthorInfo {
    pub name: String,
    pub email: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ExtensionRelationInfo {
    pub host_version: version::VersionRange,
    pub depend: HashMap<String, version::VersionRange>,
    pub conflict:Vec<String>,
}
