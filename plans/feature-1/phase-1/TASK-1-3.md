# Task 1.3: CLI 명령 체계 구축

## 1. Objective (목표)

- `clap` 라이브러리를 사용하여 `agb` CLI의 서브커맨드 구조를 정의합니다.
- `build` 명령어를 구현하고, 실행 시 `toolkit.yaml` 파일을 찾아 로드하는 기본 파이프라인을 연결합니다.
- CLI의 정상 동작을 보장하기 위해 자동화된 통합 테스트를 작성합니다.

## 2. Context & Files (작업 범위)

- **읽기 전용 (참고용):**
  - `specs/SPEC.md` (CLI 요구 사항 확인)
  - `specs/SPEC.md` (`clap` 사용 및 모듈 구조 확인)
- **수정할 파일:**
  - `src/main.rs` (CLI 엔트리포인트 및 커맨드 핸들링)
- **생성할 파일:**
  - `tests/cli_test.rs` (CLI 동작 검증을 위한 통합 테스트)

## 3. Instructions (세부 지침)

### Step 1: `clap`을 이용한 CLI 구조 설계 (`src/main.rs`)

`clap`의 `Parser` 및 `Subcommand` derive 매크로를 사용하여 명령 체계를 구성하세요.

- **Cli 구조체:** 최상위 파서로 정의합니다.
- **Commands 열거형:** `Build` 서브커맨드를 포함합니다.
- **Build 인자:**
  - `--config` (또는 `-c`): 설정 파일 경로를 지정합니다. 기본값은 `tests/fixtures/toolkit.yaml`로 설정하세요.

### Step 2: `build` 명령 실행 로직 연결

- `main` 함수에서 `Cli::parse()`를 호출하여 사용자 입력을 분석합니다.
- `Commands::Build`가 매칭되면 `src/config.rs`의 `load_config` 함수를 호출합니다.
- 설정 로드에 성공하면 타겟 에이전트 정보(`target`)를 포함한 성공 메시지를 출력합니다.
- 실패 시 `anyhow` 에러를 전파하여 CLI가 적절한 에러 메시지와 함께 종료되도록 합니다.

### Step 3: CLI 통합 테스트 구현 (`tests/cli_test.rs`)

`assert_cmd` 또는 표준 라이브러리의 `std::process::Command`를 사용하여 CLI 동작을 검증하는 테스트를 작성하세요.

- **테스트 케이스 1:** `agb --help` 실행 시 도움말이 정상적으로 출력되는지 확인.
- **테스트 케이스 2:** `agb build` 실행 시 기본 경로(`tests/fixtures/toolkit.yaml`)를 사용하여 로드 성공 메시지가 출력되는지 확인.
- **테스트 케이스 3:** 존재하지 않는 경로를 `--config`로 지정했을 때 에러가 발생하는지 확인.

## 4. Constraints (제약 사항 및 금지 행동)

- 실제 리소스를 스캔하거나 파일을 생성하는 로직은 이번 단계에서 구현하지 않습니다. (오직 Config 로드 확인까지)
- CLI 출력 메시지는 간결하고 명확하게 유지하세요.
- `clap` v4의 최신 관례(derive approach)를 준수하세요.

## 5. Acceptance Criteria (검증 체크리스트)

1. **도움말 출력 테스트:** `cargo test --test cli_test` 실행 시 도움말 관련 테스트가 통과하는가?
2. **기본 빌드 명령 테스트:** `tests/fixtures/toolkit.yaml`이 존재하는 환경에서 `agb build` 호출 시 성공 결과가 반환되는가?
3. **설정 파일 누락 테스트:** 설정 파일이 없는 경로를 지정했을 때 프로세스가 0이 아닌 종료 코드를 반환하는가?
4. **타입 안정성:** `cargo check` 실행 시 타입 에러나 미사용 변수 경고가 없는가?
