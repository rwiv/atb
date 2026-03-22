use assert_cmd::Command;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_e2e_sync_gemini_cli() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    setup_fixtures(root);

    let config = format!(
        r#"
source: {}
target: gemini-cli
resources:
  commands:
    - plugin_a:foo
  skills:
    - plugin_c:python_expert
"#,
        root.display()
    );
    fs::write(root.join("atb.yaml"), config).unwrap();

    // 1. Initial Build
    let mut cmd = Command::new(assert_cmd::cargo_bin!("atb"));
    cmd.arg("build").arg("--config").arg(root.join("atb.yaml"));
    cmd.assert().success();

    // 2. Modify target files
    let cmd_toml_path = root.join("commands/foo.toml");
    let mut toml_content = fs::read_to_string(&cmd_toml_path).unwrap();
    toml_content = toml_content.replace("Foo command description", "Updated description");
    toml_content = toml_content.replace("prompt = \"# Foo Command\"", "prompt = \"# Updated Command Content\"");
    fs::write(&cmd_toml_path, toml_content).unwrap();

    // Modify skill extra file
    let extra_file_path = root.join("skills/python_expert/extra.txt");
    fs::write(&extra_file_path, "New Extra Content").unwrap();

    // Add new file to skill
    let new_file_path = root.join("skills/python_expert/new.txt");
    fs::write(&new_file_path, "Added File Content").unwrap();

    // 3. Run Sync
    let mut cmd = Command::new(assert_cmd::cargo_bin!("atb"));
    cmd.arg("sync").arg("--config").arg(root.join("atb.yaml"));
    cmd.assert().success();

    // 4. Verify Source
    let source_md_path = root.join("plugins/plugin_a/commands/foo.md");
    let source_md_content = fs::read_to_string(source_md_path).unwrap();
    assert!(source_md_content.contains("description: Updated description"));
    assert!(source_md_content.contains("# Updated Command Content"));

    let source_extra_path = root.join("plugins/plugin_c/skills/python_expert/extra.txt");
    assert_eq!(fs::read_to_string(source_extra_path).unwrap(), "New Extra Content");

    let source_new_path = root.join("plugins/plugin_c/skills/python_expert/new.txt");
    assert_eq!(fs::read_to_string(source_new_path).unwrap(), "Added File Content");
}

#[test]
fn test_e2e_sync_claude_code() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    setup_fixtures(root);

    let config = format!(
        r#"
source: {}
target: claude-code
resources:
  commands:
    - plugin_a:foo
"#,
        root.display()
    );
    fs::write(root.join("atb.yaml"), config).unwrap();

    // 1. Initial Build
    let mut cmd = Command::new(assert_cmd::cargo_bin!("atb"));
    cmd.arg("build").arg("--config").arg(root.join("atb.yaml"));
    cmd.assert().success();

    // 2. Modify target files
    let cmd_md_path = root.join("commands/foo.md");
    let mut md_content = fs::read_to_string(&cmd_md_path).unwrap();
    md_content = md_content.replace("description: Foo command description", "description: Claude Updated");
    md_content = md_content.replace("# Foo Command", "# Claude Content Updated");
    fs::write(&cmd_md_path, md_content).unwrap();

    // 3. Run Sync
    let mut cmd = Command::new(assert_cmd::cargo_bin!("atb"));
    cmd.arg("sync").arg("--config").arg(root.join("atb.yaml"));
    cmd.assert().success();

    // 4. Verify Source
    let source_md_path = root.join("plugins/plugin_a/commands/foo.md");
    let source_md_content = fs::read_to_string(source_md_path).unwrap();
    assert!(source_md_content.contains("description: Claude Updated"));
    assert!(source_md_content.contains("# Claude Content Updated"));
}

#[test]
fn test_e2e_sync_exclude_patterns() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    setup_fixtures(root);

    let config = format!(
        r#"
source: {}
target: gemini-cli
exclude:
  - "*.tmp"
resources:
  skills:
    - plugin_c:python_expert
"#,
        root.display()
    );
    fs::write(root.join("atb.yaml"), config).unwrap();

    // 1. Initial Build
    let mut cmd = Command::new(assert_cmd::cargo_bin!("atb"));
    cmd.arg("build").arg("--config").arg(root.join("atb.yaml"));
    cmd.assert().success();

    // 2. Add excluded file to target
    let tmp_file_path = root.join("skills/python_expert/test.tmp");
    fs::write(&tmp_file_path, "Should be ignored").unwrap();

    // 3. Run Sync
    let mut cmd = Command::new(assert_cmd::cargo_bin!("atb"));
    cmd.arg("sync").arg("--config").arg(root.join("atb.yaml"));
    cmd.assert().success();

    // 4. Verify Source (should NOT contain the .tmp file)
    let source_tmp_path = root.join("plugins/plugin_c/skills/python_expert/test.tmp");
    assert!(!source_tmp_path.exists());
}

fn setup_fixtures(root: &Path) {
    let plugins = root.join("plugins");
    let plugin_a_cmds = plugins.join("plugin_a/commands");
    let plugin_c_skills = plugins.join("plugin_c/skills/python_expert");

    fs::create_dir_all(&plugin_a_cmds).unwrap();
    fs::create_dir_all(&plugin_c_skills).unwrap();

    // Frontmatter with description
    fs::write(
        plugin_a_cmds.join("foo.md"),
        "---
description: Foo command description
---
# Foo Command",
    )
    .unwrap();

    fs::write(
        plugin_a_cmds.join("foo.yaml"),
        r#"
gemini-cli:
  model: gemini-1.5-pro
claude-code:
  model: claude-3-opus
"#,
    )
    .unwrap();

    fs::write(plugin_c_skills.join("SKILL.md"), "Python Expert Content").unwrap();
    fs::write(plugin_c_skills.join("extra.txt"), "Original Extra Content").unwrap();
}
