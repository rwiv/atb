use atb::app::AppContext;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_missing_resource_should_fail_init() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    // Setup basic fixtures
    setup_minimal_fixtures(root);

    // Create toolkit.yaml with a non-existent resource
    let config_path = root.join("toolkit.yaml");
    let config = format!(
        r#"
source: {}
target: gemini-cli
resources:
  commands:
    - plugin_a:existing_cmd
    - plugin_a:non_existent_cmd # This does not exist!
"#,
        root.display()
    );
    fs::write(&config_path, config).unwrap();

    // Now, AppContext::init should FAIL
    let result = AppContext::init(&config_path.to_string_lossy());

    match result {
        Ok(_) => panic!("AppContext::init should fail when resources are missing"),
        Err(e) => {
            let err_msg = e.to_string();
            assert!(err_msg.contains("Missing resources specified in toolkit.yaml"));
            assert!(err_msg.contains("command: 'plugin_a:non_existent_cmd' (Not found)"));
        }
    }
}

#[test]
fn test_resource_type_mismatch_should_fail_init() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    // Setup: Create a Skill named 'my_res'

    let skill_dir = root.join("plugin_a/skills/my_res");
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(skill_dir.join("SKILL.md"), "Skill Content").unwrap();
    fs::write(root.join("AGENTS.md"), "# Global Instructions").unwrap();

    // Create toolkit.yaml requesting 'plugin_a:my_res' as a COMMAND
    let config_path = root.join("toolkit.yaml");
    let config = format!(
        r#"
source: {}
target: gemini-cli
resources:
  commands:
    - plugin_a:my_res # This exists, but as a SKILL, not a COMMAND
"#,
        root.display()
    );
    fs::write(&config_path, config).unwrap();

    let result = AppContext::init(&config_path.to_string_lossy());

    match result {
        Ok(_) => panic!("AppContext::init should fail due to type mismatch"),
        Err(e) => {
            let err_msg = e.to_string();
            assert!(err_msg.contains("Missing resources specified in toolkit.yaml"));
            assert!(err_msg.contains("command: 'plugin_a:my_res' (Not found)"));
        }
    }
}

fn setup_minimal_fixtures(root: &Path) {
    let plugin_a_cmds = root.join("plugin_a/commands");

    fs::create_dir_all(&plugin_a_cmds).unwrap();

    fs::write(plugin_a_cmds.join("existing_cmd.md"), "# Existing Command").unwrap();
    fs::write(root.join("AGENTS.md"), "# Global Instructions").unwrap();
}
