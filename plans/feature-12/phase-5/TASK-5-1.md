# TASK 5-1: Update `SPEC.md` and `DESIGN.md`

## 개요 (Description)
리소스 의존성 검사 기능이 추가됨에 따라, 시스템의 기술 규격(`SPEC.md`)과 전체 설계 문서(`DESIGN.md`)를 최신 상태로 갱신합니다.

## 수정 파일 (Files to Modify)
- `specs/SPEC.md`
- `specs/DESIGN.md`

## 상세 지침 (Actionable Instructions)
1. **`specs/SPEC.md`**:
    - 리소스 작성 규격 섹션에 `requirements.yaml` 파일의 위치와 포맷(YAML 구조) 설명을 추가합니다.
    - 빌드 프로세스 규격에 의존성 검증 단계와 실패 시 동작(Fail-fast)을 명시합니다.
2. **`specs/DESIGN.md`**:
    - 빌드 파이프라인(Build Pipeline) 다이어그램에 의존성 검사(Dependency Check) 단계를 추가합니다.
    - `DependencyChecker` 모듈의 역할과 `Registry`와의 상호작용 설계를 요약하여 반영합니다.

## 검증 방법 (Verification)
- 수정된 마크다운 문서들의 내용이 구현된 기능과 일치하는지 검토합니다.
- `check-docs` 커맨드(존재하는 경우) 등을 통해 링크 깨짐이나 포맷 오류가 없는지 확인합니다.
