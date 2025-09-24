use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub use ipak::utils::version::{self, Version};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Extension {
    pub id: String,
    pub about: ExtensionAboutInfo,
    pub relation: ExtensionRelationInfo,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtensionAboutInfo {
    pub name: String,
    pub version: Version,
    pub author: ExtensionAuthorInfo,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtensionAuthorInfo {
    pub name: String,
    pub email: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtensionRelationInfo {
    pub host_version: version::VersionRange,
    pub depend: HashMap<String, version::VersionRange>,
    pub conflict:Vec<String>,
}
