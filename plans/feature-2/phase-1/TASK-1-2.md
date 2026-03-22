# Task 6.2: 소스 로딩 및 파일 출력 경로 분리 로직 구현

## 1. Objective (목표)

- 리소스를 로드하는 경로(Source)와 결과물을 생성하는 경로(Output)를 완전히 분리합니다.
- `main.rs`의 실행 흐름을 수정하여, `Config`에 정의된 `source` 경로에서 리소스를 읽고, `toolkit.yaml`이 위치한 경로에 결과물을 저장하도록 오케스트레이션합니다.

## 2. Context & Files (작업 범위)

- **읽기 전용 (참고용):**
  - `src/core/loader.rs` (리소스 로딩 유틸리티)
  - `src/emitter/mod.rs` (파일 출력 유틸리티)
- **수정할 파일:**
  - `src/main.rs` (전체 빌드 파이프라인 경로 제어 로직 수정)

## 3. Instructions (세부 지침)

### Step 1: 경로 변수 정의

`main.rs`의 `Build` 명령어 실행부에서 다음 두 경로를 명확히 구분하여 정의합니다.

- `output_dir`: `toolkit.yaml` 파일이 위치한 디렉터리 (현재의 `root_dir` 역할)
- `source_dir`: `config.source`에 명시된 절대 경로

### Step 2: 리소스 로딩 경로 변경

`core::loader::scan_plugins` 및 `load_resources` 호출 시, 기존의 `root_dir` 대신 `source_dir` (내부의 `plugins/` 폴더)을 사용하도록 수정합니다.

```rust
// 수정 전
let plugins_dir = root_dir.join("plugins");
// 수정 후
let source_path = std::path::Path::new(&cfg.source);
let plugins_dir = source_path.join("plugins");
```

### Step 3: AGENTS.md 로딩 경로 변경

전역 시스템 프롬프트인 `AGENTS.md`를 찾을 때도 `source_dir`을 기준으로 찾도록 수정합니다.

- `source_path.join("AGENTS.md")` 경로를 우선적으로 확인합니다.

### Step 4: 파일 출력 경로 유지

`emitter::Emitter::new(root_dir)` 호출은 그대로 유지하거나 명시적으로 `output_dir`을 사용하게 하여, 변환된 파일들이 `toolkit.yaml` 옆에 생성되도록 보장합니다.

## 4. Constraints (제약 사항 및 금지 행동)

- `source` 경로가 실제로 존재하는지 확인하고, 존재하지 않을 경우 사용자 친화적인 에러 메시지를 출력해야 합니다.
- `Emitter`의 `clean()` 메서드가 엉뚱한 디렉터리(예: 소스 디렉터리)를 삭제하지 않도록 `output_dir` 설정을 철저히 검증하세요.

## 5. Acceptance Criteria (검증 체크리스트)

1. 리소스 스캔이 `toolkit.yaml` 위치가 아닌 `source` 경로에서 수행되는가?
2. `AGENTS.md` 파일 역시 `source` 경로에서 정상적으로 로드되는가?
3. 빌드 결과물(`commands/`, `agents/`, `skills/` 등)이 `toolkit.yaml`과 같은 위치에 생성되는가?
4. 소스 경로가 유효하지 않을 때(존재하지 않는 경로 등) 적절한 에러를 반환하는가?
