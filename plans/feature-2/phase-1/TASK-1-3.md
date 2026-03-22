# Task 6.3: 경로 분리 아키텍처 통합 및 최종 검증

## 1. Objective (목표)

- 소스 디렉터리와 출력 디렉터리가 물리적으로 분리된 환경에서 전체 빌드 프로세스가 완벽하게 작동하는지 검증합니다.
- 실제 사용 시나리오를 반영한 통합 테스트 케이스를 추가하여 회귀 방지를 보장합니다.

## 2. Context & Files (작업 범위)

- **생성 및 수정할 파일:**
  - `tests/e2e_path_split_test.rs` (신규: 경로 분리 통합 테스트)
  - `tests/fixtures/split_env/` (테스트용 가상 환경 구성)

## 3. Instructions (세부 지침)

### Step 1: 통합 테스트용 Fixture 구성

`tests/fixtures/split_env/` 아래에 다음과 같이 구조를 설계합니다 (테스트 코드 내에서 임시 디렉터리로 생성 권장).

- `source_repo/`: `plugins/`, `AGENTS.md` 포함
- `agent_workspace/`: `toolkit.yaml` 포함 (`source` 필드는 `source_repo/`의 절대 경로를 가리킴)

### Step 2: 통합 테스트 코드 작성 (`tests/e2e_path_split_test.rs`)

`agb build --config [workspace]/toolkit.yaml` 명령을 시뮬레이션하는 테스트를 작성합니다.

- **준비**: 임시 디렉터리 두 곳을 생성하고 소스 리소스와 설정 파일을 배치합니다.
- **실행**: `agb` 바이너리를 실행하거나 내부 함수를 호출하여 빌드를 수행합니다.
- **검증**:
  - `agent_workspace/` 내부에 `commands/`, `agents/` 폴더와 결과물 파일들이 생성되었는지 확인합니다.
  - 생성된 파일의 내용이 `source_repo/`의 마스터 데이터와 일치하는지 확인합니다.
  - `source_repo/` 내부에는 어떠한 출력 파일도 생성되지 않았는지(깨끗한 상태 유지) 확인합니다.

### Step 3: 에러 케이스 검증

- `source` 경로가 잘못된 경우의 에러 처리를 검증합니다.
- `source` 경로는 유효하지만 그 안에 `plugins/` 폴더가 없는 경우의 동작을 확인합니다.

## 4. Constraints (제약 사항 및 금지 행동)

- 테스트 실행 시 실제 파일 시스템을 건드리므로, 반드시 `tempfile` 크레이트 등을 사용하여 테스트 종료 후 환경이 정리되도록 하세요.
- 절대 경로 처리가 OS(Windows/Linux/macOS)에 관계없이 동작하도록 `PathBuf`를 활용하세요.

## 5. Acceptance Criteria (검증 체크리스트)

1. `tests/e2e_path_split_test.rs`의 모든 테스트 케이스가 통과하는가?
2. 소스 디렉터리와 출력 디렉터리가 다를 때 리소스 로딩과 파일 생성이 각각 올바른 위치에서 일어나는가?
3. 빌드 후 소스 디렉터리에 의도하지 않은 파일(빌드 결과물 등)이 생성되지 않았는가?
4. 모든 코드가 기존의 `cargo test` 및 `cargo build`를 통과하는가?
