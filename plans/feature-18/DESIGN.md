# 설계: 소스 디렉터리 구조 평탄화 (Flattening)

## 1. 문제 정의
현재 AI 에이전트 리소스의 소스 저장소 구조는 모든 플러그인 리소스를 담기 위해 `plugins/`라는 하위 디렉터리를 필수로 요구합니다. 이는 불필요한 계층을 추가하며, 사용자가 리소스를 직관적으로 관리하는 데 방해가 됩니다.

**현재 구조:**
```text
[Source Repository]/
├── AGENTS.md
├── overrides.yaml
└── plugins/
    └── my_plugin/
        ├── commands/
        ├── agents/
        └── skills/
```

**개선된 구조:**
```text
[Source Repository]/
├── AGENTS.md
├── overrides.yaml
└── my_plugin/
    ├── commands/
    ├── agents/
    └── skills/
```

## 2. 변경 제안

### 2.1 로더 수정 (`src/loader/mod.rs`)
- `ResourceLoader::new`에서 `plugins/` 디렉터리 존재 여부 체크 로직을 제거합니다.
- `self.root`를 `source_root.join("plugins")` 대신 `source_root` 자체로 설정합니다.
- `scan()` 로직이 루트 디렉터리부터 시작하도록 업데이트합니다.

### 2.2 전역 파일 제외 로직 (`src/core/filter.rs` 및 `src/core/constants.rs`)
- 소스 루트에 위치할 때 플러그인으로 취급되지 않아야 하는 "전역 파일"들을 식별합니다.
- 대상 파일: `AGENTS.md`, `overrides.yaml`, `toolkit.yaml`, `.gitignore` 등.
- `FileFilter::is_valid`를 업데이트하여, 루트에 위치한 이러한 전역 파일들에 대해 에러를 발생시키지 않고 조용히 스캔 대상에서 제외(skip)하도록 수정합니다. (현재는 `FORBIDDEN_FILES`에 포함된 파일 발견 시 에러 발생)

### 2.3 경로 해석 (`src/loader/resolver.rs`)
- `ResourcePathResolver::resolve`는 현재 `components[0]`을 플러그인 이름으로 간주합니다.
- 루트를 직접 스캔할 경우, `AGENTS.md`와 같은 파일은 `components`의 길이가 1이 됩니다.
- 기존의 `components.len() < 3` 체크 로직이 루트에 있는 파일들을 자연스럽게 무시하는지 검증하고 필요시 보완합니다.

### 2.4 상수 정리 (`src/core/constants.rs`)
- 더 이상 사용되지 않는 `PLUGINS_DIR_NAME` 상수를 제거하거나 사용처를 정리합니다.

### 2.5 테스트 업데이트
- 새로운 구조를 반영하도록 모든 통합 및 E2E 테스트를 업데이트합니다.
- 특히 여러 테스트 파일에 분산된 `setup_fixtures` 함수들을 수정해야 합니다.

## 3. 리스크 및 고려 사항
- **하위 호환성**: 기존 사용자들에게는 브레이킹 체인지(Breaking Change)가 됩니다. 프로젝트 초기 단계이므로 평탄화된 구조를 표준으로 채택합니다.
- **이름 충돌**: 플러그인 폴더 이름이 전역 파일 이름(예: `AGENTS.md/`)과 동일할 경우 혼동이 생길 수 있으나, 이는 극히 드문 케이스로 간주합니다.
- **FORBIDDEN_FILES 처리**: 현재 `FileFilter`는 플러그인 내부에 `AGENTS.md` 등이 있으면 에러를 냅니다. 루트에 있는 경우에는 에러 대신 스캔 제외 처리가 필요합니다.
