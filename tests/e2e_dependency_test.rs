use assert_cmd::Command;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_e2e_dependency_success() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    setup_dependency_fixtures(root);

    let config = format!(
        r#"
source: {}
target: gemini-cli
resources:
  agents:
    - plugin_a:researcher
  skills:
    - plugin_b:web_search
"#,
        root.display()
    );
    fs::write(root.join("atb.yaml"), config).unwrap();

    let mut cmd = Command::new(assert_cmd::cargo_bin!("atb"));
    cmd.arg("build").arg("--config").arg(root.join("atb.yaml"));
    cmd.assert().success();
}

#[test]
fn test_e2e_dependency_missing_skill() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    setup_dependency_fixtures(root);

    // researcher depends on web_search, but web_search is missing in atb.yaml
    let config = format!(
        r#"
source: {}
target: gemini-cli
resources:
  agents:
    - plugin_a:researcher
"#,
        root.display()
    );
    fs::write(root.join("atb.yaml"), config).unwrap();

    let mut cmd = Command::new(assert_cmd::cargo_bin!("atb"));
    cmd.arg("build").arg("--config").arg(root.join("atb.yaml"));

    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Dependency check failed:"))
        .stderr(predicates::str::contains(
            "agent 'plugin_a:researcher' requires skill 'plugin_b:web_search' but it is missing in atb.yaml",
        ));
}

#[test]
fn test_e2e_dependency_circular() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    let plugins = root.join("plugins");
    let p1_cmds = plugins.join("p1/commands");
    let p2_cmds = plugins.join("p2/commands");
    fs::create_dir_all(&p1_cmds).unwrap();
    fs::create_dir_all(&p2_cmds).unwrap();

    fs::write(p1_cmds.join("c1.md"), "C1").unwrap();
    fs::write(p2_cmds.join("c2.md"), "C2").unwrap();

    fs::write(
        plugins.join("p1/deps.yaml"),
        "commands:
  c1:
    commands:
      - p2:c2",
    )
    .unwrap();
    fs::write(
        plugins.join("p2/deps.yaml"),
        "commands:
  c2:
    commands:
      - p1:c1",
    )
    .unwrap();

    let config = format!(
        r#"
source: {}
target: gemini-cli
resources:
  commands:
    - p1:c1
    - p2:c2
"#,
        root.display()
    );
    fs::write(root.join("atb.yaml"), config).unwrap();

    let mut cmd = Command::new(assert_cmd::cargo_bin!("atb"));
    cmd.arg("build").arg("--config").arg(root.join("atb.yaml"));
    cmd.assert().success();
}

#[test]
fn test_e2e_dependency_complex() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    let plugins = root.join("plugins");
    let p1_cmds = plugins.join("p1/commands");
    let p2_agents = plugins.join("p2/agents");
    let p3_skills = plugins.join("p3/skills/s1");

    fs::create_dir_all(&p1_cmds).unwrap();
    fs::create_dir_all(&p2_agents).unwrap();
    fs::create_dir_all(&p3_skills).unwrap();

    fs::write(p1_cmds.join("c1.md"), "C1").unwrap();
    fs::write(p2_agents.join("a1.md"), "A1").unwrap();
    fs::write(p3_skills.join("SKILL.md"), "S1").unwrap();

    // a1 depends on c1 and s1
    let deps_yaml = r#"
agents:
  a1:
    commands:
      - p1:c1
    skills:
      - p3:s1
"#;
    fs::write(plugins.join("p2/deps.yaml"), deps_yaml).unwrap();

    let config = format!(
        r#"
source: {}
target: gemini-cli
resources:
  commands:
    - p1:c1
  agents:
    - p2:a1
  skills:
    - p3:s1
"#,
        root.display()
    );
    fs::write(root.join("atb.yaml"), config).unwrap();

    let mut cmd = Command::new(assert_cmd::cargo_bin!("atb"));
    cmd.arg("build").arg("--config").arg(root.join("atb.yaml"));
    cmd.assert().success();

    // Now remove c1 from atb.yaml
    let config_missing = format!(
        r#"
source: {}
target: gemini-cli
resources:
  agents:
    - p2:a1
  skills:
    - p3:s1
"#,
        root.display()
    );
    fs::write(root.join("atb.yaml"), config_missing).unwrap();

    let mut cmd = Command::new(assert_cmd::cargo_bin!("atb"));
    cmd.arg("build").arg("--config").arg(root.join("atb.yaml"));
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("agent 'p2:a1' requires command 'p1:c1'"));
}

fn setup_dependency_fixtures(root: &Path) {
    let plugins = root.join("plugins");
    let plugin_a_agents = plugins.join("plugin_a/agents");
    let plugin_b_skills = plugins.join("plugin_b/skills/web_search");

    fs::create_dir_all(&plugin_a_agents).unwrap();
    fs::create_dir_all(&plugin_b_skills).unwrap();

    fs::write(plugin_a_agents.join("researcher.md"), "Researcher Content").unwrap();
    fs::write(
        plugins.join("plugin_a/deps.yaml"),
        r#"
agents:
  researcher:
    skills:
      - plugin_b:web_search
"#,
    )
    .unwrap();

    fs::write(plugin_b_skills.join("SKILL.md"), "Web Search Content").unwrap();
}
