# Task 2.1: `developer_instructions` 필드명 마이그레이션

## 1. 목표
Codex 에이전트 설정의 지시문 필드 이름이 `prompt`에서 `developer_instructions`로 변경되었습니다. 빌드 시 이 규격에 맞추어 개별 TOML 파일이 생성되도록 `CodexTransformer`를 수정합니다.

## 2. 작업 내용
1. `src/transformer/codex.rs` 내부의 `transform_agent_to_toml` 메서드 수정:
   - 본문 텍스트(`data.content`)를 삽입하는 키워드를 `"prompt"`에서 `"developer_instructions"`로 교체합니다.
   - 이때, 기존 로직 중 메타데이터에서 `description`이 남아있을 경우 제거하는 로직을 추가합니다. (새 규격에서 description은 `config.toml`로 분리되기 때문에 개별 파일에는 포함되지 않아야 합니다.)
2. `src/transformer/codex.rs` 내의 단위 테스트(`test_codex_agent_transformation`, `test_codex_agent_multiline_transformation` 등) 수정:
   - Assertion에 사용되는 검증 대상 키를 `"prompt"`에서 `"developer_instructions"`로 변경합니다.
   - description이 더 이상 개별 TOML에 남지 않음을 검증하는 테스트 코드를 작성합니다.

## 3. 성공 기준
- 개별 에이전트 변환 테스트 코드 통과.
- `cargo test --package agb --lib transformer::codex::tests` 테스트 모음이 정상적으로 성공해야 합니다.
