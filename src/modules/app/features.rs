pub mod core;
pub mod extension;

#[derive(Default)]
pub struct FeatureManager {
    pub core: Vec<core::CoreFeature>,
    pub extension: Vec<extension::Extension>,
}


impl FeatureManager {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_core(&mut self, core: core::CoreFeature) {
        self.core.push(core);
    }
    pub fn add_extension(&mut self, extension: extension::Extension) {
        self.extension.push(extension);
    }
    pub fn del(&mut self, id: &str) {
        self.core.retain(|x| x.id() != id);
        self.extension.retain(|x| x.id() != id);
    }
}
