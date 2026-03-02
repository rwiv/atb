# Task 3.2: Emitter Clean 로직 업데이트 및 Builder 연동

## 1. 목표
새롭게 도입된 `.codex` 디렉터리가 빌드 전 초기화 단계(Clean)에서 확실하게 제거되도록 하고, 전체 동작의 안정성을 테스트합니다.

## 2. 작업 내용
1. `src/core/mod.rs` 에 새로운 상수를 추가합니다. (예: `DIR_CODEX = ".codex"`)
2. `src/builder/emitter.rs` 내의 `clean` 메서드에서 삭제할 타겟 목록(`targets`)에 `DIR_CODEX`를 포함합니다.
3. 관련 단위 테스트(`test_clean` 등)에서 `.codex` 디렉터리가 정상적으로 삭제되는지 확인하는 검증 로직을 추가합니다.

## 3. 성공 기준
- 빌드 전 `.codex` 디렉터리가 남아있을 경우 깔끔하게 삭제된다.
- 전체 코드베이스에 대해 `cargo check`, `cargo test`가 모두 성공한다.
- `cargo fmt` 및 `cargo clippy` 수행을 통해 코드 스타일 일관성을 확보한다.
