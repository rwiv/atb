# Transformer Module

`transformer` 모듈은 `agb`의 내부 리소스 모델(`core::Resource`)을 각 에이전트(Gemini, Claude, OpenCode, Codex)의 규격에 맞는 물리적 파일 형식으로 변환하는 역할을 담당합니다.

## 핵심 역할

1. **포맷 변환**: 내부 메타데이터와 Markdown 컨텐츠를 타겟 에이전트가 이해할 수 있는 형식(예: TOML, Frontmatter Markdown)으로 변환합니다.
2. **경로 결정**: 각 리소스가 타겟 에이전트의 파일 시스템 구조에서 어디에 위치해야 하는지 정의합니다. (예: `commands/foo.toml`)
3. **전역 지침 처리**: 루트의 `AGENTS.md`를 각 에이전트의 메인 메모리 파일(Gemini: `GEMINI.md`, Claude: `CLAUDE.md`, OpenCode: `AGENTS.md`)로 변환합니다.

## 모듈 구조

- `mod.rs`: `Transformer` 트레이트 및 `TransformerFactory` 정의.
- `gemini.rs`: Gemini-cli용 하이브리드 변환기.
- `codex.rs`: Codex용 변환기 (멀티 에이전트 설정 지원을 위해 `.codex/config.toml` 자동 생성 및 `developer_instructions` 매핑 수행).
- `default.rs`: 공용 마크다운 변환기 (Claude-code, OpenCode 및 타겟별 기본 변환).

## 주요 구성 요소

### 1. `Transformer` Trait

모든 변환기가 구현해야 하는 인터페이스입니다.

```rust
use crate::core::{Resource, TransformedFile};

pub trait Transformer {
    /// 리소스를 타겟 포맷으로 변환
    fn transform(&self, resource: &Resource) -> Result<TransformedFile>;
    
    /// 전역 지침(AGENTS.md)을 타겟 규격으로 변환
    fn transform_root_prompt(&self, content: &str) -> Result<TransformedFile>;
}
```

### 2. `TransformerFactory`

`core::BuildTarget` 열거형을 기반으로 적절한 `Transformer` 구현체를 동적으로 생성하여 반환합니다.

## 타겟별 특이사항

- **Gemini-cli**: 
  - **Commands**: 메타데이터 -> TOML Key-Value 매핑, 본문 -> TOML `prompt` 필드로 삽입하여 `*.toml` 생성.
  - **Agents / Skills**: `DefaultTransformer`를 사용하여 메타데이터가 포함된 마크다운 `*.md` 생성.
  - 전역 지침: `GEMINI.md` 생성.
- **Claude-code**: 
  - `DefaultTransformer` 사용. 메타데이터 -> YAML Frontmatter, 본문 -> 마크다운 본문 결합하여 `*.md` 생성.
  - 전역 지침: `CLAUDE.md` 생성.
- **OpenCode**: 
  - `DefaultTransformer` 사용. 메타데이터 -> YAML Frontmatter, 본문 -> 마크다운 본문 결합하여 `*.md` 생성.
  - 전역 지침: `AGENTS.md` 생성.
- **Codex**:
  - **Commands**: `DefaultTransformer`와 유사하게 마크다운 형식을 유지하지만, `prompts/` 디렉터리에 생성.
  - **Agents**: 개별 에이전트는 `agents/[name].toml` 로 생성되며, 본문은 `developer_instructions` 필드에 삽입됩니다. 이후 전체 에이전트 목록이 `.codex/config.toml` (설명 등 메타데이터 포함) 에 취합되어 자동 생성됩니다.
  - 전역 지침: `AGENTS.md` 생성.

## 새로운 에이전트 추가 방법

기존 구조를 유지하며 새로운 빌드 타겟을 지원하려면 다음 체크리스트를 따르십시오:

1.  **`BuildTarget` 등록**: `src/core/target.rs`의 `BuildTarget` 열거형에 새 에이전트 식별자 추가 및 예약어 상수 정의.
2.  **`Transformer` 구현**: `src/transformer/` 내에 새 모듈(예: `foo_agent.rs`)을 생성하고 `Transformer` 트레이트 구현.
    - `transform`: 내부 리소스를 에이전트 전용 포맷으로 변환.
    - `transform_root_prompt`: `AGENTS.md`를 에이전트 메인 파일로 변환.
3.  **팩토리 분기 추가**: `src/transformer/mod.rs`의 `TransformerFactory::create`에서 새 타겟에 대한 구현체 반환 로직 추가.
4.  **역변환(Detransform) 지원**: 동기화 기능을 위해 `Transformer::detransform` 구현 (선택 사항이나 권장).
