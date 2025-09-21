use crate::utils::version::{Version, VersionRange};
use std::collections::HashMap;
pub struct Extension {
    id: String,
   pub about: ExtensionAbout,
}
pub struct ExtensionAbout {
   pub name: String,
   pub summary: String,
   pub description: String,
   pub author: String,
   pub version: Version,
   pub relation: ExrtensionRelation,
}
pub struct ExrtensionRelation {
    pub depend: HashMap<String, VersionRange>,
    pub conflict: HashMap<String, VersionRange>,
    pub host_support: VersionRange,
}
impl Extension {
    pub fn id(&self) -> &str {
        &self.id
    }
}
