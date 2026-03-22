# TASK: BuildTarget Serialization Utilities

## 개요
`overrides.yaml` 파싱 시 YAML 키(문자열)를 `BuildTarget` 열거형으로 변환하거나 그 반대의 작업을 원활하게 하기 위한 유틸리티를 강화합니다.

## 작업 상세

### 1. `BuildTarget` 유틸리티 추가 (`src/core/target.rs`)
- 이미 `as_str()`과 `serde` 속성이 존재하지만, 명시적으로 문자열로부터 `BuildTarget`을 얻는 기능을 확인하거나 추가합니다.
- `FromStr` 트레이트를 구현하는 것이 좋습니다.

```rust
use std::str::FromStr;

impl FromStr for BuildTarget {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            TARGET_GEMINI => Ok(BuildTarget::GeminiCli),
            TARGET_CLAUDE => Ok(BuildTarget::ClaudeCode),
            TARGET_OPENCODE => Ok(BuildTarget::OpenCode),
            _ => anyhow::bail!("Unknown build target: {}", s),
        }
    }
}
```

## 검증 방법
- `src/core/target.rs`에 단위 테스트를 추가하여 문자열로부터 `BuildTarget`이 올바르게 생성되는지 확인합니다.
