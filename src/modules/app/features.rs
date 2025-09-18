use std::path::PathBuf;

pub struct FeatureManager {
    pub features: Vec<Feature>,
}

impl Default for FeatureManager {
    fn default() -> Self {
        Self {
            features: Vec::new(),
        }
    }
}

impl FeatureManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_feature(&mut self, feature: Feature) {
        self.features.push(feature);
    }

    pub fn get_feature_by_id(&self, id: &str) -> Option<&Feature> {
        self.features.iter().find(|f| f.id == id)
    }
}
pub struct Feature {
    pub id: String,
    pub info: FeatureInfo,
    #[allow(unused)]
    bin_path: PathBuf,
}
pub struct FeatureInfo {
    pub name: String,
    pub summary: String,
    pub description: String,
    pub url: String,
}
impl Feature {
    pub fn new(id: &str) -> Self {
        let bin_path = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "~".to_string()))
            .join(".console-ide")
            .join("features")
            .join(&id);
        Self {
            id: id.to_string(),
            info: FeatureInfo {
                name: "".into(),
                summary: "".into(),
                description: "".into(),
                url: "".into(),
            },
            bin_path,
        }
    }
}
