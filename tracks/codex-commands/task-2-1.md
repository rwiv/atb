# Task 2.1: Emitter 타겟 인식, clean 로직 수정 및 충돌 검증 추가

## Overview

`Emitter`에 `BuildTarget` 정보를 추가하여 타겟별 clean 동작을 분기합니다. Codex 타겟이면 `clean_all()` 시 output-dir 외부의 `../.agents/skills/` 디렉터리를 추가 삭제합니다. 또한 `clean()` 분기 조건에 `ends_with(SKILL_MD)` 조건을 추가하여, Codex 커맨드/스킬 경로(`../.agents/skills/[name]/SKILL.md`)처럼 `starts_with(DIR_SKILLS)` 조건을 통과하지 못하는 SKILL.md 경로도 디렉터리 단위로 올바르게 삭제되도록 합니다. `Builder::run()`과 `App::build()`의 시그니처를 조정하여 타겟 정보를 `Emitter`까지 전달하며, `Builder::run()` 초반에 Codex 타겟 한정으로 Command/Skill 이름 충돌 검증을 추가합니다.

## Related Files

### Target Files

- `src/builder/emitter.rs`: 수정 — `Emitter` 구조체, `new()`, `clean_all()`, `clean()`
- `src/builder/mod.rs`: 수정 — `Builder::run()` 시그니처에 `target` 추가, `Emitter::new()` 호출부, 충돌 검증 로직 추가
- `src/app/mod.rs`: 수정 — `builder.run()` 호출부에 `ctx.config.target` 전달
- `src/builder/README.md`: 수정 — `Emitter` 동작 설명 갱신 (타겟 필드, `clean_all()` 새 동작), 충돌 검증 동작 설명 추가
- `tests/e2e_codex_sync_test.rs`: 수정 — 신규 경로로 assertion 갱신

### Reference Files

- `src/core/target.rs`: `BuildTarget` enum 정의 확인
- `src/core/constants.rs`: `DIR_AGENTS_SKILLS`, `SKILL_MD` 상수 확인
- `src/builder/emitter.rs`: 현재 `Emitter` 구조체 및 `clean()`, `clean_all()` 구현 확인
- `src/builder/mod.rs`: `Builder::run()` 시그니처 및 `Emitter::new()` 호출부 확인
- `src/app/mod.rs`: `App::build()` — `builder.run()` 호출부 확인
- `src/builder/README.md`: 현재 `clean_all()` 설명 확인
- `tests/e2e_codex_sync_test.rs`: 현재 테스트 경로 assertion 확인

## Workflow

### Step 1: `Emitter` 구조체 및 `new()` 수정 (`src/builder/emitter.rs`)

```rust
// src/builder/emitter.rs
use crate::core::{
    AGENTS_MD, CLAUDE_MD, BuildTarget, DIR_AGENTS, DIR_AGENTS_SKILLS, DIR_COMMANDS, DIR_SKILLS,
    GEMINI_MD, SKILL_MD, TransformedResource,
};

pub struct Emitter {
    output_path: PathBuf,
    target: BuildTarget,
}

impl Emitter {
    pub fn new(output_path: impl Into<PathBuf>, target: BuildTarget) -> Self {
        Self {
            output_path: output_path.into(),
            target,
        }
    }
```

### Step 2: `clean_all()` 수정 (`src/builder/emitter.rs`)

Codex 타겟이면 output-dir 외부의 `../.agents/skills/` 디렉터리를 추가 삭제합니다. `output_path.join(DIR_AGENTS_SKILLS)`는 `PathBuf` 레벨에서 `../` 경로 참조를 보존하므로 OS가 올바르게 절대경로로 해석합니다.

```rust
/// commands/, agents/, skills/ 등 모든 출력 디렉터리와 전역 파일을 전부 삭제합니다.
/// Codex 타겟이면 output-dir 외부의 `../.agents/skills/` 디렉터리도 추가 삭제합니다.
pub fn clean_all(&self) -> Result<()> {
    let dirs = [DIR_COMMANDS, DIR_AGENTS, DIR_SKILLS];

    for dir in dirs {
        let path = self.output_path.join(dir);
        if path.exists() {
            fs::remove_dir_all(&path)
                .with_context(|| format!("Failed to remove directory: {:?}", path))?;
        }
    }

    // Codex 타겟은 output-dir 외부의 ../.agents/skills/ 디렉터리도 삭제한다.
    if self.target == BuildTarget::Codex {
        let codex_commands_path = self.output_path.join(DIR_AGENTS_SKILLS);
        if codex_commands_path.exists() {
            fs::remove_dir_all(&codex_commands_path)
                .with_context(|| format!("Failed to remove directory: {:?}", codex_commands_path))?;
        }
    }

    let files = [GEMINI_MD, CLAUDE_MD, AGENTS_MD];
    for file in files {
        let path = self.output_path.join(file);
        if path.exists() {
            fs::remove_file(&path)
                .with_context(|| format!("Failed to remove file: {:?}", path))?;
        }
    }

    Ok(())
}
```

