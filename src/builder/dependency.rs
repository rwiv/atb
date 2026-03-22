use crate::core::{CONFIG_FILE_NAME, DEPS_FILE_NAME, Resource, ResourceType};
use crate::loader::registry::Registry;
use anyhow::{Result, anyhow};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// requirements.yaml 파싱을 위한 데이터 모델
#[derive(Debug, Deserialize, Clone, Default)]
pub struct DependencyConfig {
    #[serde(flatten)]
    // Key: ResourceType (복수형, 예: "agents")
    // Value: Map<ResourceName, Map<DependencyType(복수형), Vec<Plugin:Name>>>
    pub types: HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>,
}

/// 리소스 간의 의존성을 검증하는 컴포넌트
#[derive(Default)]
pub struct DependencyChecker {
    /// 플러그인별 requirements.yaml 로드 결과 캐시 (None은 파일이 없음을 의미)
    cache: std::cell::RefCell<HashMap<String, Option<DependencyConfig>>>,
}

impl DependencyChecker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registry의 모든 리소스를 순회하며 의존성을 검증합니다.
    pub fn check_dependencies(&self, registry: &Registry, source_dir: &Path) -> Result<()> {
        let mut errors = Vec::new();

        for resource in registry.all_resources() {
            self.check_resource_dependencies(resource, registry, source_dir, &mut errors)?;
        }

        if !errors.is_empty() {
            let msg = format!("Dependency check failed:\n  - {}", errors.join("\n  - "));
            return Err(anyhow!(msg));
        }

        Ok(())
    }

    /// 단일 리소스의 의존성을 검사하고 발견된 오류를 errors 벡터에 수집합니다.
    fn check_resource_dependencies(
        &self,
        resource: &Resource,
        registry: &Registry,
        source_dir: &Path,
        errors: &mut Vec<String>,
    ) -> Result<()> {
        let plugin = resource.plugin();
        let r_type = resource.r_type();
        let name = resource.name();

        // 1. 해당 플러그인의 설정 로드 (없으면 종료)
        let config = match self.get_or_load_config(source_dir, plugin)? {
            Some(c) => c,
            None => return Ok(()),
        };

        // 2. 해당 리소스의 의존성 맵 추출 (없으면 종료)
        let r_type_plural = r_type.to_plural();
        let deps_map = match config.types.get(r_type_plural).and_then(|t| t.get(name)) {
            Some(m) => m,
            None => return Ok(()),
        };

        for (dep_type_plural, dep_ids) in deps_map {
            // 3. 의존성 타입 유효성 확인
            let dt = match ResourceType::from_plural(dep_type_plural) {
                Some(t) => t,
                None => {
                    errors.push(format!(
                        "Unknown resource type '{}' in requirements.yaml for {} '{}'",
                        dep_type_plural, r_type, name
                    ));
                    continue;
                }
            };

            for dep_id in dep_ids {
                // 4. 의존성 ID 형식 확인 ("plugin:name")
                let parts: Vec<&str> = dep_id.split(':').collect();
                if parts.len() != 2 {
                    errors.push(format!(
                        "Invalid dependency ID '{}' in {} '{}' (Expected 'plugin:name')",
                        dep_id, r_type, name
                    ));
                    continue;
                }

                let dep_plugin = parts[0];
                let dep_name = parts[1];

                // 5. 실제 존재 여부 확인
                if !registry.contains_by_id(dt, dep_plugin, dep_name) {
                    errors.push(format!(
                        "{} '{}:{}' requires {} '{}' but it is missing in {}",
                        r_type, plugin, name, dt, dep_id, CONFIG_FILE_NAME
                    ));
                }
            }
        }

        Ok(())
    }

    /// 플러그인의 requirements.yaml 파일을 로드하거나 캐시에서 가져옵니다.
    fn get_or_load_config(&self, source_dir: &Path, plugin: &str) -> Result<Option<DependencyConfig>> {
        if let Some(cached) = self.cache.borrow().get(plugin) {
            return Ok(cached.clone());
        }

        let deps_path = source_dir.join(plugin).join(DEPS_FILE_NAME);
        let config = if deps_path.exists() {
            let content = fs::read_to_string(&deps_path)?;
            let parsed: DependencyConfig = serde_yaml::from_str(&content)?;
            Some(parsed)
        } else {
            None
        };

        self.cache.borrow_mut().insert(plugin.to_string(), config.clone());
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{ResourceData, SkillData};
    use serde_json::Value;
    use std::path::PathBuf;

    fn mock_resource(name: &str, plugin: &str, r_type: ResourceType) -> Resource {
        let data = ResourceData {
            name: name.to_string(),
            plugin: plugin.to_string(),
            content: String::new(),
            metadata: Value::Null,
            source_path: PathBuf::from("mock/path"),
        };
        match r_type {
            ResourceType::Command => Resource::Command(data),
            ResourceType::Agent => Resource::Agent(data),
            ResourceType::Skill => Resource::Skill(SkillData {
                base: data,
                extras: Vec::new(),
            }),
        }
    }

    #[test]
    fn test_parse_dependency_config() {
        let yaml = r#"
agents:
  skill-writer:
    skills:
      - general:guidelines
"#;
        let config: DependencyConfig = serde_yaml::from_str(yaml).unwrap();
        assert!(config.types.contains_key("agents"));
        let agents = config.types.get("agents").unwrap();
        assert!(agents.contains_key("skill-writer"));
        let deps = agents.get("skill-writer").unwrap();
        assert!(deps.contains_key("skills"));
        let skills = deps.get("skills").unwrap();
        assert_eq!(skills[0], "general:guidelines");
    }

    #[test]
    fn test_dependency_checker_empty_registry() {
        let registry = Registry::new();
        let checker = DependencyChecker::new();
        let source_dir = Path::new("non_existent_path");
        assert!(checker.check_dependencies(&registry, source_dir).is_ok());
    }

    #[test]
    fn test_dependency_checker_full_success() {
        let temp = tempfile::tempdir().unwrap();
        let source_dir = temp.path().join("atb_test_source");
        let p1_dir = source_dir.join("p1");
        fs::create_dir_all(&p1_dir).unwrap();

        let deps_yaml = r#"
agents:
  a1:
    skills:
      - p2:s1
"#;
        fs::write(p1_dir.join("requirements.yaml"), deps_yaml).unwrap();

        let mut registry = Registry::new();
        registry
            .register(mock_resource("a1", "p1", ResourceType::Agent))
            .unwrap();
        registry
            .register(mock_resource("s1", "p2", ResourceType::Skill))
            .unwrap();

        let checker = DependencyChecker::new();
        assert!(checker.check_dependencies(&registry, &source_dir).is_ok());
    }

    #[test]
    fn test_dependency_checker_missing_dep() {
        let temp = tempfile::tempdir().unwrap();
        let source_dir = temp.path().join("atb_test_source_missing");
        let p1_dir = source_dir.join("p1");
        fs::create_dir_all(&p1_dir).unwrap();

        let deps_yaml = r#"
agents:
  a1:
    skills:
      - p2:s1
"#;
        fs::write(p1_dir.join("requirements.yaml"), deps_yaml).unwrap();

        let mut registry = Registry::new();
        registry
            .register(mock_resource("a1", "p1", ResourceType::Agent))
            .unwrap();
        // s1 is missing!

        let checker = DependencyChecker::new();
        let result = checker.check_dependencies(&registry, &source_dir);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Dependency check failed:"));
        assert!(err_msg.contains("agent 'p1:a1' requires skill 'p2:s1' but it is missing in toolkit.yaml"));
    }

    #[test]
    fn test_dependency_checker_invalid_id() {
        let temp = tempfile::tempdir().unwrap();
        let source_dir = temp.path().join("atb_test_source_invalid");
        let p1_dir = source_dir.join("p1");
        fs::create_dir_all(&p1_dir).unwrap();

        let deps_yaml = r#"
agents:
  a1:
    skills:
      - p2_s1_missing_colon
"#;
        fs::write(p1_dir.join("requirements.yaml"), deps_yaml).unwrap();

        let mut registry = Registry::new();
        registry
            .register(mock_resource("a1", "p1", ResourceType::Agent))
            .unwrap();

        let checker = DependencyChecker::new();
        let result = checker.check_dependencies(&registry, &source_dir);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid dependency ID"));
    }
}
