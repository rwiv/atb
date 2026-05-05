pub mod dependency;
pub mod emitter;

use self::dependency::DependencyChecker;
use self::emitter::Emitter;
use crate::core::{AGENTS_MD, BuildTarget, ResourceType, TransformedResource};
use crate::loader::registry::Registry;
use crate::transformer::Transformer;
use crate::utils::yaml::extract_frontmatter;
use anyhow::Context;
use log::info;
use std::collections::BTreeSet;
use std::path::Path;

#[derive(Default)]
pub struct Builder;

impl Builder {
    pub fn new() -> Self {
        Self
    }

    pub fn run(
        &self,
        transformer: &dyn Transformer,
        registry: &Registry,
        source_dir: &Path,
        output_dir: &Path,
        target: BuildTarget,
        full_clean: bool,
    ) -> anyhow::Result<()> {
        if target == BuildTarget::Codex {
            self.validate_codex_name_collisions(registry)?;
        }

        // 의존성 검사
        let checker = DependencyChecker::new();
        checker.check_dependencies(registry, source_dir)?;

        let all_resources = registry.all_resources();
        let mut transformed_resources = Vec::new();
        for res in &all_resources {
            let transformed_file = transformer
                .transform(res)
                .with_context(|| format!("Failed to transform resource: {}", res.name()))?;

            transformed_resources.push(TransformedResource {
                files: vec![transformed_file],
                extras: res.extras(),
            });
        }

        // Post-transform hook
        let post_files = transformer.post_transform(&all_resources)?;
        if !post_files.is_empty() {
            transformed_resources.push(TransformedResource {
                files: post_files,
                extras: Vec::new(),
            });
        }

        // AGENTS.md 처리
        let agents_md_path = source_dir.join(AGENTS_MD);
        if agents_md_path.exists() {
            info!("  - Found root system prompt: {}", agents_md_path.display());
            let raw_content = std::fs::read_to_string(&agents_md_path)?;
            let (_fm, pure_content) = extract_frontmatter(&raw_content);
            let transformed_file = transformer.transform_root_prompt(&pure_content)?;

            transformed_resources.push(TransformedResource {
                files: vec![transformed_file],
                extras: Vec::new(),
            });
        }

        info!("Emitting files to {}...", output_dir.display());
        let emitter = Emitter::new(output_dir, target);
        if full_clean {
            emitter.clean_all()?;
        } else {
            emitter.clean(&transformed_resources)?;
        }
        emitter.emit(&transformed_resources)?;

        info!("Build successful!");

        Ok(())
    }

    fn validate_codex_name_collisions(&self, registry: &Registry) -> anyhow::Result<()> {
        let mut command_names = BTreeSet::new();
        let mut skill_names = BTreeSet::new();

        for resource in registry.all_resources() {
            match resource.r_type() {
                ResourceType::Command => {
                    command_names.insert(resource.name());
                }
                ResourceType::Skill => {
                    skill_names.insert(resource.name());
                }
                ResourceType::Agent => {}
            }
        }

        let collisions = command_names
            .intersection(&skill_names)
            .map(|name| format!("'{}'", name))
            .collect::<Vec<_>>();

        if !collisions.is_empty() {
            anyhow::bail!(
                "Codex 타겟에서 command와 skill의 이름이 충돌합니다: {}",
                collisions.join(", ")
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Resource, ResourceData, SkillData};
    use serde_json::Value;
    use std::path::PathBuf;

    fn make_resource(name: &str, r_type: ResourceType) -> Resource {
        let data = ResourceData {
            name: name.to_string(),
            plugin: "plugin".to_string(),
            content: String::new(),
            metadata: Value::Null,
            source_path: PathBuf::from("source.md"),
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
    fn test_codex_name_collision_validation_fails_on_collision() {
        let builder = Builder::new();
        let mut registry = Registry::new();
        registry.register(make_resource("foo", ResourceType::Command)).unwrap();
        registry.register(make_resource("foo", ResourceType::Skill)).unwrap();

        let err = builder
            .validate_codex_name_collisions(&registry)
            .unwrap_err()
            .to_string();

        assert!(err.contains("Codex 타겟에서 command와 skill의 이름이 충돌합니다"));
        assert!(err.contains("'foo'"));
    }

    #[test]
    fn test_codex_name_collision_validation_succeeds_without_collision() {
        let builder = Builder::new();
        let mut registry = Registry::new();
        registry.register(make_resource("foo", ResourceType::Command)).unwrap();
        registry.register(make_resource("bar", ResourceType::Skill)).unwrap();

        assert!(builder.validate_codex_name_collisions(&registry).is_ok());
    }
}
