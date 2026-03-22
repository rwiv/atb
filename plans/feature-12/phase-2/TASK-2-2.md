# TASK 2-2: Implement `DependencyChecker` Logic

## 개요 (Description)
`Registry`에 등록된 리소스들을 순회하며 의존성을 검증하는 핵심 엔진을 구현합니다.

## 수정 파일 (Files to Modify)
- `src/builder/dependency.rs`

## 상세 지침 (Actionable Instructions)
1. `DependencyChecker` 구조체를 정의합니다.
2. `pub fn check_dependencies(&self, registry: &Registry, source_dir: &Path) -> anyhow::Result<()>` 메서드를 구현합니다.
3. 모든 리소스를 순회하며 각 리소스의 플러그인 디렉터리 내 `requirements.yaml`을 로드합니다.
4. 로드된 의존성 목록의 각 항목이 `registry.contains_by_id`를 만족하는지 검사합니다.
5. 누락된 의존성이 발견되면 즉시 중단하지 않고 모두 수집합니다.

## 검증 방법 (Verification)
- `Registry` 모킹(Mocking)을 통해 의존성이 충족된 경우와 누락된 경우를 시뮬레이션하는 테스트를 작성합니다.
