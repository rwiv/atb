# Technical Specification

본 문서는 `atb` 시스템을 구성하는 기술 스택, 데이터 처리 규칙, 타겟별 변환 및 동기화 명세, 그리고 운영 제약 사항을 정의합니다. 파일 구조 및 설정 파일 규격에 대해서는 [format.md](./format.md)를 참조하십시오.

## 1. 기술 스택 (Tech Stack)

`atb`는 다음 기술들을 기반으로 구축되었습니다:

- **언어**: Rust
- **CLI 프레임워크**: `clap`
- **데이터 직렬화**: `serde`, `serde_yaml`, `serde_json` (preserve_order 활성화), `toml`
- **에러 핸들링**: `anyhow`, `thiserror`
- **파일 시스템**: `walkdir`, `glob`, `shellexpand`
- **암호화/해싱**: `sha2` (동기화 무결성 검증용)
- **정규 표현식**: `regex` (MdPatcher 본문 치환용)
- **테스팅**: `assert_cmd`, `predicates`, `tempfile` (E2E 및 단위 테스트)

## 2. 메타데이터 처리 규격

### 2.1 병합 우선순위 (Merge Priority)

빌드 시 여러 출처의 메타데이터를 다음 순서로 병합합니다(Shallow Merge). 상세 알고리즘은 [design.md](./design.md)를 참조하십시오.

1.  **Markdown Frontmatter (Base)**: 원본 마크다운 파일의 기본 설정.
2.  **Metadata Map**: `overrides.yaml`에 정의된 타겟별 필드 값 치환 규칙.
3.  **외부 메타데이터 파일**: 타겟 전역 예약어 섹션의 오버라이트 값.

### 2.2 의존성 검증 규칙

- **Fail-fast**: 하나 이상의 의존 리소스가 빌드 대상에 누락된 경우 즉시 빌드를 중단합니다.
- **비재귀적 검사**: 현재 리소스가 직접 명시한 의존성만 확인합니다.

## 3. 보안 및 제약 사항

- **시스템 보호**: 리소스의 필수 본문(`.md`)은 `exclude` 패턴에 해당하더라도 시스템 안정성을 위해 강제로 스캔 및 빌드 대상에 포함됩니다.
- **예약어 보호**: 플러그인 소스 내에 타겟 결과물 파일명(`GEMINI.md`, `CLAUDE.md`, `AGENTS.md` 등)과 동일한 이름의 파일은 존재할 수 없습니다.
- **이름 충돌**: 동일 리소스 타입 내에서 이름이 중복되는 경우, 플러그인이 다르더라도 빌드가 실패합니다. 타입이 다르면(예: Command 'foo'와 Skill 'foo') 중복이 허용됩니다.

## 4. 타겟별 변환 사양 (Transformation)


| 타겟 | 커맨드 변환 | 에이전트/스킬 변환 | 전역 지침 |
| :--- | :--- | :--- | :--- |
| **Gemini-cli** | `*.toml` (Prompt 필드 포함) | `*.md` (메타데이터 포함) | `GEMINI.md` |
| **Claude-code** | `*.md` (Frontmatter 포함) | `*.md` (Frontmatter 포함) | `CLAUDE.md` |
| **OpenCode** | `*.md` (Frontmatter 포함) | `*.md` (Frontmatter 포함) | `AGENTS.md` |
| **Codex** | `prompts/*.md` (Frontmatter 포함) | `agents/*.toml` (`developer_instructions` 필드 포함) | `AGENTS.md` |


**특이사항**:
- **Codex 멀티 에이전트**:
  - Codex 빌드 시 각 Agent는 `.codex/config.toml` 이라는 전역 에이전트 설정 레지스트리 파일에 자동으로 취합되어 등록됩니다.
  - 개별 `agents/*.toml` 파일에는 에이전트 지시문(`developer_instructions`)만 저장되며, 설명(`description`)은 `.codex/config.toml` 내부에 저장됩니다.
- **Gemini-cli 에이전트**:
  - Agent 빌드 시 원본 메타데이터에 `tools` 필드가 누락되어 있는 경우 `tools: ["*"]`가 자동으로 주입됩니다.

## 5. 동기화 규격 (Sync Specifications)

`atb sync` 실행 시 프로젝트 세션의 변경사항을 다음 규칙에 따라 소스로 업데이트합니다. 상세 구현 방식은 [design.md](./design.md)를 참조하십시오.

- **본문 동기화**: 마크다운 본문(Frontmatter 제외) 전체를 교체합니다.
- **설명 동기화**: `description` 필드를 업데이트합니다. YAML 파서를 사용하여 한 줄 또는 멀티라인(`|`) 설명을 모두 안전하게 처리하며, `serde_json`의 `preserve_order` 피처를 통해 프론트매터 내의 키 순서를 원본과 동일하게 유지합니다.
  - **참고 (Codex)**: Codex Agent의 경우 `description`이 개별 TOML이 아닌 `.codex/config.toml`에 위치하므로, 해당 파일을 파싱하여 원본 소스에 반영합니다.
- **스킬 파일 동기화**: 해시(SHA-256) 비교를 통해 추가 파일(`extras`)을 동기화합니다. `exclude` 대상 및 필수 파일(`SKILL.md`)은 삭제되지 않습니다.
- **고정밀 무결성 보존 (High-Fidelity Preservation)**: 타겟의 변경 사항이 없을 경우, 원본 소스 파일의 마지막 개행 문자(Trailing Newline)를 포함한 모든 바이트를 100% 보존하여 `git diff` 노이즈를 방지합니다.

## 6. 예외 처리 전략

- 설정 파일(`toolkit.yaml`) 미존재 시 가이드 메시지와 함께 실행을 중단합니다.
- 빌드 전 Clean 단계에서 권한 문제 등으로 기존 파일 삭제가 불가능할 경우 안전을 위해 빌드를 중단합니다.
