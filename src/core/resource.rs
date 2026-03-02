use crate::core::SKILL_MD;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use std::path::PathBuf;

/// 리소스 타입 정의
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    Command,
    Agent,
    Skill,
}

impl ResourceType {
    /// 복수형 문자열(예: "agents", "skills")로부터 ResourceType을 생성합니다.
    pub fn from_plural(s: &str) -> Option<Self> {
        match s {
            "commands" => Some(ResourceType::Command),
            "agents" => Some(ResourceType::Agent),
            "skills" => Some(ResourceType::Skill),
            _ => None,
        }
    }

    /// 현재 타입을 복수형 문자열로 반환합니다.
    pub fn to_plural(&self) -> &'static str {
        match self {
            ResourceType::Command => "commands",
            ResourceType::Agent => "agents",
            ResourceType::Skill => "skills",
        }
    }
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ResourceType::Command => "command",
            ResourceType::Agent => "agent",
            ResourceType::Skill => "skill",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceData {
    pub name: String,
    pub plugin: String,
    pub content: String,
    pub metadata: Value,
    /// 원본 소스 위치 (Command/Agent: .md 파일 경로, Skill: 디렉터리 경로)
    pub source_path: PathBuf,
}

/// 스킬을 위한 확장 데이터
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SkillData {
    pub base: ResourceData,
    pub extras: Vec<ExtraFile>,
}

/// 추가로 복사되어야 하는 파일 정보
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExtraFile {
    /// 원본 파일 경로
    pub source: PathBuf,
    /// 대상 상대 경로 (예: skills/my_skill/extra.txt)
    pub target: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Resource {
    Command(ResourceData),
    Agent(ResourceData),
    Skill(SkillData),
}

impl Resource {
    pub fn r_type(&self) -> ResourceType {
        match self {
            Resource::Command(_) => ResourceType::Command,
            Resource::Agent(_) => ResourceType::Agent,
            Resource::Skill(_) => ResourceType::Skill,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Resource::Command(d) | Resource::Agent(d) => &d.name,
            Resource::Skill(s) => &s.base.name,
        }
    }

    pub fn plugin(&self) -> &str {
        match self {
            Resource::Command(d) | Resource::Agent(d) => &d.plugin,
            Resource::Skill(s) => &s.base.plugin,
        }
    }

    pub fn extras(&self) -> Vec<ExtraFile> {
        match self {
            Resource::Skill(s) => s.extras.clone(),
            _ => Vec::new(),
        }
    }

    /// 리소스의 메인 소스 마크다운 파일 경로를 반환합니다.
    pub fn main_source_path(&self) -> PathBuf {
        match self {
            Resource::Command(d) | Resource::Agent(d) => d.source_path.clone(),
            Resource::Skill(s) => s.base.source_path.join(SKILL_MD),
        }
    }

    /// 리소스의 메타데이터를 반환합니다.
    pub fn metadata(&self) -> &Value {
        match self {
            Resource::Command(d) | Resource::Agent(d) => &d.metadata,
            Resource::Skill(s) => &s.base.metadata,
        }
    }

    /// 리소스의 본문 내용을 반환합니다.
    pub fn content(&self) -> &str {
        match self {
            Resource::Command(d) | Resource::Agent(d) => &d.content,
            Resource::Skill(s) => &s.base.content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_type_plural_conversion() {
        assert_eq!(ResourceType::Command.to_plural(), "commands");
        assert_eq!(ResourceType::Agent.to_plural(), "agents");
        assert_eq!(ResourceType::Skill.to_plural(), "skills");

        assert_eq!(ResourceType::from_plural("commands"), Some(ResourceType::Command));
        assert_eq!(ResourceType::from_plural("agents"), Some(ResourceType::Agent));
        assert_eq!(ResourceType::from_plural("skills"), Some(ResourceType::Skill));
        assert_eq!(ResourceType::from_plural("unknown"), None);
    }
}
