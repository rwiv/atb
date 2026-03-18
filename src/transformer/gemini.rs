use crate::core::{
    BuildTarget, DIR_AGENTS, DIR_COMMANDS, DIR_SKILLS, EXT_MD, EXT_TOML, GEMINI_MD, Resource, ResourceData,
    ResourceType, SKILL_MD, TransformedFile,
};
use crate::transformer::Transformer;
use crate::transformer::default::DefaultTransformer;
use crate::utils::toml::json_to_toml;
use anyhow::{Result, anyhow};
use std::path::PathBuf;

pub struct GeminiTransformer;

impl Transformer for GeminiTransformer {
    fn transform(&self, resource: &Resource) -> Result<TransformedFile> {
        match resource {
            Resource::Command(data) => self.transform_command_to_toml(data),
            Resource::Agent(data) => {
                let mut new_data = data.clone();
                if new_data.metadata.get("tools").is_none()
                    && let Some(obj) = new_data.metadata.as_object_mut()
                {
                    obj.insert("tools".to_string(), serde_json::json!(["*"]));
                }
                let default_transformer = DefaultTransformer {
                    target: BuildTarget::GeminiCli,
                };
                default_transformer.transform(&Resource::Agent(new_data))
            }
            Resource::Skill(_) => {
                let default_transformer = DefaultTransformer {
                    target: BuildTarget::GeminiCli,
                };
                default_transformer.transform(resource)
            }
        }
    }

    fn transform_root_prompt(&self, content: &str) -> Result<TransformedFile> {
        // AGENTS.md -> GEMINI.md
        Ok(TransformedFile {
            path: PathBuf::from(GEMINI_MD),
            content: content.to_string(),
        })
    }

    fn detransform(
        &self,
        r_type: ResourceType,
        name: &str,
        file_content: &str,
        output_dir: &std::path::Path,
    ) -> Result<ResourceData> {
        match r_type {
            ResourceType::Command => {
                let mut table: toml::Table = toml::from_str(file_content)?;
                let prompt = table
                    .remove("prompt")
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .ok_or_else(|| anyhow!("Missing 'prompt' field in Gemini command TOML"))?;

                // TOML Table을 JSON Value로 변환
                let metadata = serde_json::to_value(table)?;

                Ok(ResourceData {
                    name: name.to_string(),
                    plugin: String::new(),
                    content: prompt.replace("{{args}}", "$ARGUMENTS"),
                    metadata,
                    source_path: PathBuf::new(),
                })
            }
            ResourceType::Agent | ResourceType::Skill => {
                let default_transformer = DefaultTransformer {
                    target: BuildTarget::GeminiCli,
                };
                default_transformer.detransform(r_type, name, file_content, output_dir)
            }
        }
    }

    fn get_target_path(&self, r_type: ResourceType, name: &str) -> PathBuf {
        match r_type {
            ResourceType::Command => PathBuf::from(DIR_COMMANDS).join(format!("{}{}", name, EXT_TOML)),
            ResourceType::Agent => PathBuf::from(DIR_AGENTS).join(format!("{}{}", name, EXT_MD)),
            ResourceType::Skill => PathBuf::from(DIR_SKILLS).join(name).join(SKILL_MD),
        }
    }
}

