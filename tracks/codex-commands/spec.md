# Specification: Codex Transformer 경로 정책 수정

## Overview

Codex CLI의 정책 변경에 맞춰 `atb`의 Codex Transformer가 생성하는 파일 경로 및 구조를 수정합니다. 완료 후에는 Codex CLI의 신규 멀티에이전트 규격에 완전히 부합하는 빌드 결과물이 생성됩니다.

- **커맨드 출력 경로**: `prompts/[name].md` → `../.agents/skills/[name]/SKILL.md`
- **스킬 출력 경로**: `skills/[name]/SKILL.md` → `../.agents/skills/[name]/SKILL.md` (Command와 동일 네임스페이스)
- **config.toml 경로 버그**: `.codex/.codex/config.toml` (잘못됨) → `.codex/config.toml` (올바름)
- **output-dir 검증**: 타겟 에이전트에 맞는 디렉터리에서 실행되는지 사전 검증
- **Emitter clean 로직**: `SKILL.md`로 끝나는 경로를 디렉터리 단위로 삭제
- **Emitter 타겟 인식**: Codex 타겟 전용 `../.agents/skills/` 디렉터리 clean 처리
- **Command/Skill 이름 충돌 검증**: 동일 이름의 Command와 Skill이 공존하면 빌드 시작 전 에러

## Requirements

### 1. 커맨드 출력 경로 변경

Codex CLI의 신규 정책에 따라 커맨드를 SKILL 포맷으로 저장합니다.

- 커맨드 출력 경로를 `prompts/[name].md`에서 `../.agents/skills/[name]/SKILL.md`로 변경한다.
- 새 경로는 output-dir(`.codex/`) 기준 상대경로 `../.agents/skills/[name]/SKILL.md`이다.
- `DIR_PROMPTS` 상수를 삭제하고 `DIR_AGENTS_SKILLS = "../.agents/skills"` 상수를 추가한다.
- `specs/spec.md`의 Codex 변환 규격 테이블을 신규 경로로 갱신한다.

### 2. config.toml 경로 버그 수정

`post_transform()`이 잘못된 이중 경로(`.codex/.codex/config.toml`)를 생성하는 버그를 수정합니다.

- `post_transform()`: `PathBuf::from(DIR_CODEX).join(CODEX_CONFIG_FILE_NAME)` → `PathBuf::from(CODEX_CONFIG_FILE_NAME)`
- `detransform()` Agent 분기: `output_dir.join(DIR_CODEX).join(CODEX_CONFIG_FILE_NAME)` → `output_dir.join(CODEX_CONFIG_FILE_NAME)`

### 3. output-dir 검증 추가

`toolkit.yaml`이 잘못된 위치에 있을 때 빌드가 엉뚱한 경로에 파일을 생성하는 것을 방지합니다.

- `BuildTarget`에 `expected_output_dir() -> &'static str` 메서드를 추가한다.
- `AppContext::init()`에서 output-dir 이름과 타겟의 기대값을 비교하여 불일치 시 `anyhow::bail!`로 즉시 종료한다.

| 타겟 | 기대 디렉터리명 |
|---|---|
| `codex` | `.codex` |
| `claude-code` | `.claude` |
| `gemini-cli` | `.gemini` |
| `opencode` | `.opencode` |

### 4. Codex Skill 출력 경로 변경

`CodexTransformer`가 `Resource::Skill`을 `DefaultTransformer`에 위임하면 `skills/[name]/SKILL.md`(output-dir 기준, 즉 `.codex/skills/`)로 저장된다. Codex CLI 정책상 Skill도 Command와 동일하게 `.agents/skills/`에 저장해야 하므로, Skill 분기에서도 `../.agents/skills/[name]/SKILL.md` 경로를 직접 지정한다.

- `transform()` Skill 분기: `DefaultTransformer` 위임 후 `path`를 `../.agents/skills/[name]/SKILL.md`로 교체한다.
- `get_target_path()` Skill 분기: `DIR_AGENTS_SKILLS` 기반 경로를 반환한다.

### 4-1. Codex Skill extras(부속 파일) 출력 경로 수정

`ResourceParser`는 스킬 로딩 시 extras(부속 파일)의 `target` 경로를 `skills/[name]/[relative_path]`로 고정한다. Codex 타겟에서는 SKILL.md가 `../.agents/skills/[name]/SKILL.md`로 출력되지만 extras는 여전히 `output-dir/skills/[name]/...`에 출력되어 SKILL.md와 다른 디렉터리에 분리된다.

- `ResourceParser::parse_resource()` Skill 분기에서 `self.target == BuildTarget::Codex`일 때 extras의 target base를 `DIR_SKILLS` 대신 `DIR_AGENTS_SKILLS`로 사용한다.
- 결과: Codex 타겟에서 extras가 `../.agents/skills/[name]/[relative_path]`로 출력되어 SKILL.md와 동일한 디렉터리 하에 위치한다.

