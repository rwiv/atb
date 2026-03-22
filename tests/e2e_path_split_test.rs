use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_e2e_path_split_build() {
    let source_repo = tempdir().unwrap();
    let agent_workspace = tempdir().unwrap();

    let source_path = source_repo.path();
    let workspace_path = agent_workspace.path();

    // 1. Setup source repository
    let plugins_dir = source_path;
    let plugin_a_cmds = plugins_dir.join("plugin_a/commands");
    fs::create_dir_all(&plugin_a_cmds).unwrap();

    fs::write(plugin_a_cmds.join("hello.md"), "# Hello").unwrap();
    fs::write(
        plugin_a_cmds.join("hello.yaml"),
        r#"
gemini-cli:
  model: gemini-1.5-pro
  description: Greeting
"#,
    )
    .unwrap();

    fs::write(source_path.join("AGENTS.md"), "# Global Instructions").unwrap();

    // 2. Setup agent workspace with toolkit.yaml
    let config_content = format!(
        r#"
source: {}
target: gemini-cli
resources:
  commands:
    - plugin_a:hello
"#,
        source_path.display()
    );
    let config_file = workspace_path.join("toolkit.yaml");
    fs::write(&config_file, config_content).unwrap();

    // 3. Run build from workspace
    let mut cmd = Command::new(assert_cmd::cargo_bin!("atb"));
    cmd.arg("build").arg("--config").arg(&config_file);
    cmd.assert().success();

    // 4. Verify outputs are in workspace
    assert!(workspace_path.join("commands/hello.toml").exists());
    assert!(workspace_path.join("GEMINI.md").exists());

    // 5. Verify source repo is still clean of outputs
    assert!(!source_path.join("commands").exists());
    assert!(!source_path.join("GEMINI.md").exists());

    // 6. Verify content
    let hello_toml = fs::read_to_string(workspace_path.join("commands/hello.toml")).unwrap();
    assert!(hello_toml.contains(r##"prompt = "# Hello""##));
    assert!(hello_toml.contains(r##"description = "Greeting""##));

    let gemini_md = fs::read_to_string(workspace_path.join("GEMINI.md")).unwrap();
    assert!(gemini_md.contains("# Global Instructions"));
}

#[test]
fn test_e2e_path_split_invalid_source() {
    let agent_workspace = tempdir().unwrap();
    let workspace_path = agent_workspace.path();

    let config_content = r#"
source: /non/existent/path/for/atb/test
target: gemini-cli
resources:
  commands:
    - p1:hello
"#;
    let config_file = workspace_path.join("toolkit.yaml");
    fs::write(&config_file, config_content).unwrap();

    let mut cmd = Command::new(assert_cmd::cargo_bin!("atb"));
    cmd.arg("build").arg("--config").arg(&config_file);

    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Source directory does not exist"));
}
