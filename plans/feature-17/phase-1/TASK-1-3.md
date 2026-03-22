# TASK 1-3: Update Messages and Comments

코드베이스 전반의 주석과 에러 메시지에 포함된 이전 파일명들을 새 파일명으로 일괄 수정합니다.

## 변경 대상
- [ ] `src/builder/dependency.rs`: `requirements.yaml` -> `requirements.yaml` 관련 메시지/주석 수정
- [ ] `src/app/config.rs`: `toolkit.yaml` -> `toolkit.yaml` 관련 메시지/주석 수정
- [ ] `src/app/cli.rs`: `toolkit.yaml` -> `toolkit.yaml` 관련 메시지/주석 수정
- [ ] 기타 `grep`으로 발견된 모든 소스 파일 내 주석

## 상세 작업
- `grep_search`를 사용하여 `toolkit.yaml`, `overrides.yaml`, `requirements.yaml` 패턴을 다시 한 번 확인하고 수동으로 수정한다.
