# Task 3.1: output-dir 검증 추가

## Overview

`AppContext::init()`에서 `toolkit.yaml`이 위치한 디렉터리(output-dir) 이름이 타겟의 기대값과 일치하는지 검증합니다. 불일치 시 `anyhow::bail!`로 즉시 종료하여 잘못된 위치에서의 빌드를 방지합니다. `BuildTarget`에 `expected_output_dir()` 메서드를 추가하여 타겟별 기대 디렉터리명을 관리합니다.

## Related Files

### Target Files

- `src/core/target.rs`: 수정 — `BuildTarget::expected_output_dir()` 메서드 및 단위 테스트 추가
- `src/app/context.rs`: 수정 — output-dir 이름 검증 로직 추가 (AppContext 통합 테스트 포함)
- `tests/e2e_build_test.rs`: 수정 — 모든 타겟 테스트를 dot-dir 내 `toolkit.yaml` 구조로 재편
- `tests/e2e_codex_sync_test.rs`: 수정 — `.codex/toolkit.yaml` 구조로 재편

### Reference Files

- `src/core/target.rs`: `BuildTarget` enum 및 기존 메서드(`as_str()`) 구현 확인
- `src/app/context.rs`: `AppContext::init()` — `output_dir` 결정 및 `cfg` 로드 순서 확인
- `tests/e2e_build_test.rs`: 현재 `toolkit.yaml` 위치 및 구조 확인
- `tests/e2e_codex_sync_test.rs`: 현재 `toolkit.yaml` 위치 및 구조 확인

## Workflow

### Step 1: `BuildTarget::expected_output_dir()` 메서드 추가 (`src/core/target.rs`)

```rust
// src/core/target.rs
impl BuildTarget {
    pub fn expected_output_dir(&self) -> &'static str {
        match self {
            BuildTarget::Codex => ".codex",
            BuildTarget::ClaudeCode => ".claude",
            BuildTarget::GeminiCli => ".gemini",
            BuildTarget::OpenCode => ".opencode",
        }
    }
}
```

### Step 2: `AppContext::init()`에 검증 로직 추가 (`src/app/context.rs`)

config를 로드한 직후, source_dir 유효성 검사 이전에 output-dir 이름 검증을 수행합니다.

**상대경로 문제 주의:** `config_file`이 `"toolkit.yaml"`처럼 상대경로로 전달되면 `Path::new("toolkit.yaml").parent()`가 빈 문자열(`""`)을 반환하고, `file_name()`이 `None`이 되어 검증이 항상 실패합니다. 따라서 `canonicalize()`로 절대경로로 변환한 뒤 `parent()`를 호출해야 합니다. `canonicalize()`는 파일이 실제로 존재할 때만 성공하므로, 존재 여부 확인 이후에 호출합니다.

```rust
// src/app/context.rs
pub fn init(config_file: &str) -> anyhow::Result<Self> {
    let config_path = Path::new(config_file);
    if !config_path.exists() {
        anyhow::bail!("Config file not found: {}", config_file);
    }
    // 상대경로("toolkit.yaml")의 parent()가 ""를 반환하는 문제를 방지하기 위해 절대경로로 변환한다.
    let abs_config_path = config_path.canonicalize()?;
    let output_dir = abs_config_path.parent().unwrap_or(Path::new(".")).to_path_buf();

    let cfg = load_config(config_file)?;
    let source_dir = PathBuf::from(&cfg.source);

    // output-dir 이름이 타겟의 기대값과 일치하는지 검증한다.
    let actual = output_dir.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    let expected = cfg.target.expected_output_dir();
    if actual != expected {
        anyhow::bail!(
            "output-dir 이름이 타겟 '{}'에 맞지 않습니다. 기대값: '{}', 실제값: '{}'",
            cfg.target.as_str(), expected, actual
        );
    }

    if !source_dir.exists() {
        anyhow::bail!("Source directory does not exist: {}", source_dir.display());
    }

    // ...이하 기존 로직 변경 없음...
}
```

