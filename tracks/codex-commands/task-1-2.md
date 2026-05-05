# Task 1.2: config.toml 경로 버그 수정

## Overview

`post_transform()`이 `PathBuf::from(DIR_CODEX).join(CODEX_CONFIG_FILE_NAME)`으로 경로를 구성하는데, `TransformedFile.path`는 output-dir 기준 상대경로여야 하므로 `DIR_CODEX`(".codex") prefix가 붙으면 실제 경로가 `.codex/.codex/config.toml`이 됩니다. 올바른 경로는 output-dir 바로 아래인 `.codex/config.toml`이므로 `DIR_CODEX` prefix를 제거합니다. `detransform()`의 Agent 분기와 `Emitter::clean_all()`에서도 동일한 잘못된 참조를 수정합니다.

## Related Files

### Target Files

- `src/transformer/codex.rs`: 수정 — `post_transform()`, `detransform()` Agent 분기
- `src/builder/emitter.rs`: 수정 — `clean_all()`에서 `DIR_CODEX` 제거
- `src/transformer/README.md`: 수정 — `.codex/config.toml` 경로 설명 갱신

### Reference Files

- `src/transformer/codex.rs`: `post_transform()`, `detransform()` 현재 구현 확인
- `src/builder/emitter.rs`: `clean_all()` 현재 구현 확인
- `src/transformer/README.md`: 현재 `.codex/config.toml` 경로 언급 확인

## Workflow

### Step 1: `post_transform()` 경로 수정 (`src/transformer/codex.rs`)

`TransformedFile.path`는 output-dir 기준 상대경로입니다. output-dir 자신인 `.codex`를 prefix로 붙이면 Emitter가 `output_dir.join(".codex/config.toml")`로 최종 경로를 조합하여 이중 경로가 됩니다.

```rust
fn post_transform(&self, resources: &[&Resource]) -> Result<Vec<TransformedFile>> {
    // ...agents_table 구성 로직 변경 없음...

    let content = toml::to_string_pretty(&root_table)?;
    // 변경 전: let path = PathBuf::from(DIR_CODEX).join(CODEX_CONFIG_FILE_NAME);
    let path = PathBuf::from(CODEX_CONFIG_FILE_NAME);
    // 결과: "config.toml" (output-dir 기준 상대경로)

    Ok(vec![TransformedFile { path, content }])
}
```

`DIR_CODEX` import가 `post_transform()` 외에도 `detransform()`에서 사용되므로, Step 2까지 완료한 후 사용처가 없어지면 import에서 제거합니다.

### Step 2: `detransform()` Agent 분기 `config_path` 수정 (`src/transformer/codex.rs`)

`detransform()`은 output-dir의 절대경로(`output_dir: &Path`)를 인수로 받습니다. `config.toml`은 output-dir 바로 아래에 있으므로 `DIR_CODEX` 단계를 제거합니다.

```rust
ResourceType::Agent => {
    // ...TOML 파싱 로직 변경 없음...

    // 변경 전: let config_path = output_dir.join(DIR_CODEX).join(CODEX_CONFIG_FILE_NAME);
    let config_path = output_dir.join(CODEX_CONFIG_FILE_NAME);
    // 결과: .codex/config.toml (output_dir이 이미 .codex/의 절대경로)

    // ...config 파일 파싱 및 description 복원 로직 변경 없음...
}
```

Step 1, 2 완료 후 `DIR_CODEX`의 사용처가 없으면 import에서 제거합니다.

```rust
use crate::core::{
    AGENTS_MD, BuildTarget, CODEX_CONFIG_FILE_NAME, DIR_AGENTS,
    // DIR_CODEX 제거
    DIR_AGENTS_SKILLS, EXT_TOML,
    Resource, ResourceData, ResourceType, SKILL_MD, TransformedFile,
};
```

### Step 3: `clean_all()`에서 `DIR_CODEX` 제거 (`src/builder/emitter.rs`)

`DIR_CODEX`(".codex")는 output-dir 자신이므로 `clean_all()` 대상에 포함하면 output-dir 전체가 삭제됩니다. 이는 의도하지 않은 동작이므로 제거합니다.

```rust
pub fn clean_all(&self) -> Result<()> {
    // 변경 전: let dirs = [DIR_CODEX, DIR_COMMANDS, DIR_AGENTS, DIR_SKILLS];
    let dirs = [DIR_COMMANDS, DIR_AGENTS, DIR_SKILLS];
    // DIR_CODEX 제거: output-dir 자신을 삭제하는 것은 잘못된 동작
    // ...나머지 로직 변경 없음...
}
```

`DIR_CODEX` import도 `emitter.rs`의 사용처가 없어지면 제거합니다.

### Step 4: 단위 테스트 수정 (`src/transformer/codex.rs`)

`test_codex_post_transform` 테스트의 `config_file.path` assertion을 수정합니다.

```rust
#[test]
fn test_codex_post_transform() {
    // ...resources 구성 변경 없음...

    let config_file = &result[0];
    // 변경 전: assert_eq!(config_file.path, PathBuf::from(".codex/config.toml"));
    assert_eq!(config_file.path, PathBuf::from(CODEX_CONFIG_FILE_NAME));
    // ...agents 내용 검증은 변경 없음...
}
```

### Step 5: `src/transformer/README.md` 갱신

`src/transformer/README.md`에서 `.codex/config.toml` 경로를 언급하는 두 곳을 수정합니다.

- `codex.rs` 설명: "`.codex/config.toml` 자동 생성" → "`config.toml` 자동 생성 (output-dir 기준)"
- Codex Agent 변환 설명: "`.codex/config.toml` (설명 등 메타데이터 포함)에 취합" → "`config.toml` (output-dir 기준)에 취합"

## Success Criteria

- [x] `cargo test --lib transformer::codex` 테스트가 모두 통과한다.
- [x] `cargo test --lib builder::emitter` 테스트가 모두 통과한다.
- [x] `cargo clippy -- -D warnings`가 오류 없이 통과한다.
- [x] `post_transform()` 반환 결과의 `path`가 `PathBuf::from("config.toml")`이다.
- [x] `detransform()` Agent 분기에서 `config.toml` 읽기 경로가 `output_dir.join("config.toml")`이다.
- [x] `src/transformer/README.md`에서 `.codex/config.toml` 언급이 올바른 경로로 갱신되었다.
