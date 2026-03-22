# TASK: Unit Tests for MetadataMerger

## 개요
`MetadataMerger`의 모든 병합 및 매핑 규칙이 정확히 작동하는지 검증하기 위한 포괄적인 테스트를 작성합니다.

## 작업 상세

### 1. 테스트 시나리오 작성
- **기본 병합**: FM + 외부 YAML 오버라이트가 기존처럼 잘 작동하는지.
- **매핑 적용**: `overrides.yaml`에 정의된 필드가 타겟에 맞게 치환되는지.
- **제외 조건**: `description` 필드나 `number` 타입 필드가 매핑에서 제외되는지.
- **우선순위**: FM -> Map -> External YAML 순서로 최종값이 결정되는지.
    - 특히, `overrides.yaml`로 치환된 값이 외부 YAML의 타겟 섹션에 의해 다시 덮어씌워지는지 확인.

## 검증 방법
- `cargo test loader::merger` 명령을 실행하여 모든 테스트가 통과하는지 확인합니다.
