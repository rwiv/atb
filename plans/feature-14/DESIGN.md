# Design: Codex Multi-Agent Support

## 1. 개요 (Overview)
Codex의 멀티 에이전트 워크플로우를 지원하기 위해 현재 `agb`의 Codex 에이전트 변환 및 동기화(Sync) 메커니즘을 개선합니다. 주요 변경 사항은 개별 에이전트 파일의 구조 변경과 중앙 에이전트 레지스트리(`.codex/config.toml`)의 자동 생성, 그리고 이를 통한 메타데이터(`description`) 동기화 처리입니다.

## 2. 주요 변경 사항 (Key Changes)

### 2.1. `developer_instructions` 필드 마이그레이션
- **As-Is**: 기존 `CodexTransformer`는 마크다운 본문을 `prompt` 필드에 매핑.
- **To-Be**: Codex의 새 규격에 맞춰 `prompt` 대신 `developer_instructions` 필드를 사용하도록 변경.

### 2.2. `Transformer` Trait 확장 (Post-transform Hook)
현재의 `transform` 메서드는 개별 리소스를 처리하므로, 전체 에이전트를 모아 하나의 중앙 설정 파일(`.codex/config.toml`)을 생성하는 기능이 부족합니다.
- **해결책**: `Transformer` 트레이트에 모든 리소스 변환이 끝난 후 호출되는 `post_transform` 훅을 추가합니다.
- **Codex 구현**: 빌드된 전체 리소스 중 `Agent` 리소스들을 필터링하여 `.codex/config.toml` 파일(`TransformedFile`)을 생성합니다. 여기에는 각 에이전트의 `description`과 개별 설정 파일(`config_file`) 경로가 기록됩니다.

### 2.3. 역변환(`detransform`) 컨텍스트 주입
Codex의 새 규격에서는 에이전트의 `description`이 개별 TOML 파일이 아닌 `.codex/config.toml`에 존재합니다. `Sync` 과정에서 이 `description`을 복원하여 소스에 패치하려면 역변환 시 추가적인 파일 접근이 필요합니다.
- **해결책**: `detransform` 메서드에 `output_dir: &Path` 파라미터를 추가하여, 필요 시 프로젝트 디렉터리의 다른 파일(`.codex/config.toml` 등)을 읽어 완전한 `ResourceData`를 구성할 수 있도록 컨텍스트를 제공합니다.

### 2.4. 빌드 환경 Clean 로직 업데이트
`.codex/config.toml`과 같은 디렉터리가 새로 생성되므로, Emitter의 `clean` 로직에서 이를 안전하게 정리할 수 있도록 대상 목록을 추가합니다.

## 3. 모듈별 상세 수정 계획

| 모듈 | 상세 변경 내용 |
| :--- | :--- |
| **`src/transformer/mod.rs`** | `Transformer` 트레이트에 `post_transform` 훅 추가 및 `detransform` 시그니처(`output_dir` 추가) 변경. |
| **`src/transformer/codex.rs`** | `prompt` 필드를 `developer_instructions`로 변경. `post_transform` 구현을 통해 `.codex/config.toml` 텍스트 생성. `detransform` 시 `.codex/config.toml`에서 `description` 조회 로직 추가. |
| **`src/transformer/*.rs`** | 트레이트 변경에 따른 `default.rs`, `gemini.rs` 등 타겟별 `detransform` 및 `post_transform` 구현 업데이트. |
| **`src/builder/emitter.rs`** | `.codex` 디렉터리를 초기 Clean 대상에 포함하도록 로직 수정. |
| **`src/syncer/mod.rs`** | `detransform` 호출 시 현재 `output_dir`을 인자로 넘기도록 수정. |
| **`src/builder/mod.rs`** | `transform` 루프가 끝난 뒤 `post_transform`을 호출하여 결과 `TransformedFile` 목록에 추가. |

## 4. 기대 효과 (Benefits)
- **Codex 최신 스펙 호환성 확보**: 멀티 에이전트 구조 및 `developer_instructions`를 완벽히 지원.
- **동기화 무결성 유지**: 중앙 파일로 분리된 메타데이터도 안전하게 복원하여 로컬의 변경사항이 원본 `.md`에 정상 반영됨.
- **Transformer 확장성**: `post_transform` 훅 추가를 통해 향후 다른 타겟 에이전트에서 복합적인 메타 정보(예: 인덱스 파일)를 생성할 때 활용 가능.
