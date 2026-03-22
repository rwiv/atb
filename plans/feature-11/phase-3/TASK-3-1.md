# TASK: Update ResourceLoader to load overrides.yaml

## 개요
`ResourceLoader`가 초기화될 때 소스 루트에서 `overrides.yaml`을 찾아 로드하도록 수정합니다.

## 작업 상세

### 1. `ResourceLoader` 구조체 수정 (`src/loader/mod.rs`)
- `metadata_map: Option<MetadataMap>` 필드를 추가합니다.

### 2. 초기화 로직 수정
- `new()` 메서드 내에서 소스 디렉터리의 `overrides.yaml`을 로드합니다.
- 파일이 없으면 에러가 아닌 `None` 또는 `Default`로 처리합니다.

### 3. `load()` 메서드 수정
- 로드된 `MetadataMap`을 `ResourceParser` 생성 시 전달하거나, 파싱 과정에서 사용할 수 있도록 구조를 조정합니다.

## 검증 방법
- `ResourceLoader`를 생성할 때 `overrides.yaml`이 있는 경우와 없는 경우를 각각 테스트하여 내부 상태가 올바르게 설정되는지 확인합니다.
