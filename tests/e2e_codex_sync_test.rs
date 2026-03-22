use assert_cmd::Command;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_e2e_sync_codex() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    setup_fixtures(root);

    let config = format!(
        r#"
source: {}
target: codex
resources:
  commands:
    - plugin_a:foo
  agents:
    - plugin_b:bar
"#,
        root.display()
    );
    fs::write(root.join("toolkit.yaml"), config).unwrap();

    // 1. Initial Build
    let mut cmd = Command::new(assert_cmd::cargo_bin!("atb"));
    cmd.arg("build").arg("--config").arg(root.join("toolkit.yaml"));
    cmd.assert().success();

    // Verify build structure for Codex
    assert!(root.join("prompts/foo.md").exists());
    assert!(root.join("agents/bar.toml").exists());

    // 2. Modify target files
    // Modify Command (Markdown in prompts/)
    let cmd_md_path = root.join("prompts/foo.md");
    let mut md_content = fs::read_to_string(&cmd_md_path).unwrap();
    md_content = md_content.replace(
        "description: Foo command description",
        "description: Codex Command Updated",
    );
    md_content = md_content.replace("# Foo Command", "# Codex Command Content Updated");
    fs::write(&cmd_md_path, md_content).unwrap();

    // Modify Agent (TOML in agents/)
    let agent_toml_path = root.join("agents/bar.toml");
    let mut toml_content = fs::read_to_string(&agent_toml_path).unwrap();
    println!("Agent TOML before replace: {}", toml_content);
    toml_content = toml_content.replace("# Bar Agent Content", "# Codex Agent Content Updated");
    fs::write(&agent_toml_path, toml_content).unwrap();

    // Modify Agent Description (in .codex/config.toml)
    let config_toml_path = root.join(".codex/config.toml");
    let mut config_toml_content = fs::read_to_string(&config_toml_path).unwrap();
    config_toml_content = config_toml_content.replace(
        "description = \"Bar agent description\"",
        "description = \"Codex Agent Updated\"",
    );
    fs::write(&config_toml_path, config_toml_content).unwrap();

    // 3. Run Sync
    let mut cmd = Command::new(assert_cmd::cargo_bin!("atb"));
    cmd.arg("sync").arg("--config").arg(root.join("toolkit.yaml"));
    cmd.assert().success();

    // 4. Verify Source
    // Verify Command Source
    let cmd_source_path = root.join("plugin_a/commands/foo.md");
    let cmd_source_content = fs::read_to_string(cmd_source_path).unwrap();
    assert!(cmd_source_content.contains("description: Codex Command Updated"));
    assert!(cmd_source_content.contains("# Codex Command Content Updated"));

    // Verify Agent Source
    let agent_source_path = root.join("plugin_b/agents/bar.md");
    let agent_source_content = fs::read_to_string(agent_source_path).unwrap();
    assert!(agent_source_content.contains("description: Codex Agent Updated"));
    assert!(agent_source_content.contains("# Codex Agent Content Updated"));
}

fn setup_fixtures(root: &Path) {
    let plugin_a_cmds = root.join("plugin_a/commands");
    let plugin_b_agents = root.join("plugin_b/agents");

    fs::create_dir_all(&plugin_a_cmds).unwrap();
    fs::create_dir_all(&plugin_b_agents).unwrap();

    // Command fixture
    fs::write(
        plugin_a_cmds.join("foo.md"),
        "---
description: Foo command description
---
# Foo Command",
    )
    .unwrap();

    // Agent fixture
    fs::write(
        plugin_b_agents.join("bar.md"),
        "---
description: Bar agent description
---
# Bar Agent Content",
    )
    .unwrap();
}