### Step 3: `clean()` 분기 조건 수정 (`src/builder/emitter.rs`)

`../.agents/skills/[name]/SKILL.md` 경로는 `starts_with(DIR_SKILLS)`를 충족하지 않습니다. `SKILL.md`로 끝나는 경로는 항상 부모 디렉터리 단위로 삭제해야 하므로 OR 조건을 추가합니다.

```rust
/// 빌드 대상 리소스에 해당하는 파일/디렉터리를 선택적으로 삭제합니다.
/// 전역 파일(GEMINI.md, CLAUDE.md, AGENTS.md)은 항상 삭제됩니다.
pub fn clean(&self, resources: &[TransformedResource]) -> Result<()> {
    // ...전역 파일 삭제 로직 변경 없음...

    for resource in resources {
        let Some(first_file) = resource.files.first() else { continue; };

        // 변경 전: if first_file.path.starts_with(DIR_SKILLS) { ... }
        if first_file.path.starts_with(DIR_SKILLS) || first_file.path.ends_with(SKILL_MD) {
            // Skill 및 SKILL.md로 끝나는 경로(Codex 커맨드 포함): 부모 디렉터리 전체 삭제
            if let Some(skill_dir) = first_file.path.parent() {
                let full_path = self.output_path.join(skill_dir);
                if full_path.exists() {
                    fs::remove_dir_all(&full_path)
                        .with_context(|| format!("Failed to remove directory: {:?}", full_path))?;
                }
            }
        } else {
            // ...개별 파일 삭제 로직 변경 없음...
        }
    }
    Ok(())
}
```

### Step 4: `Builder::run()` 시그니처 수정 및 충돌 검증 추가 (`src/builder/mod.rs`)

```rust
// src/builder/mod.rs
use crate::core::{AGENTS_MD, BuildTarget, TransformedResource};
use std::collections::HashSet;

impl Builder {
    pub fn run(
        &self,
        transformer: &dyn Transformer,
        registry: &Registry,
        source_dir: &Path,
        output_dir: &Path,
        target: BuildTarget,    // 추가
        full_clean: bool,
    ) -> anyhow::Result<()> {
        // Codex 타겟: Command/Skill 이름 충돌 검증
        if target == BuildTarget::Codex {
            let command_names: HashSet<&str> = registry.commands().iter().map(|c| c.name()).collect();
            let skill_names: HashSet<&str> = registry.skills().iter().map(|s| s.name()).collect();
            let mut collisions: Vec<&&str> = command_names.intersection(&skill_names).collect();
            if !collisions.is_empty() {
                collisions.sort();
                anyhow::bail!(
                    "Codex 타겟에서 command와 skill의 이름이 충돌합니다: {}",
                    collisions.iter().map(|s| format!("'{}'", s)).collect::<Vec<_>>().join(", ")
                );
            }
        }

        // ...변환 로직 변경 없음...

        let emitter = Emitter::new(output_dir, target);  // target 전달
        // ...나머지 로직 변경 없음...
    }
}
```

`registry.commands()`와 `registry.skills()`의 실제 API는 `src/loader/registry.rs` 구현을 확인하여 맞춰야 합니다.

### Step 5: `App::build()` 호출부 수정 (`src/app/mod.rs`)

```rust
// src/app/mod.rs
fn build(&self, ctx: &AppContext, full_clean: bool) -> anyhow::Result<()> {
    let builder = Builder::new();
    builder.run(
        ctx.transformer.as_ref(),
        &ctx.registry,
        &ctx.source_dir,
        &ctx.output_dir,
        ctx.config.target,   // 추가
        full_clean,
    )?;
    // ...
}
```

### Step 6: `tests/e2e_codex_sync_test.rs` 갱신

Task 1.1, 1.2 코드 변경으로 E2E sync 테스트의 경로 assertion이 깨집니다. 이 단계에서 함께 갱신합니다.

변경이 필요한 assertion:
- `root.join("prompts/foo.md").exists()` → `root.join(".agents/skills/foo/SKILL.md").exists()`
- `let cmd_md_path = root.join("prompts/foo.md")` → `root.join(".agents/skills/foo/SKILL.md")`
- `root.join(".codex/config.toml")` → `root.join("config.toml")`

sync 검증 흐름도 갱신합니다. 커맨드 수정 후 `atb sync`가 원본 command를 갱신하는지 확인하고, config.toml의 agent description sync도 새 위치(`root.join("config.toml")`) 기준으로 검증합니다.

### Step 7: 단위 테스트 수정 및 신규 테스트 추가 (`src/builder/emitter.rs`)

