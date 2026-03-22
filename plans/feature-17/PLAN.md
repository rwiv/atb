# PLAN: Core Configuration File Renaming

이 계획은 `atb` 프로젝트의 핵심 설정 파일 이름을 보다 직관적이고 도메인 친화적인 이름으로 변경하는 절차를 정의합니다.

## 변경 사항 요약

| 기존 파일명 | 새로운 파일명 | 역할 |
| :--- | :--- | :--- |
| `toolkit.yaml` | `toolkit.yaml` | 프로젝트 빌드 및 리소스 정의 (Main Config) |
| `overrides.yaml` | `overrides.yaml` | 타겟별 메타데이터 매핑 규칙 |
| `requirements.yaml` | `requirements.yaml` | 리소스 간 의존성 정의 |

## 단계별 실행 계획

### Phase 1: 상수 및 코드 로직 수정
시스템 전역에서 사용하는 파일명 상수를 변경하고, 하드코딩된 로직을 수정합니다.

- **Task 1-1**: `src/core/constants.rs`의 상수 값 변경 (`CONFIG_FILE_NAME`, `DEPS_FILE_NAME`).
- **Task 1-2**: `src/loader/mod.rs` 내의 `overrides.yaml` 참조를 `overrides.yaml`로 수정.
- **Task 1-3**: 에러 메시지 및 주석 내의 파일명 일괄 수정.

### Phase 2: 테스트 코드 수정
테스트 환경에서 사용하는 파일 생성 및 검증 로직을 새로운 파일명에 맞게 업데이트합니다.

- **Task 2-1**: `tests/` 디렉터리 내의 모든 통합/E2E 테스트 코드 수정.
- **Task 2-2**: `src/builder/dependency.rs` 등 모듈 내 유닛 테스트 수정.

### Phase 3: 문서 업데이트
사용자 가이드 및 기술 설계 문서의 파일명을 최신화합니다.

- **Task 3-1**: 루트 `README.md` 및 `specs/` 내 문서들 수정.
- **Task 3-2**: 각 모듈별 `README.md` 및 기존 `plans/` 문서 수정 (일관성 유지).

### Phase 4: 최종 검증
변경 사항이 시스템 전체에서 올바르게 작동하는지 확인합니다.

- **Task 4-1**: 전체 테스트 세트 실행 (`cargo test`).
- **Task 4-2**: 샘플 리소스를 이용한 빌드 및 동기화 동작 수동 검증.

## 주의 사항
- `src/core/constants.rs`를 먼저 수정하여 컴파일 에러를 유도함으로써 수정이 필요한 지점을 파악하는 전략을 사용합니다.
- 기존 사용자의 하위 호환성은 고려하지 않으며, 전면 교체를 원칙으로 합니다.