impl GeminiTransformer {
    fn transform_command_to_toml(&self, data: &ResourceData) -> Result<TransformedFile> {
        // 1. Metadata를 TOML Value로 변환 후 Table로 캐스팅
        let json_value = &data.metadata;
        let toml_value = json_to_toml(json_value)?;

        let mut table = match toml_value {
            toml::Value::Table(t) => t,
            _ => {
                return Err(anyhow!("Metadata must be an object for Gemini conversion"));
            }
        };

        // 2. Markdown content를 'prompt' 필드에 추가
        let mut prompt = data.content.replace("$ARGUMENTS", "{{args}}");
        if prompt.contains('\n') && !prompt.ends_with('\n') {
            prompt.push('\n');
        }
        table.insert("prompt".to_string(), toml::Value::String(prompt));

        // 3. TOML 직렬화
        let content = toml::to_string_pretty(&table)?;

        // 4. 경로 설정
        let path = PathBuf::from(DIR_COMMANDS).join(format!("{}{}", data.name, EXT_TOML));

        Ok(TransformedFile { path, content })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{ResourceData, SkillData};
    use serde_json::json;
    use toml::Table;

    #[test]
    fn test_gemini_command_transformation() {
        let transformer = GeminiTransformer;
        let resource = Resource::Command(ResourceData {
            name: "test-cmd".to_string(),
            plugin: "test-plugin".to_string(),
            content: "Hello World $ARGUMENTS".to_string(),
            metadata: json!({
                "model": "gemini-1.5-pro",
                "description": "A test command"
            }),
            source_path: PathBuf::from("src/test.md"),
        });

        let result = transformer.transform(&resource).unwrap();
        assert_eq!(
            result.path,
            PathBuf::from(DIR_COMMANDS).join(format!("test-cmd{}", EXT_TOML))
        );

        let toml_val: Table = toml::from_str(&result.content).unwrap();
        assert_eq!(toml_val.get("model").unwrap().as_str().unwrap(), "gemini-1.5-pro");
        assert_eq!(toml_val.get("description").unwrap().as_str().unwrap(), "A test command");
        assert_eq!(
            toml_val.get("prompt").unwrap().as_str().unwrap(),
            "Hello World {{args}}"
        );
    }

    #[test]
    fn test_gemini_command_multiline_prompt_formatting() {
        let transformer = GeminiTransformer;
        let resource = Resource::Command(ResourceData {
            name: "multiline-cmd".to_string(),
            plugin: "test-plugin".to_string(),
            content: "line1\nline2".to_string(),
            metadata: json!({}),
            source_path: PathBuf::from("src/test.md"),
        });

        let result = transformer.transform(&resource).unwrap();
        // Ensure the closing triple quotes are on a new line
        assert!(result.content.contains("line2\n\"\"\""));
    }

    #[test]
    fn test_gemini_skill_transformation() {
        let transformer = GeminiTransformer;
        let resource = Resource::Skill(SkillData {
            base: ResourceData {
                name: "test-skill".to_string(),
                plugin: "test-plugin".to_string(),
                content: "Skill Content".to_string(),
                metadata: json!({
                    "type": "expert"
                }),
                source_path: PathBuf::from("src/skill"),
            },
            extras: Vec::new(),
        });

        let result = transformer.transform(&resource).unwrap();
        assert_eq!(result.path, PathBuf::from(DIR_SKILLS).join("test-skill").join(SKILL_MD));
        assert!(!result.content.contains("metadata:"));
        assert!(result.content.contains("type: expert"));
        assert!(result.content.contains("Skill Content"));
    }

    #[test]
    fn test_gemini_agent_transformation() {
        let transformer = GeminiTransformer;
        let resource = Resource::Agent(ResourceData {
            name: "test-agent".to_string(),
            plugin: "test-plugin".to_string(),
            content: "Agent Content".to_string(),
            metadata: json!({
                "model": "gemini-1.5-flash"
            }),
            source_path: PathBuf::from("src/test.md"),
        });

        let result = transformer.transform(&resource).unwrap();
        assert_eq!(
            result.path,
            PathBuf::from(DIR_AGENTS).join(format!("test-agent{}", EXT_MD))
        );
        assert!(!result.content.contains("metadata:"));
        assert!(result.content.contains("model: gemini-1.5-flash"));
        assert!(result.content.contains("tools:\n- '*'"));
        assert!(result.content.contains("Agent Content"));
    }

    #[test]
    fn test_gemini_agent_transformation_with_existing_tools() {
        let transformer = GeminiTransformer;
        let resource = Resource::Agent(ResourceData {
            name: "test-agent-tools".to_string(),
            plugin: "test-plugin".to_string(),
            content: "Agent Content".to_string(),
            metadata: json!({
                "model": "gemini-1.5-flash",
                "tools": ["my-tool"]
            }),
            source_path: PathBuf::from("src/test.md"),
        });

        let result = transformer.transform(&resource).unwrap();
        assert!(result.content.contains("tools:\n- my-tool"));
        assert!(!result.content.contains("tools:\n- '*'"));
    }

    #[test]
    fn test_gemini_root_prompt_transformation() {
        let transformer = GeminiTransformer;
        let content = "# Global Instructions\nDo this and that.";
        let result = transformer.transform_root_prompt(content).unwrap();

        assert_eq!(result.path, PathBuf::from(GEMINI_MD));
        assert_eq!(result.content, content);
    }

    #[test]
    fn test_gemini_detransform_command() {
        let transformer = GeminiTransformer;
        let input = r#"description = "Updated desc"
model = "gemini-2.0"
prompt = "New Prompt {{args}}"
"#;

        let result = transformer
            .detransform(ResourceType::Command, "cmd", input, std::path::Path::new(""))
            .unwrap();

        assert_eq!(result.content, "New Prompt $ARGUMENTS");
        assert_eq!(result.metadata["description"], "Updated desc");
        assert_eq!(result.metadata["model"], "gemini-2.0");
    }
}