기존 테스트에서 `Emitter::new()` 호출에 타겟 인수를 추가하고, 신규 케이스를 작성합니다.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{BuildTarget, ExtraFile, TransformedFile};
    use std::fs;
    use tempfile::tempdir;

    /// 기본 테스트용 Emitter (ClaudeCode 타겟)를 생성하는 헬퍼.
    fn make_emitter(root: &std::path::Path) -> Emitter {
        Emitter::new(root, BuildTarget::ClaudeCode)
    }

    /// Codex 타겟의 clean_all() 호출 시 output-dir 외부의 ../.agents/skills/ 디렉터리가 삭제됨을 확인한다.
    #[test]
    fn test_clean_all_removes_codex_commands_dir() -> Result<()> {
        // ...
    }

    /// SKILL.md로 끝나는 외부 경로(../.agents/skills/[name]/SKILL.md)가 부모 디렉터리 단위로 삭제됨을 확인한다.
    #[test]
    fn test_clean_removes_parent_dir_for_external_skill_path() -> Result<()> {
        // ...
    }

    /// Codex가 아닌 타겟에서 clean_all() 호출 시 ../.agents/skills/ 디렉터리가 삭제되지 않음을 확인한다.
    #[test]
    fn test_clean_all_does_not_remove_codex_commands_for_non_codex_target() -> Result<()> {
        // ...
    }
}
```

충돌 검증 단위 테스트(`src/builder/mod.rs`):

```rust
#[cfg(test)]
mod tests {
    use super::*;

    /// 동일 이름의 command와 skill이 있을 때 Codex 타겟에서 에러가 반환됨을 확인한다.
    #[test]
    fn test_codex_build_fails_on_name_collision() {
        // registry에 "foo" command와 "foo" skill이 모두 존재하는 상황 구성
        // builder.run(..., BuildTarget::Codex, ...) 호출 시 에러 확인
        // 에러 메시지에 'foo' 포함 여부 확인
    }

    /// 이름 충돌이 없을 때 Codex 타겟에서 정상 진행됨을 확인한다.
    #[test]
    fn test_codex_build_succeeds_without_name_collision() {
        // registry에 이름이 겹치지 않는 command와 skill이 존재하는 상황 구성
        // builder.run(..., BuildTarget::Codex, ...) 호출 시 Ok 확인
    }

    /// Codex가 아닌 타겟에서는 이름 충돌 검증이 실행되지 않음을 확인한다.
    #[test]
    fn test_non_codex_build_skips_collision_check() {
        // registry에 동일 이름의 command와 skill이 있더라도
        // BuildTarget::ClaudeCode 타겟이면 충돌 검증 에러가 발생하지 않음을 확인
    }
}
```

기존 테스트들은 `make_emitter()` 헬퍼를 활용하거나 직접 `Emitter::new(root, BuildTarget::ClaudeCode)`로 수정합니다.

### Step 8: `src/builder/README.md` 갱신

`src/builder/README.md`의 `Emitter` 설명을 갱신합니다.

- `clean_all()` 설명: `.codex/`, `prompts/` 삭제 언급 제거. Codex 타겟이면 `../.agents/skills/` 추가 삭제 동작 명시.
- `Emitter` 구조체 설명에 `target: BuildTarget` 필드 추가 설명.
- `Builder::run()` 설명에 Codex 타겟 한정 충돌 검증 동작 추가.

## Success Criteria

- [x] `cargo test --lib builder::emitter` 테스트가 모두 통과한다.
- [x] `cargo test --lib builder` 테스트가 모두 통과한다.
- [x] `cargo test --test e2e_codex_sync_test` 가 통과한다.
- [x] `cargo build`가 성공한다.
- [x] `cargo clippy -- -D warnings`가 오류 없이 통과한다.
- [x] Codex 타겟으로 `clean_all()` 호출 시 `../.agents/skills/` 디렉터리가 삭제된다.
- [x] `../.agents/skills/foo/SKILL.md` 경로를 가진 리소스에 대해 `clean()` 호출 시 `../.agents/skills/foo/` 디렉터리 전체가 삭제된다.
- [x] ClaudeCode 타겟으로 `clean_all()` 호출 시 `../.agents/skills/`가 삭제되지 않는다.
- [x] Codex 타겟에서 동일 이름의 command와 skill이 있으면 빌드가 충돌 오류와 함께 종료된다.
- [x] Codex 타겟에서 이름 충돌이 없으면 빌드가 정상 진행된다.
- [x] ClaudeCode 타겟에서는 동일 이름의 command와 skill이 있어도 충돌 검증 에러가 발생하지 않는다.
- [x] `src/builder/README.md`의 `clean_all()` 및 충돌 검증 설명이 최신 동작과 일치한다.
