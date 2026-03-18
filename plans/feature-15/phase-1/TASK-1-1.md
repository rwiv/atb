# TASK-1-1: Gemini CLI Agent 빌드 시 tools 자동 주입 로직 구현

## 개요
`agb build` 시 타겟이 `gemini-cli`인 경우, Agent 리소스의 Frontmatter 메타데이터에 `tools` 필드가 아예 존재하지 않는다면 `tools: ["*"]`를 자동으로 주입해야 합니다.

## 세부 구현 계획

1. **파일 위치**: `src/transformer/gemini.rs`
2. **수정 대상**: `GeminiTransformer::transform` 메서드의 `Resource::Agent(data)` 처리 부분
3. **로직**:
   - `Resource::Agent(data)`가 매칭될 때, `data.metadata`에 `tools` 필드가 존재하는지 확인합니다.
   - 존재하지 않는다면 `data`를 복제(`clone`)하거나 새로운 `Resource::Agent` 객체를 만들어서 `metadata`에 `tools: ["*"]`를 추가합니다.
   - 이후 기존과 동일하게 `DefaultTransformer`를 통해 변환 작업을 위임합니다.
4. **테스트 추가**:
   - `src/transformer/gemini.rs` 내부의 `mod tests`에 `tools` 필드가 없는 Agent가 `tools: ["*"]`를 포함한 채로 변환되는지 확인하는 테스트 케이스를 작성합니다.
   - 이미 `tools` 필드가 존재하는 경우에는 기존 값을 덮어쓰지 않는지 확인하는 테스트 케이스도 작성합니다.
