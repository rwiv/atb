use crate::core::{
    BuildTarget, DIR_AGENTS, DIR_COMMANDS, DIR_SKILLS, EXT_YAML, EXT_YML, ExtraFile, MetadataMap, Resource,
    ResourceData, SkillData,
};
use crate::loader::ScannedResource;
use crate::loader::merger::MetadataMerger;
use crate::utils::yaml::extract_frontmatter;
use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

/// 리소스를 파싱하고 조립하는 객체입니다.
pub struct ResourceParser {
    pub target: BuildTarget,
    pub metadata_map: Option<MetadataMap>,
    merger: MetadataMerger,
}

impl ResourceParser {
    pub fn new(target: BuildTarget, metadata_map: Option<MetadataMap>) -> Self {
        Self {
            target,
            metadata_map,
            merger: MetadataMerger::new(target),
        }
    }

    /// 스캔된 리소스 정보로부터 Resource 객체를 생성합니다.
    pub fn parse_resource(&self, scanned: ScannedResource) -> Result<Resource> {
        let r_type = scanned.resource_type();
        let source_path = scanned.source_path()?;
        let (md_path, metadata_path, extras) = scanned.paths.unpack();

        // 1. Markdown 본문 및 Frontmatter 추출
        let (fm_metadata, pure_content) = if let Some(ref p) = md_path {
            let raw_content =
                fs::read_to_string(p).with_context(|| format!("Failed to read markdown content: {:?}", p))?;
            extract_frontmatter(&raw_content)
        } else {
            anyhow::bail!(
                "Markdown file is missing for resource '{}' in plugin '{}' ({})",
                scanned.name,
                scanned.plugin,
                r_type
            );
        };

        // 2. 외부 메타데이터 파일 파싱
        let ext_metadata = if let Some(ref p) = metadata_path {
            Some(self.parse_metadata(p, r_type, &scanned.name)?)
        } else {
            None
        };

        // 3. 메타데이터 병합 및 매핑 (MetadataMerger 사용)
        let final_metadata = self
            .merger
            .merge(&fm_metadata, ext_metadata.as_ref(), self.metadata_map.as_ref())
            .with_context(|| format!("Failed to merge metadata for resource: {}/{}", r_type, scanned.name))?;

        let data = ResourceData {
            name: scanned.name.clone(),
            plugin: scanned.plugin.clone(),
            content: pure_content,
            metadata: final_metadata,
            source_path,
        };

        match r_type {
            DIR_COMMANDS => Ok(Resource::Command(data)),
            DIR_AGENTS => Ok(Resource::Agent(data)),
            DIR_SKILLS => {
                // 스킬 루트(SKILL.md의 부모 디렉터리)를 기준으로 상대 경로 계산
                let skill_root = md_path.as_ref().and_then(|p| p.parent()).unwrap();

                let skill_extras = extras
                    .into_iter()
                    .map(|source| {
                        let relative_path = source.strip_prefix(skill_root).unwrap_or(&source);
                        let target = PathBuf::from(DIR_SKILLS).join(&scanned.name).join(relative_path);
                        ExtraFile { source, target }
                    })
                    .collect();

                Ok(Resource::Skill(SkillData {
                    base: data,
                    extras: skill_extras,
                }))
            }
            _ => unreachable!("Unknown resource type: {}", r_type),
        }
    }

    /// 파일 경로로부터 메타데이터를 파싱하여 serde_json::Value로 반환합니다.
    fn parse_metadata(&self, path: &Path, r_type: &str, name: &str) -> Result<Value> {
        let meta_str = fs::read_to_string(path).with_context(|| format!("Failed to read metadata file: {:?}", path))?;

        let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or_default();

        if extension == &EXT_YAML[1..] || extension == &EXT_YML[1..] {
            serde_yaml::from_str(&meta_str)
                .with_context(|| format!("Failed to parse YAML for resource: {}/{}", r_type, name))
        } else {
            anyhow::bail!(
                "Unsupported metadata extension '{}' for resource: {}/{}",
                extension,
                r_type,
                name
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loader::ScannedPaths;
    use tempfile::tempdir;
    #[test]
    fn test_parse_resource_with_frontmatter_and_external() -> Result<()> {
        let dir = tempdir()?;
        let md_path = dir.path().join("foo.md");
        let yaml_path = dir.path().join("foo.yaml");

        fs::write(
            &md_path,
            "---
name: fm-name
model: fm-model
---
# Content",
        )?;
        fs::write(
            &yaml_path,
            "gemini-cli:
  model: gemini-model",
        )?;

        let parser = ResourceParser::new(BuildTarget::GeminiCli, None);
        let scanned = ScannedResource {
            plugin: "p1".to_string(),
            name: "foo".to_string(),
            paths: ScannedPaths::Command {
                md: Some(md_path.clone()),
                metadata: Some(yaml_path),
            },
        };

        let res = parser.parse_resource(scanned)?;
        if let Resource::Command(d) = res {
            assert_eq!(d.name, "foo");
            assert_eq!(d.content, "# Content");
            assert_eq!(d.metadata["model"], "gemini-model");
            assert_eq!(d.source_path, md_path);
        } else {
            panic!("Expected Command resource");
        }
        Ok(())
    }

    #[test]
    fn test_parse_skill_with_extras() -> Result<()> {
        let dir = tempdir()?;
        let skill_dir = dir.path().join("p1/skills/my-skill");
        fs::create_dir_all(&skill_dir)?;

        let md_path = skill_dir.join("SKILL.md");
        let extra_path = skill_dir.join("logic.py");
        let nested_extra_path = skill_dir.join("ref/foo.md");
        fs::create_dir_all(skill_dir.join("ref"))?;

        fs::write(&md_path, "# Skill")?;
        fs::write(&extra_path, "print('hello')")?;
        fs::write(&nested_extra_path, "nested")?;

        let parser = ResourceParser::new(BuildTarget::GeminiCli, None);
        let scanned = ScannedResource {
            plugin: "p1".to_string(),
            name: "my-skill".to_string(),
            paths: ScannedPaths::Skill {
                md: Some(md_path),
                metadata: None,
                extras: vec![extra_path, nested_extra_path],
            },
        };

        let res = parser.parse_resource(scanned)?;
        if let Resource::Skill(s) = res {
            assert_eq!(s.base.name, "my-skill");
            assert_eq!(s.extras.len(), 2);
            assert_eq!(s.base.source_path, skill_dir);

            let targets: Vec<String> = s
                .extras
                .iter()
                .map(|e| e.target.to_str().unwrap().to_string())
                .collect();
            assert!(targets.contains(&"skills/my-skill/logic.py".to_string()));
            assert!(targets.contains(&"skills/my-skill/ref/foo.md".to_string()));
        } else {
            panic!("Expected Skill resource");
        }
        Ok(())
    }
}
