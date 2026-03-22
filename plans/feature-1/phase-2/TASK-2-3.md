# Task 2.3: 리소스 레지스트리 구축 및 이름 충돌 관리

## 1. Objective (목표)

- 프로젝트 전체에서 사용될 리소스들을 중앙 집중식으로 관리하는 `Registry` 저장소를 구축합니다.
- 서로 다른 플러그인에서 동일한 이름을 가진 리소스가 선택되었을 때 발생하는 충돌을 감지하고, 빌드를 안전하게 중단시키는 로직을 구현합니다.

## 2. Context & Files (작업 범위)

- **읽기 전용 (참고용):**
  - `specs/SPEC.md` (네임스페이스 관리 및 충돌 검사 규칙)
  - `specs/SPEC.md` (Registry 설계 및 에러 처리 전략)
- **생성 및 수정할 파일:**
  - `src/core/registry.rs` (신규 생성: 리소스 저장소 및 충돌 관리 로직)
  - `src/core/mod.rs` (수정: registry 모듈 선언)
  - `src/main.rs` (수정: 로더와 레지스트리를 연결하여 전체 로딩 흐름 완성)

## 3. Instructions (세부 지침)

### Step 1: `Registry` 구조체 구현

- `HashMap<String, Resource>`를 내부 저장소로 가지는 `Registry` 구조체를 정의하세요.
- 리소스를 등록하는 `register(resource: Resource)` 메서드를 제공합니다.

### Step 2: 이름 충돌 감지 및 에러 처리

- `register` 호출 시, 이미 동일한 이름의 키가 맵에 존재하는지 확인합니다.
- **충돌 발생 시:** 단순히 덮어쓰지 않고, 어떤 플러그인의 어떤 리소스가 충돌했는지 정보를 포함하여 `anyhow` 에러를 반환해야 합니다.
- 빌드 결과물은 플랫(Flat)하게 생성되므로, 소스의 플러그인 네임스페이스를 제외한 최종 파일명이 중복 체크의 기준이 됩니다.

### Step 3: 빌드 파이프라인 통합 및 필터링

- `main.rs`에서 `toolkit.yaml`의 `resources` 섹션(`commands`, `agents`, `skills`)에 정의된 리소스 식별자(예: `plugin_a:foo`)를 읽습니다.
- 로드된 리소스 중 `toolkit.yaml`에 명시된 리소스들만 선별하여 `Registry`에 등록하는 흐름을 구성하세요.

## 4. Constraints (제약 사항 및 금지 행동)

- 에러 메시지는 "Conflict detected: Resource 'foo' is defined in both 'plugin_a' and 'plugin_b'."와 같이 사용자가 즉시 조치할 수 있도록 상세해야 합니다.
- `toolkit.yaml`에 명시되지 않은 리소스는 로드되었더라도 `Registry`에 담기지 않아야 합니다.

## 5. Acceptance Criteria (검증 체크리스트)

1. 서로 다른 플러그인에서 같은 이름의 리소스를 빌드 대상으로 지정했을 때 에러를 발생시키며 빌드가 중단되는가?
2. `toolkit.yaml`에 정의된 리소스들만 최종적으로 `Registry`에 보관되는가?
3. 충돌 에러 메시지에 충돌이 발생한 리소스 이름과 소스 플러그인 이름이 정확히 포함되어 있는가?
4. `cargo test`를 통해 `Registry`의 등록 및 충돌 방지 로직이 검증되는가? (단위 테스트 작성 권장)
