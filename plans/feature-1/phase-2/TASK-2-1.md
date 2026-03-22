# Task 2.1: 플러그인 디렉터리 스캔 및 필터링

## 1. Objective (목표)

- `tests/fixtures/plugins/` 디렉터리를 재귀적으로 탐색하여 빌드 대상이 되는 파일 리스트를 추출합니다.
- `toolkit.yaml`에 정의된 `exclude` 패턴을 적용하여 불필요한 파일이 로딩 대상에서 제외되도록 처리하는 기초 로직을 구현합니다.

## 2. Context & Files (작업 범위)

- **읽기 전용 (참고용):**
  - `specs/SPEC.md` (디렉터리 구조 및 exclude 규칙 확인)
  - `specs/SPEC.md` (`walkdir`, `glob` 라이브러리 활용 가이드)
  - `src/config.rs` (`Config` 구조체의 `exclude` 필드 정의)
- **생성 및 수정할 파일:**
  - `src/core/loader.rs` (신규 생성: 디렉터리 스캔 및 파일 필터링 로직)
  - `src/core/mod.rs` (신규 생성: core 모듈 정의 및 노출)
  - `src/main.rs` (수정: core 모듈 연결)

## 3. Instructions (세부 지침)

### Step 1: `core` 모듈 초기화

- `src/core/mod.rs`를 생성하고 `loader` 모듈을 선언하세요.
- `src/main.rs` 상단에 `mod core;`를 추가하여 프로젝트 전체 모듈 시스템에 연결하세요.

### Step 2: 디렉터리 스캔 기능 구현 (`src/core/loader.rs`)

- `walkdir` 크레이트를 사용하여 `tests/fixtures/plugins/` 디렉터리를 재귀적으로 순회하는 기능을 구현하세요.
- `commands`, `agents`, `skills`와 같은 하위 디렉터리 구조를 올바르게 인식해야 합니다.
- 숨김 파일(예: `.DS_Store`)이나 디렉터리 자체는 파일 목록에서 제외합니다.

### Step 3: `exclude` 패턴 필터링 적용

- `toolkit.yaml`에서 로드된 `Config`의 `exclude` 리스트를 인자로 받습니다.
- `glob` 크레이트를 활용하여 각 파일 경로가 `exclude` 패턴 중 하나라도 매칭되는지 검사합니다.
- 필터링을 통과한 "유효한 파일 경로"의 `Vec<PathBuf>` 목록을 반환하는 함수를 작성하세요.

## 4. Constraints (제약 사항 및 금지 행동)

- 실제 파일 내용을 읽거나 `Resource` 객체로 변환하는 로직은 이번 단계에서 구현하지 않습니다 (Task 2.2에서 수행).
- 존재하지 않는 `tests/fixtures/plugins/` 디렉터리에 접근 시 적절한 에러를 반환해야 합니다.
- `anyhow::Result`를 사용하여 에러 핸들링을 일원화하세요.

## 5. Acceptance Criteria (검증 체크리스트)

1. `tests/fixtures/plugins/` 내의 모든 마크다운(`.md`) 및 JSON(`.json`) 파일이 재귀적으로 탐색되는가?
2. `toolkit.yaml`의 `exclude` 패턴에 명시된 파일(예: `*.tmp`)이 결과 목록에서 정확히 제외되는가?
3. 유효하지 않은 경로나 권한 문제가 있는 디렉터리에 대해 명확한 에러 메시지를 출력하는가?
4. `cargo check` 실행 시 타입 오류나 미사용 변수 경고가 없는가?
