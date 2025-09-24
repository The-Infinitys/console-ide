use std::collections::{HashMap, HashSet};
use super::extension::Extension;
use ipak::utils::version::{Version, VersionRange};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ExtensionId(String);

impl From<String> for ExtensionId {
    fn from(s: String) -> Self {
        ExtensionId(s)
    }
}

impl From<&str> for ExtensionId {
    fn from(s: &str) -> Self {
        ExtensionId(s.to_string())
    }
}

impl std::fmt::Display for ExtensionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Resolver {
    available_extensions: HashMap<ExtensionId, Extension>,
}

#[derive(Debug)]
pub enum ResolutionError {
    NotFound(ExtensionId),
    VersionMismatch {
        extension_id: ExtensionId,
        required: VersionRange,
        found: Version,
    },
    Conflict(ExtensionId, ExtensionId),
    CircularDependency(ExtensionId),
    MissingDependency {
        dependent: ExtensionId,
        missing: ExtensionId,
        required_range: VersionRange,
    },
}

impl std::fmt::Display for ResolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolutionError::NotFound(id) => write!(f, "Extension '{}' not found.", id),
            ResolutionError::VersionMismatch { extension_id, required, found } => {
                write!(f, "Extension '{}' requires version '{}', but found '{}'.", extension_id, required, found)
            }
            ResolutionError::Conflict(id1, id2) => {
                write!(f, "Conflict detected between extensions '{}' and '{}'.", id1, id2)
            }
            ResolutionError::CircularDependency(id) => {
                write!(f, "Circular dependency detected involving extension '{}'.", id)
            }
            ResolutionError::MissingDependency { dependent, missing, required_range } => {
                write!(f, "Extension '{}' depends on '{}' (version '{}'), but it is missing.", dependent, missing, required_range)
            }
        }
    }
}

