# TASK 2-1: Define `requirements.yaml` Parsing Model

## 개요 (Description)
플러그인별 의존성 설정 파일인 `requirements.yaml`을 파싱하기 위한 데이터 모델을 정의합니다.

## 수정 파일 (Files to Modify)
- `src/builder/dependency.rs` (신규 생성)

## 상세 지침 (Actionable Instructions)
1. `src/builder/dependency.rs` 파일을 생성합니다.
2. `serde::Deserialize`를 구현하는 `DependencyConfig` 구조체를 정의합니다.
3. YAML 구조에 맞게 중첩된 HashMap 구조를 가집니다.
   - 예: `HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>`
   - 키 순서: `ResourceType(plural)` -> `ResourceName` -> `DependencyType(plural)` -> `Vec<Plugin:Name>`

## 검증 방법 (Verification)
- 샘플 YAML 문자열을 해당 모델로 역직렬화(Deserialize)하는 유닛 테스트를 작성하여 성공 여부를 확인합니다.
