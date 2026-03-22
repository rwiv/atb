# TASK-1-1: 상수 정리 및 파일 필터 업데이트

## 작업 내용

- [ ] `src/core/constants.rs`에서 `PLUGINS_DIR_NAME` 상수 제거 및 사용처 검토.
- [ ] `src/core/filter.rs`의 `is_valid` 함수 수정:
    - `AGENTS.md`, `overrides.yaml` 등 전역 파일들이 소스 루트에 있을 경우, 에러를 발생시키는 대신 `Ok(false)`를 반환하여 스캔 대상에서 제외하도록 변경.
    - 현재는 `FORBIDDEN_FILES`에 포함된 파일 발견 시 즉시 에러를 발생시킴.
- [ ] `FileFilter` 관련 유닛 테스트 업데이트.

## 검증 방법

- `cargo test core::filter` 실행하여 필터 로직 검증.
- `FORBIDDEN_FILES`가 루트에 있을 때 에러 없이 제외되는지 확인하는 테스트 케이스 추가.
