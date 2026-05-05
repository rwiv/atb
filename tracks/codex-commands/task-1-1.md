# Task 1.1: 커맨드·스킬 출력 경로 변경

## Overview

Codex CLI의 신규 정책에 따라 커맨드와 스킬의 출력 경로를 모두 `../.agents/skills/[name]/SKILL.md`로 변경합니다. `DIR_PROMPTS` 상수를 삭제하고 `DIR_AGENTS_SKILLS` 상수를 추가하며, `transform()`과 `get_target_path()`의 Command·Skill 분기, `Emitter::clean_all()`에서의 `DIR_PROMPTS` 참조를 일괄 수정합니다. 변경 완료 후 `specs/spec.md`의 Codex 변환 규격 테이블을 갱신합니다.

## Related Files

### Target Files

- `src/core/constants.rs`: 수정 — `DIR_PROMPTS` 삭제, `DIR_AGENTS_SKILLS` 추가
- `src/transformer/codex.rs`: 수정 — `transform()`, `get_target_path()` Command·Skill 분기 및 관련 테스트
- `src/builder/emitter.rs`: 수정 — `clean_all()`에서 `DIR_PROMPTS` 제거
- `src/transformer/README.md`: 수정 — Codex Command·Skill 출력 경로 설명 갱신
- `specs/spec.md`: 수정 — Codex 변환 규격 테이블 갱신

### Reference Files

- `specs/spec.md`: 4절 타겟별 변환 사양 — 갱신 대상 확인
- `src/core/constants.rs`: 상수 정의 현황 확인
- `src/transformer/codex.rs`: 변환 로직 및 기존 테스트 확인
- `src/builder/emitter.rs`: `clean_all()` 구현 확인
- `src/transformer/README.md`: 현재 경로 설명 확인

## Workflow

### Step 1: 상수 변경 (`src/core/constants.rs`)

`DIR_PROMPTS`는 Codex 전용이었으며 이 작업 이후 참조처가 없으므로 삭제합니다. 새 경로를 위한 `DIR_AGENTS_SKILLS` 상수를 추가합니다.

```rust
// src/core/constants.rs

// 삭제
// pub const DIR_PROMPTS: &str = "prompts";

// 추가 (output-dir 기준 상대경로: output-dir의 상위 디렉터리에 위치)
pub const DIR_AGENTS_SKILLS: &str = "../.agents/skills";
```

`DIR_AGENTS_SKILLS`에 `../`를 포함한 이유: output-dir(`.codex/`)가 프로젝트 루트의 하위 디렉터리이므로, `.agents/skills/`는 output-dir 기준으로 상위 디렉터리에 위치합니다. Rust `PathBuf::join`이 `../`를 보존하므로 OS가 올바르게 해석합니다. Command와 Skill이 동일 경로를 공유하므로 상수명은 경로의 실제 의미를 반영합니다.

### Step 2: `transform()` Command·Skill 분기 수정 (`src/transformer/codex.rs`)

import에서 `DIR_PROMPTS`와 `EXT_MD`를 제거하고 `DIR_AGENTS_SKILLS`를 추가합니다.

```rust
// src/transformer/codex.rs
use crate::core::{
    AGENTS_MD, BuildTarget, CODEX_CONFIG_FILE_NAME, DIR_AGENTS, DIR_CODEX,
    DIR_AGENTS_SKILLS, EXT_TOML,
    Resource, ResourceData, ResourceType, SKILL_MD, TransformedFile,
};

impl Transformer for CodexTransformer {
    fn transform(&self, resource: &Resource) -> Result<TransformedFile> {
        match resource {
            Resource::Command(data) => {
                let default_transformer = DefaultTransformer { target: BuildTarget::Codex };
                let mut transformed = default_transformer.transform(resource)?;
                transformed.path = PathBuf::from(DIR_AGENTS_SKILLS)
                    .join(&data.name)
                    .join(SKILL_MD);
                Ok(transformed)
            }
            Resource::Agent(data) => self.transform_agent_to_toml(data),
            Resource::Skill(data) => {
                let default_transformer = DefaultTransformer { target: BuildTarget::Codex };
                let mut transformed = default_transformer.transform(resource)?;
                transformed.path = PathBuf::from(DIR_AGENTS_SKILLS)
                    .join(&data.base.name)
                    .join(SKILL_MD);
                Ok(transformed)
            }
        }
    }
```

`DefaultTransformer::transform()`을 통해 컨텐츠는 그대로 유지하고 `path`만 교체합니다. Command와 Skill 모두 frontmatter + Markdown 포맷이므로 별도 변환은 불필요합니다. Command와 Skill이 동일한 `DIR_AGENTS_SKILLS` 상수를 사용합니다.

### Step 3: `get_target_path()` Command·Skill 분기 수정 (`src/transformer/codex.rs`)

