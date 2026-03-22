# Task 4.2: 전체 빌드 워크플로우 통합 (Orchestration)

## 1. Objective (목표)

- `Loader`, `Registry`, `Transformer`, `Emitter`를 하나의 유기적인 파이프라인으로 결합합니다.
- `agb build` 명령어 실행 시 설정 로드부터 최종 파일 출력까지의 전 과정이 단일 워크플로우로 작동하도록 구현합니다.

## 2. Context & Files (작업 범위)

- **읽기 전용 (참고용):**
  - `specs/SPEC.md` (시스템 아키텍처 및 데이터 흐름 확인)
- **수정할 파일:**
  - `src/main.rs` (빌드 파이프라인 오케스트레이션)
  - `src/core/mod.rs` (필요 시 통합 로직 추상화)

## 3. Instructions (세부 지침)

### Step 1: 빌드 오케스트레이터(Orchestrator) 구현

각 모듈을 순차적으로 호출하는 통합 로직을 `main.rs` 또는 전용 모듈에 작성하세요.

1. **Config Load:** `toolkit.yaml`을 읽어 타겟 에이전트와 리소스 목록을 확보합니다.
2. **Resource Load:** `Loader`를 통해 플러그인에서 대상 리소스(Markdown, JSON)를 메모리로 로드합니다.
3. **Registration:** `Registry`에 리소스를 등록하고 이름 충돌을 검증합니다.
4. **Transformation:** 선택된 타겟(예: Gemini)에 맞는 `Transformer`를 생성하여 리소스를 변환합니다.
5. **Emission:** `Emitter`를 통해 기존 결과물을 정리하고 변환된 파일들을 출력합니다.

### Step 2: CLI 인자 연동 및 사용자 피드백

- `agb build` 명령어 실행 시 현재 경로를 기준으로 빌드를 수행하도록 합니다.
- 각 단계의 진행 상황을 사용자에게 알립니다 (예: `[1/5] Loading config...`).
- 빌드 완료 후 성공 메시지와 생성된 리소스의 개수를 요약하여 출력합니다.

### Step 3: 통합 에러 처리

- 파이프라인 중간에 에러가 발생할 경우, 해당 단계를 명시하고 빌드를 중단합니다.
- 예: "Build failed during transformation: Resource 'foo' missing metadata."

## 4. Constraints (제약 사항 및 금지 행동)

- 각 모듈 간의 결합도를 낮추기 위해 모듈 내부 로직을 직접 수정하기보다는 정의된 인터페이스(Trait 및 메서드)를 호출하는 방식으로 작성하세요.
- 모든 에러는 `anyhow`를 사용하여 전파하되, 사용자에게 노출될 때는 이해하기 쉬운 메시지로 가공하세요.

## 5. Acceptance Criteria (검증 체크리스트)

1. `agb build` 명령어 하나로 설정 로드부터 파일 생성까지 전 과정이 자동으로 수행되는가?
2. 리소스 이름 충돌이나 설정 오류 시 빌드가 안전하게 중단되고 적절한 에러 메시지가 출력되는가?
3. 빌드 결과물이 `SPEC.md`에 정의된 폴더 구조와 파일 포맷을 정확히 따르는가?
4. `cargo run -- build` 실행 시 에러 없이 모든 샘플 리소스가 빌드되는가?