### 5. Command/Skill 이름 충돌 검증

Codex 타겟에서 Command와 Skill이 동일한 `../.agents/skills/` 네임스페이스를 공유하므로, 같은 이름이 존재하면 동일 경로에 파일을 덮어씌우는 문제가 발생한다. `Builder::run()` 초반에서 registry를 통해 이름 교집합을 확인하고, 충돌이 있으면 `anyhow::bail!`로 즉시 종료한다.

- Codex 타겟에 한정하여 command 이름 집합과 skill 이름 집합의 교집합을 검사한다.
- 에러 메시지에 충돌하는 이름을 명시한다: `"Codex 타겟에서 command와 skill의 이름이 충돌합니다: 'foo', 'bar'"`

### 6. Emitter::clean() 경로 분기 수정

Codex 커맨드의 새 경로 `../.agents/skills/[name]/SKILL.md`는 `starts_with(DIR_SKILLS)` 조건을 통과하지 못해 파일 단위로 삭제되는 문제를 수정합니다.

- `clean()` 분기 조건에 `|| first_file.path.ends_with(SKILL_MD)` 조건을 추가하여, `SKILL.md`로 끝나는 경로는 항상 부모 디렉터리 단위로 삭제한다.

### 7. Emitter 타겟 정보 추가

`clean_all()` 시 Codex 타겟이면 `../.agents/skills/` 디렉터리도 삭제해야 합니다.

- `Emitter` 구조체에 `target: BuildTarget` 필드를 추가한다.
- `Emitter::new()` 시그니처를 `new(output_path, target)`으로 변경한다.
- `clean_all()`에서 Codex 타겟이면 `output_path.join(DIR_AGENTS_SKILLS)` 디렉터리를 추가 삭제한다.
- `clean_all()`에서 의미 없는 `DIR_CODEX`(output-dir 자신) 삭제를 제거한다.
- `clean_all()`에서 더 이상 사용되지 않는 `DIR_PROMPTS` 삭제를 제거한다.

## Directory Structure

### 변경 전

```text
프로젝트 루트/
├── .codex/
│   ├── toolkit.yaml
│   ├── .codex/             ← config.toml 버그로 인한 잘못된 이중 경로
│   │   └── config.toml
│   ├── agents/
│   │   └── [name].toml
│   └── prompts/            ← 구 커맨드 출력 위치
│       └── [name].md
```

### 변경 후

```text
프로젝트 루트/
├── .codex/
│   ├── toolkit.yaml
│   ├── config.toml         ← 올바른 경로로 수정
│   └── agents/
│       └── [name].toml
└── .agents/
    └── skills/             ← 신 커맨드·스킬 통합 출력 위치
        ├── [command-name]/
        │   └── SKILL.md
        └── [skill-name]/
            └── SKILL.md
```

## Testing Strategy

- `src/transformer/codex.rs` 기존 단위 테스트의 경로 assertion 수정
  - `prompts/test-cmd.md` → `../.agents/skills/test-cmd/SKILL.md`
  - `.codex/config.toml` → `config.toml`
- `src/transformer/codex.rs` Skill 변환 경로 테스트 추가
  - `skills/test-skill/SKILL.md` → `../.agents/skills/test-skill/SKILL.md`
- `src/core/target.rs` `expected_output_dir()` 단위 테스트 추가
- `src/builder/emitter.rs` 신규 케이스 추가
  - `clean()`: `SKILL.md`로 끝나는 외부 경로에 대해 디렉터리 단위 삭제 검증
  - `clean_all()`: Codex 타겟이면 `../.agents/skills/` 디렉터리 삭제 검증
- `src/builder/mod.rs` 충돌 검증 단위 테스트 추가
  - 동일 이름의 command와 skill이 있을 때 에러 반환 검증
  - 이름 충돌이 없을 때 정상 진행 검증

## Acceptance Criteria

- [ ] Codex 빌드 시 커맨드가 `../.agents/skills/[name]/SKILL.md`로 출력된다.
- [ ] Codex 빌드 시 스킬이 `../.agents/skills/[name]/SKILL.md`로 출력된다 (`.codex/skills/` 아님).
- [ ] Codex 빌드 시 스킬의 부속 파일(extras)이 `../.agents/skills/[name]/[relative_path]`로 출력된다 (SKILL.md와 동일 디렉터리).
- [ ] Codex 빌드 시 동일 이름의 command와 skill이 존재하면 명확한 충돌 오류와 함께 종료된다.
- [ ] Codex 빌드 시 `config.toml`이 `.codex/config.toml`에 생성된다 (`.codex/.codex/config.toml` 아님).
- [ ] 잘못된 output-dir에서 `atb build` 실행 시 명확한 오류 메시지와 함께 종료된다.
- [ ] `cargo test`가 모두 통과한다.
- [ ] `cargo clippy -- -D warnings`가 오류 없이 통과한다.
