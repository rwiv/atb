# Builder 모듈

## 개요
`builder` 모듈은 `atb` 프로젝트의 리소스 빌드 유틸리티를 제공합니다. `app` 모듈에 의해 호출되어 변환된 리소스를 실제 파일로 배포(Emission)하는 역할을 담당합니다.

## 주요 구성 요소

### 1. Builder (`mod.rs`)
빌드 인스턴스를 관리하며, `app::App`에서 빌드 로직을 수행할 때 보조적인 역할을 수행합니다.

### 2. DependencyChecker (`dependency.rs`)
리소스 간의 정적 의존성을 검증합니다.
- **검증 시점**: 실제 변환(`Transform`) 작업이 수행되기 직전에 실행됩니다.
- **동작**: 플러그인별 `deps.yaml` 파일을 읽어 의존하는 리소스가 현재 빌드 대상(`Registry`)에 포함되어 있는지 확인합니다. 누락된 의존성이 발견되면 상세 에러와 함께 빌드를 중단합니다.

### 3. Emitter (`emitter.rs`)
변환된 최종 결과물을 물리적 파일로 출력합니다.
- **Clean (삭제 범위)**: 빌드 시작 전, `atb.yaml`이 위치한 타겟 루트 디렉터리 내의 `commands/`, `agents/`, `skills/` 디렉터리와 타겟 메인 파일(`GEMINI.md` 등)을 삭제합니다. 타겟 루트 내의 다른 사용자 파일(예: `.git`, `atb.yaml`)은 삭제되지 않습니다.
- **Emit**: `core::TransformedResource` 목록을 바탕으로, 텍스트 변환된 파일(`files`)들은 디스크에 기록하고 단순 포함 파일(`extras`)들은 물리적으로 대상 디렉터리에 복사합니다.

## 빌드 프로세스에서의 역할
`app::App::build`에서 호출되는 `Builder::run`은 다음과 같은 순서로 빌드를 조율합니다:

1.  **의존성 검사**: `DependencyChecker`를 통해 리소스 간 의존성 무결성을 확인합니다.
2.  **변환**: `transformer`를 이용해 각 리소스를 타겟 포맷으로 변환하여 `TransformedResource` 목록을 생성합니다.
3.  **배포**: `emitter`를 이용해 변환된 결과물과 추가 파일(`extras`)을 최종 파일 시스템에 기록합니다.

## 사용 예시

```rust
use crate::builder::emitter::Emitter;

let emitter = Emitter::new(output_dir);
emitter.clean()?;
emitter.emit(&transformed_resources)?;
```
