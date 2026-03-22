# TASK-1-2: `source` 필드 경로 확장 구현

## 목표
- `toolkit.yaml`의 `source` 경로에서 `~` 기호를 절대 경로로 확장.

## 작업 내용
1. `src/builder/config.rs`의 `parse_config` 또는 `load_config` 함수 수정.
2. `shellexpand::tilde`를 사용하여 `config.source` 값 업데이트.
3. 관련 단위 테스트 추가:
   - `~/test` -> `/Users/user/test` 변환 확인.

## 성공 기준
- `toolkit.yaml`에 `source: ~/path` 기입 시 로더가 실제 절대 경로에서 리소스를 찾음.
