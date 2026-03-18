# TASK-1-2: Gemini CLI Agent 동기화 시 tools 필드 제외 로직 구현

## 개요
`agb sync` 시 타겟이 `gemini-cli`인 경우, 타겟 파일(마크다운 Frontmatter)에 `tools: ["*"]`가 존재하더라도, 원본 소스에 원래 `tools` 필드가 없었다면 해당 필드를 제외(삭제)한 상태로 동기화 처리를 진행해야 합니다. 

## 세부 구현 계획

1. **상태 접근 필요성**: `Syncer` 내부에서 현재 타겟이 무엇인지(`BuildTarget`)를 알아야 합니다.
   - **수정**: `src/app/mod.rs`와 `src/syncer/mod.rs`를 수정하여 `Syncer::new` 생성 시 `BuildTarget`을 인자로 받도록 합니다.
2. **파일 위치**: `src/syncer/mod.rs`
3. **수정 대상**: `Syncer::sync_resource` 메서드
4. **로직**:
   - `detransform`을 통해 가져온 타겟 데이터(`detransformed.metadata`)와 원본 소스 데이터(`current_metadata`)를 비교합니다.
   - 다음 조건을 모두 만족할 경우 `detransformed.metadata`에서 `tools` 필드를 삭제합니다.
     - 현재 `Syncer`의 타겟이 `BuildTarget::GeminiCli` 일 때.
     - `resource` 타입이 `Agent` 일 때.
     - `detransformed.metadata["tools"]`의 값이 배열 `["*"]` 일 때.
     - `current_metadata`에 `tools` 필드가 아예 존재하지 않을 때.
   - 필드를 삭제함으로써, 이후 `tools`와 관련된 동기화 로직이 추가되더라도 원본이 변경된 것으로 오인되지 않게 만듭니다.
5. **테스트 확인**:
   - (선택사항) 해당 로직이 잘 동작하는지 확인할 수 있는 단위/통합 테스트가 있다면 작성합니다.
