# TASK 4-2: End-to-End Integration Tests

## 개요 (Description)
실제 파일 시스템과 `agb build` 명령어를 연계한 통합 테스트를 작성합니다.

## 수정 파일 (Files to Modify)
- `tests/e2e_dependency_test.rs` (신규 생성)

## 상세 지침 (Actionable Instructions)
1. `tests/fixtures/` 디렉터리에 의존성이 포함된 테스트 프로젝트 환경을 구성합니다.
2. 의존성이 충족된 경우 빌드 성공을 확인합니다.
3. 의존성이 누락된 `toolkit.yaml`을 사용했을 때 빌드가 실패하고 정확한 에러 메시지가 출력되는지 확인합니다.

## 검증 방법 (Verification)
- `cargo test --test e2e_dependency_test` 실행.
