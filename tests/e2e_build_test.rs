use assert_cmd::Command;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_e2e_build_gemini_cli() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    // Setup fixtures
    setup_fixtures(root);

    // Create toolkit.yaml
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
    fs::write(root.join("toolkit.yaml"), config).unwrap();

    // Run build
    let mut cmd = Command::new(assert_cmd::cargo_bin!("atb"));
    cmd.arg("build").arg("--config").arg(root.join("toolkit.yaml"));
    cmd.assert().success();

    // Verify outputs
    assert!(root.join("commands/foo.toml").exists());
    assert!(root.join("skills/python_expert/SKILL.md").exists());

    let content = fs::read_to_string(root.join("commands/foo.toml")).unwrap();
    assert!(content.contains("prompt = \"# Foo Command\""));
    assert!(content.contains("model = \"gemini-1.5-pro\""));

    let skill_content = fs::read_to_string(root.join("skills/python_expert/SKILL.md")).unwrap();
    assert!(!skill_content.contains("metadata:"));
    assert!(skill_content.contains("type: expert"));
    assert!(skill_content.contains("Python Expert Content"));
}

#[test]
fn test_e2e_build_claude_code() {
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
    fs::write(root.join("toolkit.yaml"), config).unwrap();

    let mut cmd = Command::new(assert_cmd::cargo_bin!("atb"));
    cmd.arg("build").arg("--config").arg(root.join("toolkit.yaml"));
    cmd.assert().success();

    assert!(root.join("commands/foo.md").exists());
    let content = fs::read_to_string(root.join("commands/foo.md")).unwrap();
    assert!(!content.contains("metadata:"));
    assert!(content.contains("description: Foo command description"));
    assert!(content.contains("# Foo Command"));
}

#[test]
fn test_e2e_build_opencode() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    setup_fixtures(root);

    let config = format!(
        r#"
source: {}
target: opencode
resources:
  commands:
    - plugin_a:foo
"#,
        root.display()
    );
    fs::write(root.join("toolkit.yaml"), config).unwrap();

    let mut cmd = Command::new(assert_cmd::cargo_bin!("atb"));
    cmd.arg("build").arg("--config").arg(root.join("toolkit.yaml"));
    cmd.assert().success();

    assert!(root.join("commands/foo.md").exists());
    assert!(root.join("AGENTS.md").exists());

    let agents_content = fs::read_to_string(root.join("AGENTS.md")).unwrap();
    assert!(agents_content.contains("# Global Instructions"));

    let content = fs::read_to_string(root.join("commands/foo.md")).unwrap();
    assert!(!content.contains("metadata:"));
    assert!(content.contains("# Foo Command"));
}

fn setup_fixtures(root: &Path) {
    let plugin_a_cmds = root.join("plugin_a/commands");
    let plugin_c_skills = root.join("plugin_c/skills/python_expert");

    fs::create_dir_all(&plugin_a_cmds).unwrap();
    fs::create_dir_all(&plugin_c_skills).unwrap();

    fs::write(plugin_a_cmds.join("foo.md"), "# Foo Command").unwrap();
    fs::write(
        plugin_a_cmds.join("foo.yaml"),
        r#"
gemini-cli:
  model: gemini-1.5-pro
  description: Foo command description
claude-code:
  description: Foo command description
opencode:
  description: Foo command description
"#,
    )
    .unwrap();

    fs::write(plugin_c_skills.join("SKILL.md"), "Python Expert Content").unwrap();
    fs::write(
        plugin_c_skills.join("SKILL.yaml"),
        r#"
gemini-cli:
  type: expert
claude-code:
  type: expert
opencode:
  type: expert
"#,
    )
    .unwrap();

    fs::write(root.join("AGENTS.md"), "# Global Instructions").unwrap();
}
