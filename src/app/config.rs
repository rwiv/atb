use serde::Deserialize;
use std::fs;
use std::path::Path;

use crate::core::BuildTarget;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Resources {
    pub commands: Option<Vec<String>>,
    pub agents: Option<Vec<String>>,
    pub skills: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    pub source: String,
    pub target: BuildTarget,
    pub exclude: Option<Vec<String>>,
    pub resources: Resources,
}

/// atb.yaml 파일을 읽어 Config 구조체로 파싱합니다.
pub fn load_config<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
    let content = fs::read_to_string(path)?;
    parse_config(&content)
}

/// 문자열로부터 Config 구조체를 파싱합니다. (테스트 용이성을 위해 분리)
pub fn parse_config(content: &str) -> anyhow::Result<Config> {
    let mut config: Config = serde_yaml::from_str(content)?;
    config.source = shellexpand::tilde(&config.source).into_owned();
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_config_parsing() {
        let yaml = r#"
source: /absolute/path/to/source
target: gemini-cli
exclude:
  - "*.tmp"
resources:
  commands:
    - p1:cmd1
  agents:
    - p1:agent1
  skills:
    - p1:skill1
"#;
        let config = parse_config(yaml).unwrap();
        assert_eq!(config.source, "/absolute/path/to/source");
        assert_eq!(config.target, BuildTarget::GeminiCli);
        assert_eq!(config.exclude.unwrap(), vec!["*.tmp"]);
        let res = config.resources;
        assert_eq!(res.commands.unwrap(), vec!["p1:cmd1"]);
        assert_eq!(res.agents.unwrap(), vec!["p1:agent1"]);
        assert_eq!(res.skills.unwrap(), vec!["p1:skill1"]);
    }

    #[test]
    fn test_invalid_target() {
        let yaml = "source: /src
target: unknown-agent
resources: {}";
        let result = parse_config(yaml);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("unknown variant"));
    }

    #[test]
    fn test_optional_fields_missing() {
        let yaml = r#"
source: /src
target: claude-code
resources:
  commands:
    - p1:cmd1
"#;
        let config = parse_config(yaml).unwrap();
        assert_eq!(config.source, "/src");
        assert_eq!(config.target, BuildTarget::ClaudeCode);
        assert!(config.exclude.is_none());
        assert!(config.resources.agents.is_none());
        assert!(config.resources.skills.is_none());
        assert_eq!(config.resources.commands.unwrap(), vec!["p1:cmd1"]);
    }

    #[test]
    fn test_missing_required_target() {
        let yaml = "source: /src
resources: {}";
        let result = parse_config(yaml);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("missing field `target`"));
    }

    #[test]
    fn test_missing_required_source() {
        let yaml = "target: gemini-cli
resources: {}";
        let result = parse_config(yaml);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("missing field `source`"));
    }

    #[test]
    fn test_path_expansion() {
        // '~' 문자가 포함된 경로가 확장되는지 확인
        // 실제 홈 디렉터리 경로는 환경마다 다르므로, '~'가 그대로 남아있지 않은지 확인
        let yaml = r#"
source: ~/atb-source
target: gemini-cli
resources: {}
"#;
        let config = parse_config(yaml).unwrap();
        assert_ne!(config.source, "~/atb-source");
        assert!(config.source.contains("atb-source"));
        // 절대 경로 형식이 되어야 함 (Unix 기준 / 또는 Windows 기준 \ 혹은 드라이브 문자)
        assert!(config.source.starts_with('/') || config.source.contains(':') || config.source.starts_with('\\'));
    }
}
