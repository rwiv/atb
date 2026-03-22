# Task 1.2: toolkit.yaml 파싱 및 설정 모델링

## 1. Objective (목표)

- `tests/fixtures/toolkit.yaml` 파일을 읽어 Rust 데이터 구조체로 정확히 역직렬화(Deserialization)하는 기능을 구현합니다.
- 빌드 대상 에이전트(`target`), 제외 패턴(`exclude`), 포함할 리소스(`resources`) 목록을 체계적으로 관리할 수 있는 기초를 마련합니다.

## 2. Context & Files (작업 범위)

- **읽기 전용 (참고용):**
  - `specs/SPEC.md` (toolkit.yaml 구조 및 필드 정의 확인)
  - `specs/SPEC.md` (사용 라이브러리 및 모듈 구조 확인)
- **생성 및 수정할 파일:**
  - `src/config.rs` (신규 생성: 설정 데이터 모델 및 로드 로직)
  - `src/main.rs` (수정: CLI 실행 시 설정 파일 로드 테스트)
  - `tests/fixtures/toolkit.yaml` (테스트용 샘플 파일 생성)

## 3. Instructions (세부 지침)

### Step 1: `Config` 구조체 및 데이터 모델 정의 (`src/config.rs`)

`serde`를 활용하여 `toolkit.yaml`의 구조를 반영하는 모델을 설계하세요.

- **Target Enum:** `target` 필드는 아래의 값만 허용하는 `enum`으로 정의합니다.
  - `gemini-cli`, `claude-code`, `opencode`
- **Resources Struct:** `commands`, `agents`, `skills` 목록을 각각 `Vec<String>` 형태로 가집니다.
- **Config Struct:** 전체 설정을 담는 최상위 구조체입니다.

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum BuildTarget {
    #[serde(rename = "gemini-cli")]
    GeminiCli,
    #[serde(rename = "claude-code")]
    ClaudeCode,
    #[serde(rename = "opencode")]
    OpenCode,
}

#[derive(Debug, Deserialize)]
pub struct Resources {
    pub commands: Option<Vec<String>>,
    pub agents: Option<Vec<String>>,
    pub skills: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub target: BuildTarget,
    pub exclude: Option<Vec<String>>,
    pub resources: Resources,
}
```

### Step 2: 설정 로드 로직 구현

파일 시스템에서 `tests/fixtures/toolkit.yaml`을 읽어 `Config` 객체를 반환하는 함수를 구현하세요.

- 함수 시그니처: `pub fn load_config<P: AsRef<Path>>(path: P) -> anyhow::Result<Config>`
- `serde_yaml::from_reader` 또는 `from_str`을 사용하여 파싱을 수행합니다.
- 파일이 존재하지 않거나, YAML 문법이 틀렸거나, 필수 필드(`target`, `resources`)가 누락된 경우 `anyhow`를 통해 명확한 에러를 반환해야 합니다.

### Step 3: `main.rs` 연동 및 검증

- `main.rs`에서 `agb build` 실행 흐름 초입에 `load_config`를 호출하도록 수정합니다.
- 로드된 `Config` 객체를 `println!("{:?}", config)` 등을 통해 출력하여 정상 작동 여부를 확인합니다.

## 4. Constraints (제약 사항 및 금지 행동)

- 반드시 `serde`와 `serde_yaml` 라이브러리를 사용하세요.
- 설정 파일 이름은 `toolkit.yaml`이며, 기본 경로는 `tests/fixtures/toolkit.yaml`입니다.
- `target` 필드에 정의되지 않은 문자열이 들어올 경우 파싱 단계에서 에러가 발생해야 합니다.

## 5. Acceptance Criteria (검증 체크리스트)

1. `tests/fixtures/toolkit.yaml`의 모든 필드(target, exclude, resources)가 Rust 구조체로 유실 없이 매핑되는가?
2. `target` 값이 `gemini-cli`, `claude-code`, `opencode` 중 하나가 아닐 때 적절한 에러를 반환하는가?
3. `exclude`나 `resources` 내의 리스트가 비어있거나 생략되었을 때(Optional) 에러 없이 기본값(None 또는 빈 Vec)으로 처리되는가?
4. `cargo check` 및 빌드 시 타입 관련 경고나 에러가 없는가?
