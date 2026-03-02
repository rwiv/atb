# agb (Agents Builder)

`agb`는 여러 AI 코딩 에이전트(Claude Code, Gemini CLI 등)의 프롬프트와 스킬을 단일 소스에서 관리하고, 각 환경의 규격에 맞춰 최적화하여 배포하는 **AI 에이전트 리소스 오케스트레이터**입니다.

## 주요 기능

- **중앙화된 리소스 관리 (Base)**: 프로젝트마다 파편화된 설정을 방지하고, 하나의 베이스 디렉토리에서 플러그인 단위로 리소스를 통합 관리합니다.
- **양방향 동기화 (Sync)**: 프로젝트 진행 중 로컬에서 개선된 프롬프트나 스킬을 다시 베이스 디렉토리로 안전하게 반영합니다.
- **멀티 타겟 빌드**: 단일 마크다운 소스를 기반으로 타겟 에이전트의 규격(TOML, YAML, JSON 등)에 맞춰 자동으로 변환 및 빌드합니다.

## 지원 에이전트 리스트

- **Claude Code** (`claude-code`)
- **Gemini CLI** (`gemini-cli`)
- **OpenCode** (`opencode`)
- **Codex** (`codex`)

## 시작하기

### 1. 설치

```bash
cargo install --path .
```

### 2. 프로젝트 설정 (`agb.yaml`)

프로젝트 루트에 `agb.yaml`을 생성하여 베이스 디렉토리와 타겟 에이전트를 지정합니다.

```yaml
source: ~/agb-resources      # 리소스 소스 저장소(Base) 경로
target: gemini-cli           # 빌드 타겟 (gemini-cli, claude-code 등)
exclude:
  - "*.tmp"                  # 제외할 패턴 (선택 사항)

resources:
  commands:
    - my_plugin:web_search   # [플러그인]:[리소스명]
  skills:
    - shared_plugin:python_expert
```

### 3. 빌드 (Base → Project)

중앙 저장소의 리소스를 현재 프로젝트의 에이전트 규격에 맞춰 생성합니다.

```bash
agb build
```

### 4. 동기화 (Project → Base)

프로젝트 환경에서 수정된 내용을 원본 소스에 반영합니다.

```bash
agb sync
```

## 시스템 구조

`agb`는 중앙 리소스 저장소(Source)와 실제 개발 환경(Project)을 분리하여 관리합니다.

### 1. 중앙 리소스 저장소 (Base)

프롬프트와 스킬을 관리하는 마스터 라이브러리 구조입니다.

```text
[Base Directory]/
├── AGENTS.md               # 공용 시스템 지침
├── map.yaml                # 타겟별 메타데이터 매핑 규칙 (선택)
└── plugins/                # 플러그인 단위 리소스 모음
    └── my_plugin/
        ├── deps.yaml       # 리소스 간 의존성 정의 (선택)
        ├── commands/       # [name].md (+ 선택적 .yaml)
        ├── agents/         # [name].md (+ 선택적 .yaml)
        └── skills/         # [name]/SKILL.md (+ 추가 파일들)
```

### 2. 프로젝트 개발 환경 (Project)

`agb build`를 통해 생성되는 결과물 구조입니다.

```text
[Project Root]/
├── agb.yaml                # 프로젝트 빌드 및 동기화 설정
├── GEMINI.md               # 변환된 전역 지침 (타겟에 따라 이름 변경)
├── commands/               # 변환된 커맨드들
├── agents/                 # 변환된 에이전트들
└── skills/                 # 변환된 스킬 폴더들
```

## 관련 문서

상세한 요구사항, 규격 및 설계 내용은 다음 문서를 참조하십시오.

- [**prd.md**](./specs/prd.md): 제품 요구사항 및 비즈니스 목표
- [**format.md**](./specs/format.md): 리소스 구조 및 설정 규격 (사용자/개발자 가이드)
- [**design.md**](./specs/design.md): 시스템 아키텍처 및 상세 알고리즘
- [**spec.md**](./specs/spec.md): 기술 규격 및 처리 규칙
- [**model.md**](./specs/model.md): 핵심 데이터 모델 정의 (Rust 코드 스니펫 포함)
