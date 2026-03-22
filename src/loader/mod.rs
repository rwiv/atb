pub mod merger;
pub mod model;
pub mod parser;
pub mod registry;
pub mod resolver;

pub use model::*;

use crate::core::{BuildTarget, FileFilter, MetadataMap, OVERRIDES_FILE_NAME, Resource};
use anyhow::Result;
use glob::Pattern;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use self::parser::ResourceParser;
use self::resolver::ResourcePathResolver;

/// 소스 디렉터리를 탐색하고 리소스를 로드하는 객체입니다.
pub struct ResourceLoader {
    root: PathBuf,
    filter: FileFilter,
    resolver: ResourcePathResolver,
    parser: ResourceParser,
    exclude_patterns: Vec<Pattern>,
}

impl ResourceLoader {
    /// 새로운 ResourceLoader 인스턴스를 생성합니다.
    pub fn new<P: AsRef<Path>>(source_root: P, exclude_patterns: Vec<Pattern>, target: BuildTarget) -> Result<Self> {
        let source_root = source_root.as_ref().to_path_buf();
        if !source_root.exists() {
            anyhow::bail!("Source root directory not found: {:?}", source_root);
        }

        let filter = FileFilter::new();
        let resolver = ResourcePathResolver::new();

        // overrides.yaml 로드
        let map_path = source_root.join(OVERRIDES_FILE_NAME);
        let metadata_map = Self::load_metadata_map(&map_path).ok();

        let parser = ResourceParser::new(target, metadata_map);

        Ok(Self {
            root: source_root,
            filter,
            resolver,
            parser,
            exclude_patterns,
        })
    }

    /// 리소스를 로드합니다.
    pub fn load(&self) -> Result<Vec<Resource>> {
        let files = self.scan()?;
        let scanned_resources = self.resolver.resolve(&self.root, files)?;

        scanned_resources
            .into_iter()
            .map(|scanned| self.parser.parse_resource(scanned))
            .collect()
    }

    /// 플러그인 디렉터리를 스캔하여 유효한 파일 경로 목록을 반환합니다.
    fn scan(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for entry in WalkDir::new(&self.root).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if self.filter.is_valid(&self.root, path, &self.exclude_patterns)? {
                files.push(path.to_path_buf());
            }
        }

        Ok(files)
    }

    /// overrides.yaml 파일을 로드하여 MetadataMap 객체로 변환합니다.
    fn load_metadata_map(path: &Path) -> Result<MetadataMap> {
        if !path.exists() {
            return Ok(MetadataMap::default());
        }
        let content = fs::read_to_string(path)?;
        let map: MetadataMap = serde_yaml::from_str(&content)?;
        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_resource_loader_load_integration() -> Result<()> {
        let dir = tempdir()?;
        let source_root = dir.path();

        // 샘플 구조 생성 (플러그인이 루트 하위에 바로 위치)
        let cmd_dir = source_root.join("plugin_a/commands");
        let skill_dir = source_root.join("plugin_b/skills/my_skill");
        fs::create_dir_all(&cmd_dir)?;
        fs::create_dir_all(&skill_dir)?;

        // Command: md + yaml
        fs::write(cmd_dir.join("foo.md"), "# Foo Content")?;
        fs::write(cmd_dir.join("foo.yaml"), "gemini-cli:\n  key: val")?;
        // Exclude 대상
        fs::write(cmd_dir.join("test.tmp"), "temp")?;

        // Skill: SKILL.yaml + md
        fs::write(skill_dir.join("SKILL.yaml"), "gemini-cli:\n  desc: skill")?;
        fs::write(skill_dir.join("SKILL.md"), "prompt")?;

        let patterns = vec![Pattern::new("*.tmp")?];
        let loader = ResourceLoader::new(source_root, patterns, BuildTarget::GeminiCli)?;
        let resources = loader.load()?;

        assert_eq!(resources.len(), 2);

        let mut found_foo = false;
        let mut found_skill = false;

        for res in resources {
            match res {
                Resource::Command(d) if d.name == "foo" => {
                    assert_eq!(d.plugin, "plugin_a");
                    assert_eq!(d.content, "# Foo Content");
                    assert_eq!(d.metadata["key"], "val");
                    found_foo = true;
                }
                Resource::Skill(s) if s.base.name == "my_skill" => {
                    assert_eq!(s.base.plugin, "plugin_b");
                    assert_eq!(s.base.metadata["desc"], "skill");
                    assert!(s.base.content.contains("prompt"));
                    found_skill = true;
                }
                _ => {}
            }
        }

        assert!(found_foo);
        assert!(found_skill);

        Ok(())
    }
}
