use crate::core::{
    AGENTS_MD, BuildTarget, CODEX_CONFIG_FILE_NAME, DIR_AGENTS, DIR_CODEX, DIR_PROMPTS, DIR_SKILLS, EXT_MD, EXT_TOML,
    Resource, ResourceData, ResourceType, SKILL_MD, TransformedFile,
};
use crate::transformer::Transformer;
use crate::transformer::default::DefaultTransformer;
use crate::utils::toml::json_to_toml;
use anyhow::{Result, anyhow};
use std::path::PathBuf;

pub struct CodexTransformer;

impl Transformer for CodexTransformer {
    fn transform(&self, resource: &Resource) -> Result<TransformedFile> {
        match resource {
            Resource::Command(data) => {
                // Commands: Markdown 포맷 유지, prompts/ 디렉터리 사용
                let default_transformer = DefaultTransformer {
                    target: BuildTarget::Codex,
                };
                let mut transformed = default_transformer.transform(resource)?;
                transformed.path = PathBuf::from(DIR_PROMPTS).join(format!("{}{}", data.name, EXT_MD));
                Ok(transformed)
            }
            Resource::Agent(data) => self.transform_agent_to_toml(data),
            Resource::Skill(_) => {
                // Skills: 기본 스킬 변환 로직 적용
                let default_transformer = DefaultTransformer {
                    target: BuildTarget::Codex,
                };
                default_transformer.transform(resource)
            }
        }
    }

    fn transform_root_prompt(&self, content: &str) -> Result<TransformedFile> {
        // AGENTS.md
        Ok(TransformedFile {
            path: PathBuf::from(AGENTS_MD),
            content: content.to_string(),
        })
    }

    fn post_transform(&self, resources: &[&Resource]) -> Result<Vec<TransformedFile>> {
        let mut agents_table = toml::Table::new();

        for res in resources {
            let Resource::Agent(data) = res else {
                continue;
            };

            let mut agent_config = toml::Table::new();

            let description = data
                .metadata
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            agent_config.insert("description".to_string(), toml::Value::String(description));
            agent_config.insert(
                "config_file".to_string(),
                toml::Value::String(format!("agents/{}{}", data.name, EXT_TOML)),
            );

            agents_table.insert(data.name.clone(), toml::Value::Table(agent_config));
        }

        if agents_table.is_empty() {
            return Ok(vec![]);
        }

        let mut root_table = toml::Table::new();
        root_table.insert("agents".to_string(), toml::Value::Table(agents_table));

        let content = toml::to_string_pretty(&root_table)?;
        let path = PathBuf::from(DIR_CODEX).join(CODEX_CONFIG_FILE_NAME);

        Ok(vec![TransformedFile { path, content }])
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
                // prompts/ 내의 .md 파일을 ResourceData로 복원
                let default_transformer = DefaultTransformer {
                    target: BuildTarget::Codex,
                };
                default_transformer.detransform(r_type, name, file_content, output_dir)
            }
            ResourceType::Agent => {
                // agents/ 내의 .toml 파일을 ResourceData로 복원
                let mut table: toml::Table = toml::from_str(file_content)?;
                let prompt = table
                    .remove("developer_instructions")
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .ok_or_else(|| anyhow!("Missing 'developer_instructions' field in Codex agent TOML"))?;

                let mut metadata = serde_json::to_value(table)?;

                // .codex/config.toml 파싱하여 description 복원
                let config_path = output_dir.join(DIR_CODEX).join(CODEX_CONFIG_FILE_NAME);
                if config_path.exists()
                    && let Ok(config_content) = std::fs::read_to_string(&config_path)
                    && let Ok(config_table) = toml::from_str::<toml::Table>(&config_content)
                    && let Some(agents) = config_table.get("agents").and_then(|a| a.as_table())
                    && let Some(agent) = agents.get(name).and_then(|a| a.as_table())
                    && let Some(desc) = agent.get("description").and_then(|d| d.as_str())
                    && let Some(obj) = metadata.as_object_mut()
                {
                    obj.insert("description".to_string(), serde_json::Value::String(desc.to_string()));
                }

                Ok(ResourceData {
                    name: name.to_string(),
                    plugin: String::new(),
                    content: prompt,
                    metadata,
                    source_path: PathBuf::new(),
                })
            }
            ResourceType::Skill => {
                let default_transformer = DefaultTransformer {
                    target: BuildTarget::Codex,
                };
                default_transformer.detransform(r_type, name, file_content, output_dir)
            }
        }
    }

    fn get_target_path(&self, r_type: ResourceType, name: &str) -> PathBuf {
        match r_type {
            ResourceType::Command => PathBuf::from(DIR_PROMPTS).join(format!("{}{}", name, EXT_MD)),
            ResourceType::Agent => PathBuf::from(DIR_AGENTS).join(format!("{}{}", name, EXT_TOML)),
            ResourceType::Skill => PathBuf::from(DIR_SKILLS).join(name).join(SKILL_MD),
        }
    }
}

