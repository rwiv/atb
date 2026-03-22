# Task 6.1: toolkit.yaml 모델 확장 및 source 필드 추가

## 1. Objective (목표)

- `toolkit.yaml` 설정 파일에 필수 필드인 `source`를 추가하여, 리소스(Plugins, AGENTS.md)가 위치한 절대 경로를 명시할 수 있도록 합니다.
- Rust의 `Config` 구조체를 업데이트하여 새로운 필드를 지원하고, 파싱 시 유효성을 검사합니다.

## 2. Context & Files (작업 범위)

- **읽기 전용 (참고용):**
  - `specs/SPEC.md` (설정 구조 변경 내역 확인)
  - `src/config.rs` (현재 Config 모델 구현체)
- **수정할 파일:**
  - `src/config.rs` (Config 구조체 및 파싱 로직 수정)
  - `tests/fixtures/toolkit.yaml` (테스트용 샘플 파일에 source 필드 추가)

## 3. Instructions (세부 지침)

### Step 1: `Config` 구조체에 `source` 필드 추가

`src/config.rs` 파일의 `Config` 구조체에 `source` 필드를 추가합니다.

- **타입:** `String` (절대 경로를 담기 위함)
- **필수 여부:** 필수 (Option이 아님)

```rust
#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    pub source: String, // 추가된 필드: 소스 리소스의 절대 경로
    pub target: BuildTarget,
    pub exclude: Option<Vec<String>>,
    pub resources: Resources,
}
```

### Step 2: 파싱 및 유효성 검사 로직 업데이트

`parse_config` 함수에서 `source` 필드가 올바르게 로드되는지 확인합니다. 또한, `source` 필드가 비어있거나 누락되었을 때 `serde`가 적절한 에러를 발생시키는지 확인합니다.

### Step 3: 단위 테스트 업데이트

`src/config.rs`의 테스트 코드를 수정하여 `source` 필드가 포함된 YAML이 정상적으로 파싱되는지 검증합니다.

```rust
#[test]
fn test_full_config_parsing() {
    let yaml = r#"
source: /absolute/path/to/source
target: gemini-cli
resources:
  commands:
    - p1:cmd1
"#;
    let config = parse_config(yaml).unwrap();
    assert_eq!(config.source, "/absolute/path/to/source");
    // ... 기존 검증 로직
}
```

## 4. Constraints (제약 사항 및 금지 행동)

- `source` 필드는 반드시 **절대 경로** 문자열이어야 합니다. (이 단계에서는 문자열 파싱만 확인하며, 경로의 실제 존재 여부는 다음 태스크에서 다룹니다.)
- 기존의 `target`, `resources` 필드와의 하위 호환성을 고려하되, `source`는 필수 필드로 처리합니다.

## 5. Acceptance Criteria (검증 체크리스트)

1. `toolkit.yaml`에 `source` 필드가 있을 때 `Config` 구조체로 정확히 매핑되는가?
2. `source` 필드가 누락되었을 때 파싱 에러가 발생하는가?
3. `cargo test`를 실행하여 `config.rs`의 모든 테스트가 통과하는가?
