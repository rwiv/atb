# Task 5.2: OpenCode용 변환기 구현 및 최종 검증

## 1. Objective (목표)

- OpenCode 에이전트를 위한 변환기를 구현하여 모든 지원 타겟 라인업을 완성합니다.
- 실제 샘플 플러그인들을 활용하여 전체 빌드 프로세스를 엔드투엔드(E2E)로 테스트하고 안정성을 최종 검증합니다.

## 2. Context & Files (작업 범위)

- **읽기 전용 (참고용):**
  - `tests/fixtures/` (테스트용 샘플 데이터 확인)
- **생성 및 수정할 파일:**
  - `src/transformers/opencode.rs` (신규 생성: OpenCode용 변환 로직)
  - `tests/e2e_build_test.rs` (신규 생성: 통합 빌드 테스트 코드)

## 3. Instructions (세부 지침)

### Step 1: `OpenCodeTransformer` 구현

OpenCode는 Claude-code와 유사한 마크다운 기반 형식을 사용하지만, 특정 메타데이터 필드 처리가 다를 수 있습니다.

- `src/transformers/opencode.rs`를 생성하고 `Transformer` 트레이트를 구현하세요.
- `AGENTS.md`는 `AGENTS.md`로 변환되도록 설정합니다.

### Step 2: 엔드투엔드(E2E) 테스트 작성

실제 파일 시스템을 사용하는 통합 테스트를 작성하세요.

- `tests/fixtures/toolkit.yaml`과 샘플 플러그인들을 사용하여 `agb build`를 시뮬레이션합니다.
- 각 타겟(`gemini-cli`, `claude-code`, `opencode`)에 대해 빌드를 실행하고, 출력 디렉터리에 기대하는 파일들이 생성되었는지 검증합니다.
- 파일의 내용(TOML 필드, 마크다운 구조 등)이 변환 규칙에 맞는지 확인합니다.

### Step 3: 최종 코드 정리 및 문서화

- 사용되지 않는 코드(Dead code)를 제거하고 전체 프로젝트의 `cargo fmt` 및 `cargo clippy`를 실행합니다.
- `README.md`에 새로운 에이전트 타겟 사용법을 업데이트합니다.

## 4. Constraints (제약 사항 및 금지 행동)

- E2E 테스트 실행 시 실제 사용자의 설정 파일을 덮어쓰지 않도록 임시 디렉터리(`tempfile` 크레이트 권장)를 사용하세요.
- 모든 지원 타겟에 대해 최소 하나 이상의 성공 케이스를 테스트해야 합니다.

## 5. Acceptance Criteria (검증 체크리스트)

1. `target: opencode` 빌드가 에러 없이 성공하고 결과물이 생성되는가?
2. `tests/e2e_build_test.rs`의 모든 테스트 케이스가 통과하는가?
3. 모든 리소스(Commands, Agents, Skills)가 에이전트별로 올바른 포맷으로 변환되는가?
4. `cargo clippy` 실행 시 경고가 없는 상태인가?
