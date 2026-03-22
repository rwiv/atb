# TASK 1-1: Update File Name Constants

시스템 전역에서 사용하는 핵심 설정 파일명의 상수 값을 변경합니다.

## 변경 내용
- `src/core/constants.rs` 파일 수정
    - `CONFIG_FILE_NAME`: `toolkit.yaml` -> `toolkit.yaml`
    - `DEPS_FILE_NAME`: `requirements.yaml` -> `requirements.yaml`

## 상세 작업
- [ ] `src/core/constants.rs` 파일을 열어 상수를 수정한다.
- [ ] 컴파일을 시도하여 해당 상수를 사용하는 지점들에서 발생하는 에러를 확인한다. (이후 Task의 가이드가 됨)
