# Plan: Gemini CLI Subagent Tools Field Mapping

## Overview

Gemini CLI 환경에서 Subagent (Agent 리소스)를 `agb build` 할 때, Frontmatter에 `tools` 필드가 작성되어 있지 않으면 자동으로 `tools: ["*"]`를 주입하는 로직을 추가합니다. 
반대로 `agb sync` 과정에서는 대상 Subagent가 Gemini CLI이고 `tools` 필드가 `["*"]` 이며, 원본 소스에 `tools`가 명시되지 않았던 경우, 해당 `tools` 필드를 제외(삭제)한 상태로 동기화 처리를 진행하여 원본 소스에 불필요하게 병합되지 않도록 합니다.

## Phases

### Phase 1: 구현 및 문서 갱신

- **TASK-1-1**: `GeminiTransformer`의 `transform` 메서드를 수정하여 Agent 빌드 시 `tools` 필드가 없을 경우 `["*"]`를 자동 주입하도록 구현합니다.
- **TASK-1-2**: `Syncer::sync_resource` 메서드 등을 수정하여 `agb sync` 수행 시, 타겟이 `GeminiCli`이고 `tools: ["*"]`이면서 원본 데이터에 `tools`가 없었다면 역변환된 메타데이터에서 `tools` 필드를 삭제하여 동기화 과정에서 무시되도록 처리합니다.
- **TASK-1-3**: 변경된 동작(Gemini CLI 빌드/동기화 시의 `tools` 자동 주입 및 제외 규칙)을 관련 기술 문서(`specs/spec.md`)에 갱신합니다.