### Step 3: 단위 테스트 추가 (`src/core/target.rs`)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    /// 각 BuildTarget의 expected_output_dir()가 올바른 디렉터리명을 반환하는지 확인한다.
    #[test]
    fn test_expected_output_dir() {
        assert_eq!(BuildTarget::Codex.expected_output_dir(), ".codex");
        assert_eq!(BuildTarget::ClaudeCode.expected_output_dir(), ".claude");
        assert_eq!(BuildTarget::GeminiCli.expected_output_dir(), ".gemini");
        assert_eq!(BuildTarget::OpenCode.expected_output_dir(), ".opencode");
    }
}
```

### Step 4: `AppContext::init()` 통합 테스트 추가 (`src/app/context.rs`)

검증 로직은 실제 파일 시스템과 config 로드에 의존하므로 `tempdir` 기반 통합 테스트로 검증합니다.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn write_toolkit_yaml(dir: &std::path::Path, target: &str, source: &str) {
        /// 테스트용 최소 toolkit.yaml 파일을 생성하는 헬퍼.
        let content = format!("source: {}\ntarget: {}\nresources: {{}}\n", source, target);
        std::fs::write(dir.join("toolkit.yaml"), content).unwrap();
    }

    /// .codex/toolkit.yaml에 target: codex가 설정된 경우 검증이 통과함을 확인한다.
    #[test]
    fn test_init_succeeds_with_matching_output_dir() {
        // ...
    }

    /// .codex/toolkit.yaml에 target: claude-code가 설정된 경우 output-dir 불일치 오류가 발생함을 확인한다.
    #[test]
    fn test_init_fails_with_mismatched_output_dir() {
        // ...
    }

    /// 상대경로 "toolkit.yaml"로 호출 시 canonicalize 처리로 검증이 올바르게 동작함을 확인한다.
    #[test]
    fn test_init_with_relative_config_path() {
        // ...
    }
}
```

### Step 5: E2E 테스트 재편 (`tests/e2e_build_test.rs`, `tests/e2e_codex_sync_test.rs`)

output-dir 검증이 추가되면 `toolkit.yaml`을 프로젝트 루트에 두는 기존 E2E 테스트 구조가 전부 실패합니다. 각 타겟의 dot-dir 내에 `toolkit.yaml`을 배치하도록 모든 E2E 테스트를 재편합니다.

**`tests/e2e_build_test.rs` 변경 패턴:**

```rust
// 변경 전: toolkit.yaml을 root에 배치
fs::write(root.join("toolkit.yaml"), config).unwrap();
cmd.arg("build").arg("--config").arg(root.join("toolkit.yaml"));

// 변경 후: 타겟 dot-dir 내에 배치
// (예: gemini-cli의 경우)
let output_dir = root.join(".gemini");
fs::create_dir_all(&output_dir).unwrap();
fs::write(output_dir.join("toolkit.yaml"), config).unwrap();
cmd.arg("build").arg("--config").arg(output_dir.join("toolkit.yaml"));
```

빌드 결과물 경로도 `root.join("commands/...")` → `output_dir.join("commands/...")` 등으로 갱신합니다.

**`tests/e2e_codex_sync_test.rs` 변경 패턴:**

```rust
// 변경 전
fs::write(root.join("toolkit.yaml"), config).unwrap();
// 변경 후
let codex_dir = root.join(".codex");
fs::create_dir_all(&codex_dir).unwrap();
fs::write(codex_dir.join("toolkit.yaml"), config).unwrap();
cmd.arg("build").arg("--config").arg(codex_dir.join("toolkit.yaml"));
```

Task 2.1에서 갱신한 `prompts/` → `.agents/skills/`, `.codex/config.toml` → `config.toml` assertion도 함께 적용합니다.

## Success Criteria

- [x] `cargo test --lib core::target` 테스트가 모두 통과한다.
- [x] `cargo test --lib app::context` 통합 테스트가 모두 통과한다.
- [x] `cargo test --test e2e_build_test` 가 모두 통과한다.
- [x] `cargo test --test e2e_codex_sync_test` 가 모두 통과한다.
- [x] `cargo build`가 성공한다.
- [x] `cargo clippy -- -D warnings`가 오류 없이 통과한다.
- [x] `.codex/toolkit.yaml`에 `target: claude-code`가 설정된 경우, `AppContext::init()` 호출 시 `"output-dir 이름이 타겟 'claude-code'에 맞지 않습니다"` 오류와 함께 실패한다.
- [x] `.codex/toolkit.yaml`에 `target: codex`가 설정된 경우, `AppContext::init()`이 정상적으로 계속 진행한다.
- [x] `atb build`를 `.codex/` 내부에서 (기본값 `"toolkit.yaml"` 상대경로로) 실행했을 때 검증이 정상 동작한다.