#[derive(Debug)]
pub struct ResolvedExtension {
    pub extension: Extension,
    pub resolved_dependencies: Vec<ExtensionId>,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            available_extensions: HashMap::new(),
        }
    }

    pub fn add_extension(&mut self, extension: Extension) {
        self.available_extensions.insert(extension.id.clone().into(), extension);
    }

    pub fn resolve(&self, root_extension_ids: &[ExtensionId]) -> Result<HashMap<ExtensionId, ResolvedExtension>, Vec<ResolutionError>> {
        let mut resolved_extensions: HashMap<ExtensionId, ResolvedExtension> = HashMap::new();
        let mut errors: Vec<ResolutionError> = Vec::new();
        let mut visited: HashSet<ExtensionId> = HashSet::new();
        let mut visiting: HashSet<ExtensionId> = HashSet::new();

        for root_id in root_extension_ids {
            if let Err(e) = self.resolve_recursive(root_id, &mut resolved_extensions, &mut errors, &mut visited, &mut visiting) {
                errors.push(e);
            }
        }

        if errors.is_empty() {
            Ok(resolved_extensions)
        } else {
            Err(errors)
        }
    }

    fn resolve_recursive(
        &self,
        extension_id: &ExtensionId,
        resolved_extensions: &mut HashMap<ExtensionId, ResolvedExtension>,
        errors: &mut Vec<ResolutionError>,
        visited: &mut HashSet<ExtensionId>,
        visiting: &mut HashSet<ExtensionId>,
    ) -> Result<(), ResolutionError> {
        if resolved_extensions.contains_key(extension_id) {
            return Ok(());
        }

        if visiting.contains(extension_id) {
            return Err(ResolutionError::CircularDependency(extension_id.clone()));
        }

        if visited.contains(extension_id) {
            return Ok(());
        }

        visiting.insert(extension_id.clone());

        let extension = self.available_extensions.get(extension_id)
            .ok_or_else(|| ResolutionError::NotFound(extension_id.clone()))?;

        let mut current_resolved_dependencies = Vec::new();

        // Resolve dependencies
        for (dep_id, dep_range) in &extension.relation.depend {
            let dep_id: ExtensionId = dep_id.clone().into();
            let dep_extension = self.available_extensions.get(&dep_id)
                .ok_or_else(|| ResolutionError::MissingDependency {
                    dependent: extension_id.clone(),
                    missing: dep_id.clone(),
                    required_range: dep_range.clone(),
                })?;

            // Check version compatibility
            let dep_version = dep_extension.about.version.clone();
            if !dep_range.compare(&dep_version) {
                return Err(ResolutionError::VersionMismatch {
                    extension_id: dep_id.clone(),
                    required: dep_range.clone(),
                    found: dep_version,
                });
            }

            self.resolve_recursive(&dep_id, resolved_extensions, errors, visited, visiting)?;
            current_resolved_dependencies.push(dep_id);
        }

        // Check for conflicts
        for conflict_id_str in &extension.relation.conflict {
            let conflict_id: ExtensionId = conflict_id_str.clone().into();
            if self.available_extensions.contains_key(&conflict_id) {
                return Err(ResolutionError::Conflict(extension_id.clone(), conflict_id));
            }
        }

        visiting.remove(extension_id);
        visited.insert(extension_id.clone());

        resolved_extensions.insert(
            extension_id.clone(),
            ResolvedExtension {
                extension: extension.clone(),
                resolved_dependencies: current_resolved_dependencies,
            },
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use super::super::extension::{ExtensionAboutInfo, ExtensionAuthorInfo, ExtensionRelationInfo};

    fn create_extension(
        id: &str,
        version: &str,
        dependencies: Vec<(&str, &str)>,
        conflicts: Vec<&str>,
    ) -> Extension {
        let mut depend_map = HashMap::new();
        for (dep_id, dep_version_range) in dependencies {
            depend_map.insert(dep_id.to_string(), VersionRange::from_str(dep_version_range).unwrap());
        }

        Extension {
            id: id.to_string(),
            about: ExtensionAboutInfo {
                name: format!("{} Name", id),
                version: Version::from_str(version).unwrap(),
                author: ExtensionAuthorInfo {
                    name: "Test Author".to_string(),
                    email: None,
                },
            },
            relation: ExtensionRelationInfo {
                host_version: VersionRange::from_str("*").unwrap(),
                depend: depend_map,
                conflict: conflicts.iter().map(|s| s.to_string()).collect(),
            },
        }
    }

    #[test]
    fn test_add_extension() {
        let mut resolver = Resolver::new();
        let ext1 = create_extension("ext1", "1.0.0", vec![], vec![]);
        resolver.add_extension(ext1.clone());
        assert!(resolver.available_extensions.contains_key(&ExtensionId::from("ext1")));
        assert_eq!(resolver.available_extensions.get(&ExtensionId::from("ext1")).unwrap().id, "ext1");
    }

    #[test]
    fn test_resolve_single_extension_no_deps() {
        let mut resolver = Resolver::new();
        let ext1 = create_extension("ext1", "1.0.0", vec![], vec![]);
        resolver.add_extension(ext1.clone());

        let result = resolver.resolve(&[ExtensionId::from("ext1")]);
        assert!(result.is_ok());
        let resolved = result.unwrap();
        assert_eq!(resolved.len(), 1);
        assert!(resolved.contains_key(&ExtensionId::from("ext1")));
    }

    #[test]
    fn test_resolve_with_dependency() {
        let mut resolver = Resolver::new();
        let ext_dep = create_extension("ext_dep", "1.0.0", vec![], vec![]);
        let ext_main = create_extension("ext_main", "1.0.0", vec![("ext_dep", "1.0.0")], vec![]);
        resolver.add_extension(ext_dep.clone());
        resolver.add_extension(ext_main.clone());

        let result = resolver.resolve(&[ExtensionId::from("ext_main")]);
        assert!(result.is_ok());
        let resolved = result.unwrap();
        assert_eq!(resolved.len(), 2);
        assert!(resolved.contains_key(&ExtensionId::from("ext_main")));
        assert!(resolved.contains_key(&ExtensionId::from("ext_dep")));
    }

    #[test]
    fn test_resolve_missing_dependency() {
        let mut resolver = Resolver::new();
        let ext_main = create_extension("ext_main", "1.0.0", vec![("ext_dep", "1.0.0")], vec![]);
        resolver.add_extension(ext_main.clone());

        let result = resolver.resolve(&[ExtensionId::from("ext_main")]);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        match &errors[0] {
            ResolutionError::MissingDependency { dependent, missing, .. } => {
                assert_eq!(dependent, &ExtensionId::from("ext_main"));
                assert_eq!(missing, &ExtensionId::from("ext_dep"));
            },
            _ => panic!("Expected MissingDependency error"),
        }
    }

    #[test]
    fn test_resolve_version_mismatch() {
        let mut resolver = Resolver::new();
        let ext_dep = create_extension("ext_dep", "2.0.0", vec![], vec![]);
        let ext_main = create_extension("ext_main", "1.0.0", vec![("ext_dep", "1.0.0")], vec![]);
        resolver.add_extension(ext_dep.clone());
        resolver.add_extension(ext_main.clone());

        let result = resolver.resolve(&[ExtensionId::from("ext_main")]);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        match &errors[0] {
            ResolutionError::VersionMismatch { extension_id, required: _, found } => {
                assert_eq!(extension_id, &ExtensionId::from("ext_dep"));
                assert_eq!(found, &Version::from_str("2.0.0").unwrap());
            },
            _ => panic!("Expected VersionMismatch error"),
        }
    }

    #[test]
    fn test_resolve_circular_dependency() {
        let mut resolver = Resolver::new();
        let ext1 = create_extension("ext1", "1.0.0", vec![("ext2", "*")], vec![]);
        let ext2 = create_extension("ext2", "1.0.0", vec![("ext1", "*")], vec![]);
        resolver.add_extension(ext1.clone());
        resolver.add_extension(ext2.clone());

        let result = resolver.resolve(&[ExtensionId::from("ext1")]);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        match &errors[0] {
            ResolutionError::CircularDependency(id) => {
                assert!(id == &ExtensionId::from("ext1") || id == &ExtensionId::from("ext2"));
            },
            _ => panic!("Expected CircularDependency error"),
        }
    }

    #[test]
    fn test_resolve_conflict() {
        let mut resolver = Resolver::new();
        let ext1 = create_extension("ext1", "1.0.0", vec![], vec!["ext2"]);
        let ext2 = create_extension("ext2", "1.0.0", vec![], vec![]);
        resolver.add_extension(ext1.clone());
        resolver.add_extension(ext2.clone());

        let result = resolver.resolve(&[ExtensionId::from("ext1"), ExtensionId::from("ext2")]);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        match &errors[0] {
            ResolutionError::Conflict(id1, id2) => {
                assert!((id1 == &ExtensionId::from("ext1") && id2 == &ExtensionId::from("ext2")) ||
                         (id1 == &ExtensionId::from("ext2") && id2 == &ExtensionId::from("ext1")));
            },
            _ => panic!("Expected Conflict error"),
        }
    }
}