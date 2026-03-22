# 계획: 소스 디렉터리 구조 평탄화 (Flattening)

소스 저장소에서 `plugins/` 하위 디렉터리 필무 요구 사항을 제거하고 루트에서 직접 리소스를 스캔하도록 리팩토링합니다.

## Phase 1: 코어 로직 리팩토링

- [ ] `src/core/constants.rs` 업데이트: `PLUGINS_DIR_NAME` 제거 혹은 정리.
- [ ] `src/core/filter.rs` 업데이트: 루트에 위치한 전역 파일 제외 로직 구현.
- [ ] `src/loader/mod.rs` 수정: `ResourceLoader::new`에서 `plugins/` 디렉터리 요구 제거 및 루트 스캔 설정.
- [ ] `src/loader/resolver.rs` 검증: 루트 파일들이 리소스로 오인되지 않도록 확인.
- [ ] 기존 로직 변경에 따른 핵심 유닛 테스트 수정.

## Phase 2: 테스트 코드 업데이트 (통합/E2E)

- [ ] `src/loader/mod.rs` 내부 테스트 수정.
- [ ] `tests/app_integration_test.rs` 업데이트.
- [ ] `tests/e2e_build_test.rs` 업데이트.
- [ ] `tests/e2e_sync_test.rs` 업데이트.
- [ ] 기타 모든 `tests/e2e_*_test.rs` 파일들의 `setup_fixtures` 및 경로 설정 수정.

## Phase 3: 최종 검증 및 문서화

- [ ] 전체 테스트 실행 및 성공 확인.
- [ ] 사용자 가이드 문서 업데이트 (`README.md`, `specs/format.md`).
- [ ] 실제 소스 저장소 구조로 빌드 및 동기화 수동 테스트 수행.
