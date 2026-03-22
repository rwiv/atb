# TASK 2-3: Implement `requirements.yaml` Caching

## 개요 (Description)
동일한 플러그인에 속한 여러 리소스를 검사할 때 중복된 파일 읽기를 방지하기 위해 `requirements.yaml` 로딩 결과를 캐싱합니다.

## 수정 파일 (Files to Modify)
- `src/builder/dependency.rs`

## 상세 지침 (Actionable Instructions)
1. `DependencyChecker` 내부에 `HashMap<String, Option<DependencyConfig>>` 형태의 캐시 필드를 추가합니다.
   - 키는 플러그인 이름입니다.
   - `Option`을 사용하여 파일이 없는 경우도 캐싱하여 반복적인 파일 존재 확인을 방지합니다.
2. 파일 로드 시 먼저 캐시를 확인하고, 없으면 읽어서 캐시에 저장하는 로직을 적용합니다.

## 검증 방법 (Verification)
- 동일 플러그인 리소스 검사 시 파일 I/O가 최소화되는지 확인하는 로그를 추가하여 검증합니다.
