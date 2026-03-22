use atb::app::{App, Cli, Commands};
use std::fs;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_app_build_integration() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    // Setup fixtures
    setup_fixtures(root);

    // Create toolkit.yaml
    let config_path = root.join("toolkit.yaml");
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
    fs::write(&config_path, config).unwrap();

    let app = App::new();
    let cli = Cli {
        command: Commands::Build {
            config: Some(config_path.to_string_lossy().into_owned()),
        },
    };

    // Run build directly
    app.run(cli).expect("App run failed");

    // Verify outputs
    assert!(root.join("commands/foo.toml").exists());
    assert!(root.join("skills/python_expert/SKILL.md").exists());
}

#[test]
fn test_app_sync_integration() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    // Setup fixtures
    setup_fixtures(root);

    // Build first to create targets
    let config_path = root.join("toolkit.yaml");
    let config = format!(
        r#"
source: {}
target: gemini-cli
resources:
  commands:
    - plugin_a:foo
"#,
        root.display()
    );
    fs::write(&config_path, config).unwrap();

    let app = App::new();
    let build_cli = Cli {
        command: Commands::Build {
            config: Some(config_path.to_string_lossy().into_owned()),
        },
    };
    app.run(build_cli).unwrap();

    // Modify target file
    let target_path = root.join("commands/foo.toml");
    let mut content = fs::read_to_string(&target_path).unwrap();
    content = content.replace(
        "description = \"Foo command description\"",
        "description = \"Updated description\"",
    );
    fs::write(&target_path, content).unwrap();

    // Run sync directly
    let sync_cli = Cli {
        command: Commands::Sync {
            config: Some(config_path.to_string_lossy().into_owned()),
        },
    };
    app.run(sync_cli).expect("App sync failed");

    // Verify source update
    let source_path = root.join("plugin_a/commands/foo.md");
    let source_content = fs::read_to_string(source_path).unwrap();
    assert!(source_content.contains("description: Updated description"));
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
