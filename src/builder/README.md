# Builder 모듈

## 개요
`builder` 모듈은 `atb` 프로젝트의 리소스 빌드 유틸리티를 제공합니다. `app` 모듈에 의해 호출되어 변환된 리소스를 실제 파일로 배포(Emission)하는 역할을 담당합니다.

## 주요 구성 요소

### 1. Builder (`mod.rs`)
빌드 인스턴스를 관리하며, `app::App`에서 빌드 로직을 수행할 때 보조적인 역할을 수행합니다.
Codex 타겟에서는 Command와 Skill이 동일한 `../.agents/skills/` 네임스페이스를 공유하므로, 빌드 초기에 동일 이름 충돌을 검증합니다.

### 2. DependencyChecker (`dependency.rs`)
리소스 간의 정적 의존성을 검증합니다.
- **검증 시점**: 실제 변환(`Transform`) 작업이 수행되기 직전에 실행됩니다.
- **동작**: 플러그인별 `requirements.yaml` 파일을 읽어 의존하는 리소스가 현재 빌드 대상(`Registry`)에 포함되어 있는지 확인합니다. 누락된 의존성이 발견되면 상세 에러와 함께 빌드를 중단합니다.

### 3. Emitter (`emitter.rs`)
변환된 최종 결과물을 물리적 파일로 출력합니다.
- **`target` 필드**: 타겟별 출력 구조 차이를 반영하기 위해 `BuildTarget`을 보관합니다.
- **`clean(resources)`**: 빌드 시작 전, 현재 빌드 대상 리소스에 해당하는 파일/디렉터리만 선택적으로 삭제합니다. Command/Agent는 해당 파일만, Skill 또는 `SKILL.md`로 끝나는 리소스는 서브디렉터리 전체를 삭제하며, 전역 파일(`GEMINI.md`, `CLAUDE.md`, `AGENTS.md`)은 항상 삭제됩니다. 빌드 대상이 아닌 기존 파일은 출력 디렉터리에 그대로 유지됩니다.
- **`clean_all()`**: `--clean` 옵션 사용 시 호출됩니다. 기본적으로 `commands/`, `agents/`, `skills/` 디렉터리와 전역 파일(`GEMINI.md`, `CLAUDE.md`, `AGENTS.md`) 전체를 삭제합니다. Codex 타겟에서는 `.codex/skills/`를 보존하고, output-dir 외부의 `../.agents/skills/` 디렉터리를 삭제합니다.
- **`emit(resources)`**: `core::TransformedResource` 목록을 바탕으로, 텍스트 변환된 파일(`files`)들은 디스크에 기록하고 단순 포함 파일(`extras`)들은 물리적으로 대상 디렉터리에 복사합니다.

## 빌드 프로세스에서의 역할
`app::App::build`에서 호출되는 `Builder::run`은 다음과 같은 순서로 빌드를 조율합니다:

1.  **Codex 이름 충돌 검증**: Codex 타겟이면 Command와 Skill의 동일 이름 충돌을 확인합니다.
2.  **의존성 검사**: `DependencyChecker`를 통해 리소스 간 의존성 무결성을 확인합니다.
3.  **변환**: `transformer`를 이용해 각 리소스를 타겟 포맷으로 변환하여 `TransformedResource` 목록을 생성합니다.
4.  **정리 및 배포**: `full_clean` 여부에 따라 `emitter.clean_all()`(전체 삭제) 또는 `emitter.clean(&resources)`(선택적 삭제)를 호출한 후, `emitter.emit`으로 변환된 결과물과 추가 파일(`extras`)을 파일 시스템에 기록합니다.

## 사용 예시

```rust
use crate::builder::emitter::Emitter;
use crate::core::BuildTarget;

let emitter = Emitter::new(output_dir, BuildTarget::ClaudeCode);

// 선택적 삭제 (기본 동작)
emitter.clean(&transformed_resources)?;

// 전체 삭제 (--clean 옵션)
emitter.clean_all()?;

emitter.emit(&transformed_resources)?;
```
