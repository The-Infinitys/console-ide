use crate::utils::version::Version;
pub struct CoreFeature {
    id: String,
    pub about: CoreFeatureAbout,
}
pub struct CoreFeatureAbout {
    pub name: String,
    pub summary: String,
    pub description: String,
    pub author: String,
    pub version: Version,
}
impl CoreFeature {
    pub fn id(&self) -> &str {
        &self.id
    }
}
