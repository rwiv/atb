# Plan: Main Features

이 문서는 `agb` 프로젝트의 전체 개발 여정을 관리합니다. 각 테스크는 독립적인 `plans/feature-1/phase-X/TASK-X-Y.md` 문서에 상세 가이드를 가집니다.

## Phase 1: 기반 설정 및 CLI 환경 구축 (CLI & Config)
*   **Task 1.1: 프로젝트 초기화 및 의존성 설정**
    *   *성공 기준:* `Cargo.toml`에 필요한 라이브러리(`clap`, `serde`, `anyhow` 등)가 설정되고 빌드가 성공한다.
*   **Task 1.2: `toolkit.yaml` 파싱 및 설정 모델링**
    *   *성공 기준:* `tests/fixtures/toolkit.yaml` 파일을 읽어 Rust 데이터 구조체로 정확히 역직렬화한다.
*   **Task 1.3: CLI 명령 체계 구축**
    *   *성공 기준:* `agb build` 명령어를 실행했을 때 설정 파일을 로드하는 기본 흐름이 작동한다.

## Phase 2: 코어 리소스 로더 (Core Loader)
*   **Task 2.1: 플러그인 디렉터리 스캔 및 필터링**
    *   *성공 기준:* `exclude` 패턴을 적용하여 `tests/fixtures/plugins/` 내의 파일들을 재귀적으로 탐색하고 대상 목록을 추출한다.
*   **Task 2.2: 리소스(Markdown/JSON) 로딩 및 유효성 검사**
    *   *성공 기준:* Markdown 내용과 JSON 메타데이터를 하나의 `Resource` 객체로 병합하여 메모리에 로드한다.
*   **Task 2.3: 리소스 레지스트리 구축 및 이름 충돌 관리**
    *   *성공 기준:* 모든 플러그인 리소스를 수집하고, 이름이 중복된 리소스가 있을 경우 명확한 에러를 발생시킨다.

## Phase 3: 변환 인터페이스 및 Gemini 지원 (Transformer - Gemini)
*   **Task 3.1: `Transformer` 트레이트 정의**
    *   *성공 기준:* 에이전트별 변환 로직을 추상화하는 공통 인터페이스(Trait) 설계를 완료한다.
*   **Task 3.2: Gemini-cli용 TOML 변환 로직 구현**
    *   *성공 기준:* Markdown과 JSON 데이터를 Gemini-cli 규격에 맞는 TOML 포맷으로 변환한다.
*   **Task 3.3: 전역 시스템 프롬프트(`AGENTS.md`) 변환**
    *   *성공 기준:* 루트의 `AGENTS.md`를 타겟 에이전트의 메인 메모리 파일(예: `GEMINI.md`)로 변환한다.

## Phase 4: 파일 출력 및 통합 빌드 파이프라인 (Emitter & Integration)
*   **Task 4.1: 빌드 디렉터리 정리 및 파일 쓰기 (Emitter)**
    *   *성공 기준:* 대상 디렉터리의 기존 결과물을 삭제하고, 변환된 파일들을 정해진 구조에 맞춰 물리적으로 생성한다.
*   **Task 4.2: 전체 빌드 워크플로우 통합 (Orchestration)**
    *   *성공 기준:* `agb build` 명령 실행 시 로드-검증-변환-출력의 전 과정이 단일 파이프라인으로 작동한다.

## Phase 5: 멀티 에이전트 확장 (Multi-target Support)
*   **Task 5.1: Claude-code용 Markdown 변환기 구현**
    *   *성공 기준:* `target: claude-code` 설정 시 Claude 규격에 최적화된 마크다운 결과물을 생성한다.
*   **Task 5.2: OpenCode용 변환기 구현 및 최종 검증**
    *   *성공 기준:* 모든 지원 타겟에 대해 샘플 플러그인을 활용한 엔드투엔드(E2E) 빌드 테스트를 통과한다.
