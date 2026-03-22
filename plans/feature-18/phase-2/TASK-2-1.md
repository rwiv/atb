# TASK-2-1: E2E 및 통합 테스트 경로 업데이트

## 작업 내용

- [ ] `tests/*.rs` 파일들에 포함된 `setup_fixtures` 함수 수정:
    - `plugins/` 하위 디렉터리 생성 로직 제거.
    - 리소스들이 `source_root` 바로 하위의 플러그인 폴더에 위치하도록 변경.
- [ ] 다음 파일들의 테스트 케이스 및 경로 업데이트:
    - `tests/app_integration_test.rs`
    - `tests/e2e_build_test.rs`
    - `tests/e2e_sync_test.rs`
    - `tests/e2e_codex_sync_test.rs`
    - `tests/e2e_dependency_test.rs`
    - `tests/e2e_path_split_test.rs`
    - `tests/e2e_skill_extras_test.rs`
    - `tests/e2e_sync_multiline_test.rs`
    - `tests/metadata_map_test.rs`
    - `tests/repro_missing_resource_test.rs`

## 검증 방법

- `cargo test` 실행하여 전체 테스트 통과 여부 확인.
- 특히 `build` 및 `sync` 관련 E2E 테스트가 평탄화된 구조에서 올바르게 작동하는지 검증.
