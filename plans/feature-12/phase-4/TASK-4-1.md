# TASK 4-1: Unit Tests for Parsing and Mapping

## 개요 (Description)
`requirements.yaml` 파싱 및 `ResourceType` 매핑 로직의 정확성을 유닛 테스트로 검증합니다.

## 수정 파일 (Files to Modify)
- `src/builder/dependency.rs`
- `src/core/resource.rs`

## 상세 지침 (Actionable Instructions)
1. 다양한 형태의 `requirements.yaml`(유효한 형식, 비어있는 형식, 잘못된 형식)에 대한 파싱 테스트를 작성합니다.
2. `ResourceType`의 복수형 변환이 모든 타입에 대해 정확한지 테스트합니다.

## 검증 방법 (Verification)
- `cargo test builder::dependency` 실행.
