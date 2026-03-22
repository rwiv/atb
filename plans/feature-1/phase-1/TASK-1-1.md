# Task 1.1: 프로젝트 초기화 및 의존성 설정

## 1. Objective (목표)
- `agb` 프로젝트의 Rust 개발 환경을 구축하고 필요한 외부 라이브러리를 설정합니다.
- 안정적인 빌드 환경을 확인합니다.

## 2. Context & Files (작업 범위)
- **읽기 전용 (참고용):**
    - `specs/SPEC.md` (기술 스택 및 라이브러리 목록 확인)
- **수정할 파일:**
    - `Cargo.toml`
    - `src/main.rs` (기본 뼈대 확인)

## 3. Instructions (세부 지침)

### Step 1: `Cargo.toml` 의존성 추가
`SPEC.md`에서 정의한 라이브러리들을 `[dependencies]` 섹션에 추가하세요. 각 라이브러리의 최신 안정 버전을 사용하거나 적절한 버전을 지정하세요.

필요한 라이브러리 목록:
- `clap`: CLI 명령어 및 인자 파싱 (v4, `derive` 기능 포함)
- `serde`: 직렬화/역직렬화 프레임워크
- `serde_yaml`: `toolkit.yaml` 파싱용
- `serde_json`: 리소스 메타데이터(`*.json`) 파싱용
- `toml`: Gemini-cli용 결과물 생성용
- `anyhow`: 애플리케이션 레벨 에러 핸들링
- `thiserror`: 커스텀 에러 정의용
- `walkdir`: 디렉터리 재귀 탐색용
- `glob`: 파일 패턴 매칭용

### Step 2: 빌드 및 환경 확인
의존성을 추가한 후 프로젝트를 빌드하여 모든 라이브러리가 정상적으로 다운로드되고 컴파일되는지 확인하세요.

```bash
cargo build
```

### Step 3: `src/main.rs` 기본 구조 작성
`clap`을 사용하여 아주 기본적인 CLI 구조만 잡습니다. 아직 기능을 구현하지는 마세요.

## 4. Constraints (제약 사항 및 금지 행동)
- 아직 `src/` 내에 복잡한 로직(파싱, 로딩 등)을 구현하지 마세요. 오직 환경 설정과 기본 CLI 구조에만 집중하세요.
- 불필요한 라이브러리를 추가하지 마세요.

## 5. Acceptance Criteria (검증 체크리스트)
1. `cargo build` 명령어가 에러 없이 성공하는가?
2. `Cargo.toml`에 명시된 모든 라이브러리가 포함되었는가?
3. `agb --help` 명령을 실행했을 때 `clap`에 의해 생성된 도움말 메시지가 출력되는가?
