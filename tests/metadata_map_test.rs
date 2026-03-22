use anyhow::Result;
use atb::core::{BuildTarget, Resource};
use atb::loader::ResourceLoader;
use glob::Pattern;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_metadata_map_integration() -> Result<()> {
    let dir = tempdir()?;
    let source_root = dir.path();
    let plugins_dir = source_root.join("plugins");
    let cmd_dir = plugins_dir.join("p1/commands");
    fs::create_dir_all(&cmd_dir)?;

    // 1. map.yaml 작성
    let map_yaml = "
model:
  sonnet:
    gemini-cli: gemini-3.0-flash
    claude-code: sonnet-v3
";
    fs::write(source_root.join("map.yaml"), map_yaml)?;

    // 2. 리소스 작성 (Frontmatter 포함)
    let md_content = "---
name: my-cmd
model: sonnet
---
# Content";
    fs::write(cmd_dir.join("my-cmd.md"), md_content)?;

    // 3. ResourceLoader로 로드 (Gemini 타겟)
    let loader = ResourceLoader::new(source_root, Vec::<Pattern>::new(), BuildTarget::GeminiCli)?;
    let resources = loader.load()?;

    assert_eq!(resources.len(), 1);
    if let Resource::Command(d) = &resources[0] {
        assert_eq!(d.metadata["model"], "gemini-3.0-flash");
    } else {
        panic!("Expected Command resource");
    }

    // 4. ResourceLoader로 로드 (Claude 타겟)
    let loader = ResourceLoader::new(source_root, Vec::<Pattern>::new(), BuildTarget::ClaudeCode)?;
    let resources = loader.load()?;
    if let Resource::Command(d) = &resources[0] {
        assert_eq!(d.metadata["model"], "sonnet-v3");
    }

    Ok(())
}

#[test]
fn test_metadata_map_with_external_override() -> Result<()> {
    let dir = tempdir()?;
    let source_root = dir.path();
    let plugins_dir = source_root.join("plugins");
    let cmd_dir = plugins_dir.join("p1/commands");
    fs::create_dir_all(&cmd_dir)?;

    // 1. map.yaml 작성
    let map_yaml = "
model:
  sonnet:
    gemini-cli: gemini-3.0-flash
";
    fs::write(source_root.join("map.yaml"), map_yaml)?;

    // 2. 리소스 작성 (MD + External YAML)
    let md_content = "---
model: sonnet
---
# Content";
    fs::write(cmd_dir.join("my-cmd.md"), md_content)?;

    // External YAML should override Map result
    let ext_yaml = "
gemini-cli:
  model: gemini-override
";
    fs::write(cmd_dir.join("my-cmd.yaml"), ext_yaml)?;

    // 3. ResourceLoader로 로드
    let loader = ResourceLoader::new(source_root, Vec::<Pattern>::new(), BuildTarget::GeminiCli)?;
    let resources = loader.load()?;

    if let Resource::Command(d) = &resources[0] {
        // External YAML (gemini-override) should win over Map (gemini-3.0-flash)
        assert_eq!(d.metadata["model"], "gemini-override");
    }

    Ok(())
}