```rust
    fn get_target_path(&self, r_type: ResourceType, name: &str) -> PathBuf {
        match r_type {
            ResourceType::Command => {
                PathBuf::from(DIR_AGENTS_SKILLS).join(name).join(SKILL_MD)
            }
            ResourceType::Agent => PathBuf::from(DIR_AGENTS).join(format!("{}{}", name, EXT_TOML)),
            ResourceType::Skill => PathBuf::from(DIR_AGENTS_SKILLS).join(name).join(SKILL_MD),
        }
    }
```

### Step 4: 단위 테스트 수정 및 추가 (`src/transformer/codex.rs`)

`test_codex_command_transformation` 테스트의 경로 assertion을 변경하고, Skill 변환 경로 테스트를 추가합니다.

```rust
#[test]
fn test_codex_command_transformation() {
    let transformer = CodexTransformer;
    let resource = Resource::Command(ResourceData {
        // ...
    });

    let result = transformer.transform(&resource).unwrap();
    assert_eq!(
        result.path,
        PathBuf::from(DIR_AGENTS_SKILLS).join("test-cmd").join(SKILL_MD)
        // 변경 전: PathBuf::from(DIR_PROMPTS).join("test-cmd.md")
    );
    // content assertions는 변경 없음
}

/// Codex 타겟에서 Skill이 ../.agents/skills/[name]/SKILL.md 경로로 변환됨을 확인한다.
#[test]
fn test_codex_skill_transformation() {
    let transformer = CodexTransformer;
    let resource = Resource::Skill(SkillData {
        base: ResourceData {
            name: "test-skill".to_string(),
            // ...
        },
        extras: Vec::new(),
    });

    let result = transformer.transform(&resource).unwrap();
    assert_eq!(
        result.path,
        PathBuf::from(DIR_AGENTS_SKILLS).join("test-skill").join(SKILL_MD)
        // 변경 전(DefaultTransformer 위임): PathBuf::from(DIR_SKILLS).join("test-skill").join(SKILL_MD)
    );
    // content assertions는 변경 없음
}
```

### Step 5: `clean_all()`에서 `DIR_PROMPTS` 제거 (`src/builder/emitter.rs`)

`DIR_PROMPTS`가 삭제된 상수이므로 import와 `dirs` 배열에서 제거합니다.

```rust
// src/builder/emitter.rs
use crate::core::{
    AGENTS_MD, CLAUDE_MD, DIR_AGENTS, DIR_CODEX, DIR_COMMANDS, DIR_SKILLS, GEMINI_MD,
    TransformedResource,
    // DIR_PROMPTS 제거
};

pub fn clean_all(&self) -> Result<()> {
    let dirs = [DIR_CODEX, DIR_COMMANDS, DIR_AGENTS, DIR_SKILLS];
    // DIR_PROMPTS 제거. DIR_CODEX와 DIR_SKILLS는 Task 1.2·2.1에서 추가 처리 예정
    // ...
}
```

### Step 6: `specs/spec.md` 갱신

`specs/spec.md` 4절 변환 규격 테이블의 Codex 커맨드·스킬 컬럼을 갱신합니다.

```markdown
| 타겟 | 커맨드 변환 | 에이전트/스킬 변환 | 전역 지침 |
| :--- | :--- | :--- | :--- |
| **Codex** | `../.agents/skills/[name]/SKILL.md` (SKILL 포맷) | Commands: `../.agents/skills/[name]/SKILL.md`, Skills: `../.agents/skills/[name]/SKILL.md`, Agents: `agents/*.toml` | `AGENTS.md` |
```

### Step 7: `src/transformer/README.md` 갱신

`src/transformer/README.md`에서 Codex Command·Skill 경로를 언급하는 부분을 신규 경로로 갱신합니다.

- "Commands: `prompts/` 디렉터리에 생성" → "`../.agents/skills/[name]/SKILL.md`로 생성 (SKILL 포맷)"
- "Skills: `skills/[name]/SKILL.md`로 생성" → "`../.agents/skills/[name]/SKILL.md`로 생성 (Command와 동일 네임스페이스)"

## Success Criteria

- [x] `cargo test --lib transformer::codex` 테스트가 모두 통과한다.
- [x] `cargo build`가 `DIR_PROMPTS` 관련 컴파일 오류 없이 성공한다.
- [x] `cargo clippy -- -D warnings`가 오류 없이 통과한다.
- [x] Codex 타겟에서 Command가 `../.agents/skills/[name]/SKILL.md`로 변환된다.
- [x] Codex 타겟에서 Skill이 `../.agents/skills/[name]/SKILL.md`로 변환된다 (`.codex/skills/` 아님).
- [x] `specs/spec.md` Codex 커맨드·스킬 변환 컬럼이 `../.agents/skills/[name]/SKILL.md`로 갱신되었다.
- [x] `src/transformer/README.md`에서 `prompts/` 및 `skills/` 경로 언급이 신규 경로로 갱신되었다.