impl CodexTransformer {
    fn transform_agent_to_toml(&self, data: &ResourceData) -> Result<TransformedFile> {
        // Metadata를 TOML Value로 변환 후 Table로 캐스팅
        let json_value = &data.metadata;
        let toml_value = json_to_toml(json_value)?;

        let mut table = match toml_value {
            toml::Value::Table(mut t) => {
                // description은 config.toml로 분리되므로 개별 파일에서는 제거합니다.
                t.remove("description");
                t
            }
            _ => {
                return Err(anyhow!("Metadata must be an object for Codex agent conversion"));
            }
        };

        // Markdown content를 'developer_instructions' 필드에 추가
        let mut prompt = data.content.clone();
        if prompt.contains('\n') && !prompt.ends_with('\n') {
            prompt.push('\n');
        }
        table.insert("developer_instructions".to_string(), toml::Value::String(prompt));

        // TOML 직렬화
        let content = toml::to_string_pretty(&table)?;

        // 경로 설정: agents/NAME.toml
        let path = PathBuf::from(DIR_AGENTS).join(format!("{}{}", data.name, EXT_TOML));

        Ok(TransformedFile { path, content })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ResourceData;
    use serde_json::json;
    use toml::Table;

    #[test]
    fn test_codex_command_transformation() {
        let transformer = CodexTransformer;
        let resource = Resource::Command(ResourceData {
            name: "test-cmd".to_string(),
            plugin: "test-plugin".to_string(),
            content: "Hello Codex".to_string(),
            metadata: json!({
                "description": "A test command"
            }),
            source_path: PathBuf::from("src/test.md"),
        });

        let result = transformer.transform(&resource).unwrap();
        assert_eq!(
            result.path,
            PathBuf::from(DIR_PROMPTS).join(format!("test-cmd{}", EXT_MD))
        );
        assert!(result.content.contains("description: A test command"));
        assert!(result.content.contains("Hello Codex"));
    }

    #[test]
    fn test_codex_agent_transformation() {
        let transformer = CodexTransformer;
        let resource = Resource::Agent(ResourceData {
            name: "test-agent".to_string(),
            plugin: "test-plugin".to_string(),
            content: "You are a codex agent.".to_string(),
            metadata: json!({
                "model": "codex-latest",
                "temperature": 0.3,
                "description": "Agent description"
            }),
            source_path: PathBuf::from("src/test.md"),
        });

        let result = transformer.transform(&resource).unwrap();
        assert_eq!(
            result.path,
            PathBuf::from(DIR_AGENTS).join(format!("test-agent{}", EXT_TOML))
        );

        let toml_val: Table = toml::from_str(&result.content).unwrap();
        assert_eq!(toml_val.get("model").unwrap().as_str().unwrap(), "codex-latest");
        assert_eq!(toml_val.get("temperature").unwrap().as_float().unwrap(), 0.3);
        assert!(toml_val.get("description").is_none());
        assert_eq!(
            toml_val.get("developer_instructions").unwrap().as_str().unwrap(),
            "You are a codex agent."
        );
    }

    #[test]
    fn test_codex_agent_multiline_transformation() {
        let transformer = CodexTransformer;
        let resource = Resource::Agent(ResourceData {
            name: "test-agent".to_string(),
            plugin: "test-plugin".to_string(),
            content: "Line 1\nLine 2".to_string(),
            metadata: json!({}),
            source_path: PathBuf::from("src/test.md"),
        });

        let result = transformer.transform(&resource).unwrap();
        let toml_val: Table = toml::from_str(&result.content).unwrap();
        assert_eq!(
            toml_val.get("developer_instructions").unwrap().as_str().unwrap(),
            "Line 1\nLine 2\n"
        );
    }

    #[test]
    fn test_codex_detransform_agent() {
        let transformer = CodexTransformer;
        let input = r#"model = "codex-001"
developer_instructions = "Agent Logic"
"#;

        let result = transformer
            .detransform(ResourceType::Agent, "test", input, std::path::Path::new(""))
            .unwrap();

        assert_eq!(result.content, "Agent Logic");
        assert_eq!(result.metadata["model"], "codex-001");
    }

    #[test]
    fn test_codex_post_transform() {
        let transformer = CodexTransformer;
        let r1 = Resource::Agent(ResourceData {
            name: "test-agent-1".to_string(),
            plugin: "test-plugin".to_string(),
            content: "Agent 1".to_string(),
            metadata: json!({
                "description": "Desc 1"
            }),
            source_path: PathBuf::from(""),
        });
        let r2 = Resource::Agent(ResourceData {
            name: "test-agent-2".to_string(),
            plugin: "test-plugin".to_string(),
            content: "Agent 2".to_string(),
            metadata: json!({}),
            source_path: PathBuf::from(""),
        });

        let resources = vec![&r1, &r2];
        let result = transformer.post_transform(&resources).unwrap();

        assert_eq!(result.len(), 1);
        let config_file = &result[0];
        assert_eq!(config_file.path, PathBuf::from(".codex/config.toml"));

        let root: Table = toml::from_str(&config_file.content).unwrap();
        let agents = root.get("agents").unwrap().as_table().unwrap();

        let a1 = agents.get("test-agent-1").unwrap().as_table().unwrap();
        assert_eq!(a1.get("description").unwrap().as_str().unwrap(), "Desc 1");
        assert_eq!(
            a1.get("config_file").unwrap().as_str().unwrap(),
            "agents/test-agent-1.toml"
        );

        let a2 = agents.get("test-agent-2").unwrap().as_table().unwrap();
        assert_eq!(a2.get("description").unwrap().as_str().unwrap(), "");
        assert_eq!(
            a2.get("config_file").unwrap().as_str().unwrap(),
            "agents/test-agent-2.toml"
        );
    }
}
