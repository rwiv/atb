# TASK: Verify Sync Behavior with Mapping

## 개요
`agb sync` 동작 시 매핑된 값이 소스로 역전파되어 원본을 훼손하지 않는지 확인합니다.

## 작업 상세

### 1. `agb sync` 테스트
- `overrides.yaml`에 의해 `model: sonnet`이 `model: gemini-3.0-flash`로 빌드된 상태에서, `agb sync`를 실행했을 때 소스의 `sonnet` 값이 유지되는지 확인합니다.
- 설계 상 `sync`는 `Transformer::detransform`을 거치지만, 최종적으로 소스에 쓰여질 때 `overrides.yaml`에 의한 변환 결과가 아닌 원본 데이터 구조를 유지해야 함을 검증합니다. (현재 `agb`는 `content`와 `description` 위주로 동기화하므로 안전할 것으로 예상되나 명시적 확인이 필요함)

## 검증 방법
- `overrides.yaml`이 적용된 프로젝트에서 `agb sync`를 수행하고 소스 파일의 변경 사항을 검사하는 테스트 코드를 작성합니다.
