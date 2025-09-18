use ipak::prelude::ipak::{Version, VersionRange};
use std::str::FromStr;
use std::{collections::HashMap, path::PathBuf};
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
    pub relations: FeatureRelationsInfo,
    #[allow(unused)]
    bin_path: PathBuf,
}
pub struct FeatureInfo {
    pub name: String,
    pub summary: String,
    pub description: String,
    pub url: String,
    pub version: Version,
}
pub struct FeatureRelationsInfo {
    pub host_version: VersionRange,
    pub depend: HashMap<String, VersionRange>,
    pub conflict: Vec<String>,
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
                version: Version::from_str("1.0.0").unwrap(),
            },
            relations: FeatureRelationsInfo {
                host_version: VersionRange::from_str("*").unwrap(),
                depend: HashMap::new(),
                conflict: Vec::new(),
            },
            bin_path,
        }
    }
}
