# TASK: Integration Test for Metadata Map

## 개요
실제 파일 시스템과 `overrides.yaml` 파일을 사용하여 전체 빌드 프로세스에서 매핑이 정상적으로 작동하는지 검증합니다.

## 작업 상세

### 1. 통합 테스트 케이스 추가
- `tests/fixtures/` 아래에 `overrides.yaml`을 포함한 테스트 시나리오를 구성하거나, 코드 상에서 임시 디렉터리를 생성하여 테스트합니다.
- `agb build` 실행 후 결과물 파일(TOML 또는 MD)에서 `model` 필드 등이 타겟에 맞게 치환되었는지 확인합니다.

## 검증 방법
- `cargo test`를 실행하여 새로운 통합 테스트가 통과하는지 확인합니다.
