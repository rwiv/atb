# Plan: Codex 트랜스포머 경로 정책 수정

## Phase 1: Codex 트랜스포머 경로 수정

### Task 1.1: 커맨드·스킬 출력 경로 변경 및 충돌 검증

- [x] `DIR_PROMPTS` 상수 삭제 및 `DIR_AGENTS_SKILLS` 상수 추가 (`src/core/constants.rs`)
- [x] `transform()` Command 분기 경로 수정 (`src/transformer/codex.rs`)
- [x] `transform()` Skill 분기 경로 수정: `DefaultTransformer` 위임 후 path를 `../.agents/skills/[name]/SKILL.md`로 교체 (`src/transformer/codex.rs`)
- [x] `get_target_path()` Command 분기 경로 수정 (`src/transformer/codex.rs`)
- [x] `get_target_path()` Skill 분기 경로 수정 (`src/transformer/codex.rs`)
- [x] `clean_all()`에서 `DIR_PROMPTS` 제거 (`src/builder/emitter.rs`)
- [x] `test_codex_command_transformation` 경로 assertion 수정 (`src/transformer/codex.rs`)
- [x] `test_codex_skill_transformation` 경로 assertion 추가 (`src/transformer/codex.rs`)
- [x] `src/transformer/README.md` — Codex Command·Skill 출력 경로 설명 갱신
- [x] `specs/spec.md` Codex 변환 규격 테이블 갱신

### Task 1.2: config.toml 경로 버그 수정

- [x] `post_transform()` 경로 수정: `PathBuf::from(DIR_CODEX).join(...)` → `PathBuf::from(...)` (`src/transformer/codex.rs`)
- [x] `detransform()` Agent 분기 `config_path` 수정: `DIR_CODEX` 단계 제거 (`src/transformer/codex.rs`)
- [x] `clean_all()`에서 `DIR_CODEX` 제거 (`src/builder/emitter.rs`)
- [x] `test_codex_post_transform` 경로 assertion 수정 (`src/transformer/codex.rs`)
- [x] `src/transformer/README.md` — `.codex/config.toml` 경로 설명 갱신

## Phase 2: Emitter 타겟 인식 및 clean 로직 수정

### Task 2.1: Emitter 수정 및 충돌 검증 추가

- [x] `Emitter` 구조체에 `target: BuildTarget` 필드 추가 (`src/builder/emitter.rs`)
- [x] `Emitter::new()` 시그니처 변경: `target` 인수 추가 (`src/builder/emitter.rs`)
- [x] `clean_all()`에서 Codex 타겟이면 `output_path.join(DIR_AGENTS_SKILLS)` 삭제 추가 (`src/builder/emitter.rs`)
- [x] `clean()` 분기 조건에 `|| first_file.path.ends_with(SKILL_MD)` 추가 (`src/builder/emitter.rs`)
- [x] `Builder::run()` 시그니처에 `target: BuildTarget` 추가 및 `Emitter::new()` 호출부 수정 (`src/builder/mod.rs`)
- [x] `Builder::run()` 초반에 Codex 타겟 한정 Command/Skill 이름 충돌 검증 로직 추가 (`src/builder/mod.rs`)
- [x] `App::build()`에서 `ctx.config.target`을 `builder.run()`에 전달 (`src/app/mod.rs`)
- [x] `tests/e2e_codex_sync_test.rs` — 신규 경로로 assertion 갱신
- [x] 기존 테스트에 타겟 인수 추가 및 신규 테스트 작성 (`src/builder/emitter.rs`)
- [x] 충돌 검증 단위 테스트 추가 (`src/builder/mod.rs`)
- [x] `src/builder/README.md` — `Emitter` 동작 설명 갱신, 충돌 검증 동작 설명 추가

## Phase 3: AppContext output-dir 검증

### Task 3.1: output-dir 검증 추가

- [x] `BuildTarget::expected_output_dir()` 메서드 추가 (`src/core/target.rs`)
- [x] `AppContext::init()`에서 `canonicalize()` 기반 output-dir 이름 검증 로직 추가 (`src/app/context.rs`)
- [x] `test_expected_output_dir` 단위 테스트 추가 (`src/core/target.rs`)
- [x] `AppContext::init()` 검증 로직 통합 테스트 추가 — 올바른/잘못된 output-dir, 상대경로 케이스 (`src/app/context.rs`)
- [x] `tests/e2e_build_test.rs` — 모든 타겟 테스트를 dot-dir 내 `toolkit.yaml` 구조로 재편
- [x] `tests/e2e_codex_sync_test.rs` — `.codex/toolkit.yaml` 구조로 재편
