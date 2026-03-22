# TASK 5-2: E2E Tests for `agb sync` with Fixtures

## 개요 (Description)
`agb sync` 명령어의 전체 프로세스를 `fixtures`를 사용하여 검증하는 E2E 테스트를 작성합니다.

## 수정 파일 (Files to Modify)
- `tests/e2e_sync_test.rs` (신규 파일)
- `tests/fixtures/` (테스트 데이터 추가)

## 상세 지침 (Actionable Instructions)
1. `tests/fixtures/sync/` 디렉터리에 샘플 소스(`source`)와 타겟(`dest`) 구조를 구축합니다.
2. `source` 디렉터리에 `toolkit.yaml`을 포함한 `plugins`를 구성합니다.
3. `dest` 디렉터리에 `build`된 리소스를 수동으로(또는 `build` 명령으로) 생성합니다.
4. `dest`의 파일들을 임의로 수정(프롬프트, 설명 변경, 스킬 내 파일 추가/삭제)합니다.
5. `agb sync` 명령을 실행하여 `source`의 파일들이 예상대로 업데이트되었는지 검증합니다.
    - 파일의 타임스탬프, 내용(Content), 메타데이터(`description`) 등을 확인합니다.
6. `exclude` 패턴이 적용되어 동기화가 무시된 파일이 있는지 로그를 통해 확인합니다.

## 검증 방법 (Verification)
- `cargo test --test e2e_sync_test`를 실행하여 모든 E2E 테스트가 통과하는지 확인합니다.
- `tempfile`을 사용하여 독립된 테스트 환경에서 실행합니다.
