# TASK 1-2: Update `overrides.yaml` to `overrides.yaml`

`overrides.yaml` 파일명을 `overrides.yaml`로 변경하기 위해 로더 로직을 수정합니다.

## 변경 내용
- `src/loader/mod.rs` 파일 수정
    - `overrides.yaml` 하드코딩된 문자열을 `overrides.yaml`로 변경.
    - (선택 사항) `src/core/constants.rs`에 `MAP_FILE_NAME` 상수를 추가하고 이를 참조하도록 리팩토링.

## 상세 작업
- [ ] `src/loader/mod.rs`에서 `overrides.yaml`을 사용하는 모든 곳을 찾아 수정한다.
- [ ] 가능하다면 상수로 분리하여 관리하도록 개선한다.
