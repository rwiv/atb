# TASK 1-2: Add plural form support to `ResourceType`

## 개요 (Description)
`requirements.yaml`의 키값(예: "agents", "skills")과 `ResourceType` 열거형 간의 변환 기능을 추가합니다.

## 수정 파일 (Files to Modify)
- `src/core/resource.rs` (또는 `ResourceType`이 정의된 곳)

## 상세 지침 (Actionable Instructions)
1. `ResourceType` 열거형에 `pub fn from_plural(s: &str) -> Option<Self>` 연관 함수를 추가합니다.
    - "commands" -> `ResourceType::Command`
    - "agents" -> `ResourceType::Agent`
    - "skills" -> `ResourceType::Skill`
2. `ResourceType` 열거형에 `pub fn to_plural(&self) -> &'static str` 메서드를 추가하여 현재 타입을 복수형 문자열로 반환합니다.

## 검증 방법 (Verification)
- `src/core/resource.rs`에 유닛 테스트를 추가하여 상호 변환이 정확한지 확인합니다.
