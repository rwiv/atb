# TASK 2-1: Update Integration/E2E Test Codes

테스트 환경에서 사용하는 파일 생성 및 검증 로직을 새로운 파일명으로 업데이트합니다.

## 변경 대상
- `tests/app_integration_test.rs`
- `tests/e2e_sync_multiline_test.rs`
- `tests/metadata_map_test.rs`
- `tests/fixtures/` 내 파일들 (필요한 경우)

## 상세 작업
- [ ] 각 테스트 코드 내의 `toolkit.yaml`, `overrides.yaml`, `requirements.yaml` 파일 생성을 `toolkit.yaml`, `overrides.yaml`, `requirements.yaml`로 변경한다.
- [ ] 에러 메시지를 검증하는 `assert!` 구문의 문자열도 최신화된 메시지로 수정한다.
- [ ] 전체 테스트를 실행하여 정상 동작 여부를 확인한다. (`cargo test`)
