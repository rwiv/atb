# PLAN: Build-time Resource Dependency Check

## 1. 개요 (Objective)
빌드 시 리소스 간의 의존성(`requirements.yaml`)을 검증하고, 누락된 리소스가 있을 경우 빌드를 중단하는 기능을 단계별로 구현합니다.

## 2. 페이즈 및 작업 목록 (Phases & Tasks)

### Phase 1: 기반 기능 확장 (Infrastructure Expansion)
- **Task 1-1**: `loader::registry::Registry`에 리소스 식별자(`plugin:name`) 기반의 존재 여부 확인 메서드(`contains_by_id`) 추가.
- **Task 1-2**: `core::ResourceType`에 복수형 문자열과의 변환 기능(예: "agents", "skills") 추가.

### Phase 2: 의존성 검사기 구현 (Dependency Checker)
- **Task 2-1**: `src/builder/dependency.rs` 신규 생성 및 `requirements.yaml` 파싱 모델 정의.
- **Task 2-2**: `DependencyChecker` 구조체 및 검사 로직 구현.
- **Task 2-3**: 플러그인별 `requirements.yaml` 파일 로딩 및 캐싱 메커니즘 추가.

### Phase 3: 빌드 파이프라인 통합 (Pipeline Integration)
- **Task 3-1**: `src/builder/mod.rs`에서 `Builder::run` 실행 시 변환(Transform) 전 단계에 의존성 검사 단계 삽입.
- **Task 3-2**: 의존성 누락 시 상세 오류 메시지 구성 및 `anyhow::Result` 반환.

### Phase 4: 검증 및 테스트 (Validation & Testing)
- **Task 4-1**: `requirements.yaml` 파싱 및 매칭 로직 유닛 테스트 작성.
- **Task 4-2**: 의존성 누락 시 빌드 실패 및 로그 확인을 위한 통합 테스트 작성.
- **Task 4-3**: 복합적인 의존성 상황(다양한 타입, 다중 플러그인) 시나리오 검증.

### Phase 5: 문서 갱신 (Documentation Update)
- **Task 5-1**: `SPEC.md` 및 `DESIGN.md`에 의존성 검사 규격 및 설계 내용 반영.
- **Task 5-2**: `src/builder/README.md`에 빌드 파이프라인의 의존성 검사 단계 설명 추가.

## 3. 의존성 및 제약 (Dependencies & Constraints)
- **의존성**: `Registry` 확장이 먼저 이루어져야 검사 로직 구현이 가능합니다.
- **제약**: `requirements.yaml` 내의 리소스 타입 키는 시스템에 정의된 `ResourceType`과 정확히 일치해야 합니다.

## 4. 성공 기준 (Success Criteria)
- `atb build` 실행 시 모든 명시된 의존성이 충족되면 정상적으로 빌드가 완료됨.
- 하나 이상의 의존성이 누락된 경우, 어떤 리소스가 무엇을 필요로 하는지 상세히 로깅하고 빌드가 중단됨.
- `requirements.yaml` 파일이 없는 경우에도 문제 없이 빌드가 진행됨.
