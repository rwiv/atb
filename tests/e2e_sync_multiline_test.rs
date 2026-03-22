use atb::app::AppContext;
use atb::syncer::Syncer;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_sync_succeeds_on_multiline_description() {
    let source_dir = tempdir().unwrap();
    let project_dir = tempdir().unwrap();

    // 1. 소스에 멀티라인 description을 가진 에이전트 생성
    let agent_dir = source_dir.path().join("my_plugin/agents");
    fs::create_dir_all(&agent_dir).unwrap();
    let source_md = agent_dir.join("researcher.md");
    fs::write(
        &source_md,
        "---
name: researcher
description: |
  Existing multi-line
  description
---
Content",
    )
    .unwrap();

    // 2. toolkit.yaml 설정
    let atb_yaml = project_dir.path().join("toolkit.yaml");
    fs::write(
        &atb_yaml,
        format!(
            "source: {}
target: gemini-cli
resources:
  agents:
    - my_plugin:researcher",
            source_dir.path().to_str().unwrap()
        ),
    )
    .unwrap();

    // 3. 타겟 파일 생성 (업데이트된 멀티라인 설명 포함)
    let target_agent_dir = project_dir.path().join("agents");
    fs::create_dir_all(&target_agent_dir).unwrap();
    let target_md = target_agent_dir.join("researcher.md");
    fs::write(
        &target_md,
        "---
name: researcher
description: |
  New multi-line
  description from target
---
Content",
    )
    .unwrap();

    // 4. Sync 실행
    let ctx = AppContext::init(atb_yaml.to_str().unwrap()).unwrap();
    let syncer = Syncer::new(ctx.exclude_patterns.clone());

    let resource = ctx.registry.all_resources().into_iter().next().unwrap();
    let transformer = ctx.transformer.as_ref();

    syncer.sync_resource(resource, transformer, project_dir.path()).unwrap();

    // 5. 소스 파일이 정확히 업데이트되었는지 확인
    let final_source_content = fs::read_to_string(&source_md).unwrap();
    assert!(final_source_content.contains("New multi-line"));
    assert!(final_source_content.contains("description from target"));
    assert!(final_source_content.contains("name: researcher"));
    assert!(final_source_content.contains("Content"));
}
