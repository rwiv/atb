use crate::core::{
    AGENTS_MD, BuildTarget, CLAUDE_MD, DIR_AGENTS, DIR_COMMANDS, DIR_SKILLS, EXT_MD, GEMINI_MD, Resource, ResourceData,
    ResourceType, SKILL_MD, TransformedFile,
};
use crate::transformer::Transformer;
use crate::utils::yaml::extract_frontmatter;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct DefaultFrontmatter {
    #[serde(flatten)]
    metadata: serde_json::Value,
}

pub struct DefaultTransformer {
    pub target: BuildTarget,
}

impl Transformer for DefaultTransformer {
    fn transform(&self, resource: &Resource) -> Result<TransformedFile> {
        let (data, folder) = match resource {
            Resource::Command(d) => (d, DIR_COMMANDS),
            Resource::Agent(d) => (d, DIR_AGENTS),
            Resource::Skill(s) => (&s.base, DIR_SKILLS),
        };

        let frontmatter = DefaultFrontmatter {
            metadata: data.metadata.clone(),
        };

        let yaml_frontmatter = serde_yaml::to_string(&frontmatter)?;
        let content = format!("---\n{}---\n\n{}", yaml_frontmatter, data.content);

        let path = if matches!(resource, Resource::Skill(_)) {
            PathBuf::from(folder).join(&data.name).join(SKILL_MD)
        } else {
            PathBuf::from(folder).join(format!("{}{}", data.name, EXT_MD))
        };

        Ok(TransformedFile { path, content })
    }

    fn transform_root_prompt(&self, content: &str) -> Result<TransformedFile> {
        let filename = match self.target {
            BuildTarget::GeminiCli => GEMINI_MD,
            BuildTarget::ClaudeCode => CLAUDE_MD,
            BuildTarget::OpenCode => AGENTS_MD,
            BuildTarget::Codex => AGENTS_MD,
        };

        Ok(TransformedFile {
            path: PathBuf::from(filename),
            content: content.to_string(),
        })
    }

    fn detransform(
        &self,
        _r_type: ResourceType,
        name: &str,
        file_content: &str,
        _output_dir: &std::path::Path,
    ) -> Result<ResourceData> {
        let (metadata, content) = extract_frontmatter(file_content);

        Ok(ResourceData {
            name: name.to_string(),
            plugin: String::new(), // detransform 시점에는 알 수 없음
            content,
            metadata,
            source_path: PathBuf::new(), // Syncer에서 보완 예정
        })
    }

    fn get_target_path(&self, r_type: ResourceType, name: &str) -> PathBuf {
        let folder = match r_type {
            ResourceType::Command => DIR_COMMANDS,
            ResourceType::Agent => DIR_AGENTS,
            ResourceType::Skill => DIR_SKILLS,
        };

        if matches!(r_type, ResourceType::Skill) {
            PathBuf::from(folder).join(name).join(SKILL_MD)
        } else {
            PathBuf::from(folder).join(format!("{}{}", name, EXT_MD))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{ResourceData, SkillData};
    use serde_json::json;

    #[test]
    fn test_default_transformation() {
        let transformer = DefaultTransformer {
            target: BuildTarget::ClaudeCode,
        };
        let resource = Resource::Command(ResourceData {
            name: "test-cmd".to_string(),
            plugin: "test-plugin".to_string(),
            content: "# Hello World".to_string(),
            metadata: json!({
                "description": "A test command",
                "model": "claude-3-opus"
            }),
            source_path: PathBuf::from("src/test.md"),
        });

        let result = transformer.transform(&resource).unwrap();
        assert_eq!(
            result.path,
            PathBuf::from(DIR_COMMANDS).join(format!("test-cmd{}", EXT_MD))
        );

        assert!(!result.content.contains("metadata:"));
        assert!(result.content.contains("description: A test command"));
        assert!(result.content.contains("model: claude-3-opus"));
        assert!(result.content.contains("# Hello World"));
        assert!(result.content.starts_with("---"));
    }

    #[test]
    fn test_default_skill_transformation() {
        let transformer = DefaultTransformer {
            target: BuildTarget::ClaudeCode,
        };
        let resource = Resource::Skill(SkillData {
            base: ResourceData {
                name: "test-skill".to_string(),
                plugin: "test-plugin".to_string(),
                content: "Skill Content".to_string(),
                metadata: json!({
                    "description": "Skill description",
                    "type": "expert"
                }),
                source_path: PathBuf::from("src/skill"),
            },
            extras: Vec::new(),
        });

        let result = transformer.transform(&resource).unwrap();
        assert_eq!(result.path, PathBuf::from(DIR_SKILLS).join("test-skill").join(SKILL_MD));
        assert!(!result.content.contains("metadata:"));
        assert!(result.content.contains("description: Skill description"));
        assert!(result.content.contains("type: expert"));
        assert!(result.content.contains("Skill Content"));
    }

    #[test]
    fn test_default_root_prompt_transformation() {
        let claude = DefaultTransformer {
            target: BuildTarget::ClaudeCode,
        };
        let opencode = DefaultTransformer {
            target: BuildTarget::OpenCode,
        };
        let gemini = DefaultTransformer {
            target: BuildTarget::GeminiCli,
        };

        assert_eq!(
            claude.transform_root_prompt("test").unwrap().path,
            PathBuf::from(CLAUDE_MD)
        );
        assert_eq!(
            opencode.transform_root_prompt("test").unwrap().path,
            PathBuf::from(AGENTS_MD)
        );
        assert_eq!(
            gemini.transform_root_prompt("test").unwrap().path,
            PathBuf::from(GEMINI_MD)
        );
    }

    #[test]
    fn test_default_detransform() {
        let transformer = DefaultTransformer {
            target: BuildTarget::ClaudeCode,
        };
        let input = "---
description: Updated description
model: new-model
---

# New Content";

        let result = transformer
            .detransform(ResourceType::Command, "cmd", input, std::path::Path::new(""))
            .unwrap();

        assert_eq!(result.content, "# New Content");
        assert_eq!(result.metadata["description"], "Updated description");
        assert_eq!(result.metadata["model"], "new-model");
    }
}
