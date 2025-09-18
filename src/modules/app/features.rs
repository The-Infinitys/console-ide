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
        self.features.iter().find(|f| f.id() == id)
    }
}

pub enum Feature {
    Buildin(BuildinFeature),
    Extension(ExtensionFeature),
}

impl Feature {
    pub fn id(&self) -> &str {
        match self {
            Feature::Buildin(f) => &f.id,
            Feature::Extension(f) => &f.id,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Feature::Buildin(f) => &f.name,
            Feature::Extension(f) => &f.name,
        }
    }

    pub fn author(&self) -> &str {
        match self {
            Feature::Buildin(f) => &f.author,
            Feature::Extension(f) => &f.author,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Feature::Buildin(f) => &f.description,
            Feature::Extension(f) => &f.description,
        }
    }
}

pub struct BuildinFeature {
    pub id: String,
    pub name: String,
    pub author: String,
    pub description: String,
}

impl BuildinFeature {
    pub fn new(id: &str, name: &str, author: &str, description: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            author: author.to_string(),
            description: description.to_string(),
        }
    }
}

pub struct ExtensionFeature {
    pub id: String,
    pub name: String,
    pub author: String,
    pub description: String,
    pub binary_path: PathBuf,
}

impl ExtensionFeature {
    pub fn new(id: String) -> Self {
        let binary_path = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "~".to_string()))
            .join(".console-ide")
            .join("extensions")
            .join(&id);

        Self {
            id: id.clone(),
            name: id.clone(),
            author: id.clone(),
            description: id.clone(),
            binary_path,
        }
    }
}
