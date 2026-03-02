# Plan: Codex Multi-Agent Support

이 문서는 Codex의 멀티 에이전트 지원을 위한 `agb` 내부 구조의 개선 및 기능 추가 계획을 관리합니다. 각 테스크는 독립적인 문서(`plans/feature-14/phase-X/TASK-X-Y.md`)로 상세 내용이 기술됩니다.

## Phase 1: Transformer 인터페이스 확장 및 에러 해결 (Foundation & Traits)
*   **Task 1.1: `Transformer` Trait 시그니처 변경 및 파급 오류 해결**
    *   *목표:* `post_transform` 훅을 추가하고, `detransform` 시그니처에 `output_dir`을 추가.
    *   *성공 기준:* `Transformer` 트레이트가 갱신되고 `codex`, `gemini`, `default` 모듈과 `Syncer`가 컴파일 에러 없이 빌드된다.

## Phase 2: Codex 에이전트 변환 로직 마이그레이션 (Codex Agent Transform)
*   **Task 2.1: `developer_instructions` 필드명 마이그레이션**
    *   *목표:* 마크다운 본문을 개별 에이전트 TOML로 변환할 때, `prompt` 대신 `developer_instructions` 키를 사용하도록 수정.
    *   *성공 기준:* 개별 Codex Agent가 올바르게 변환되고 관련 유닛 테스트를 통과한다.

*   **Task 2.2: `post_transform`을 통한 `.codex/config.toml` 생성**
    *   *목표:* 빌드된 리소스들을 종합하여 에이전트 설정 레지스트리인 `.codex/config.toml` 내용을 구성하고 반환하는 로직 구현.
    *   *성공 기준:* `agb build` 시 `.codex/config.toml`에 모든 에이전트의 `description` 및 경로가 정상적으로 작성된다.

## Phase 3: 역변환(Sync) 및 Clean 지원 (Detransform & Emitter)
*   **Task 3.1: Codex Agent 역변환 시 `.codex/config.toml` 파싱 로직 추가**
    *   *목표:* Agent 역변환 시 개별 TOML의 `developer_instructions`뿐 아니라 `.codex/config.toml`을 파싱하여 `description`을 복원.
    *   *성공 기준:* `agb sync` 시 `.codex/config.toml`에서 수정된 description이 원본 `.md` 파일에 성공적으로 패치된다.

*   **Task 3.2: Emitter Clean 로직 업데이트 및 Builder 연동**
    *   *목표:* 빌드 전 `clean` 시 `.codex` 디렉터리가 삭제되도록 하고, Builder 파이프라인에서 `post_transform`이 정상 작동하도록 연동.
    *   *성공 기준:* Emitter가 `.codex` 폴더를 제대로 정리하고 전체 파이프라인에서 결과물을 올바르게 디스크에 기록한다.
