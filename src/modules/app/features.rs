#[derive(Default)]
pub struct FeatureManager {
    pub features: Vec<Feature>,
}

impl FeatureManager {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add(&mut self, feature: Feature) {
        self.features.push(feature);
    }
    pub fn del(&mut self, id: &str) {
        self.features.retain(|x| x.id() != id);
    }
}

pub struct Feature {
    id: String,
}

impl Feature {
    pub fn id(&self) -> &str {
        &self.id
    }
}
