# TASK: overrides.yaml Parsing Logic

## 개요
소스 루트의 `overrides.yaml` 파일을 읽어 `MetadataMap` 객체로 변환하는 기능을 구현합니다.

## 작업 상세

### 1. 파싱 함수 구현
- `src/utils/yaml.rs` 또는 `src/core/model.rs` (또는 적절한 유틸리티 모듈)에 `overrides.yaml`을 파싱하는 함수를 작성합니다.
- `serde_yaml`을 사용하여 파일을 읽습니다.

```rust
pub fn load_metadata_map(path: &Path) -> Result<MetadataMap> {
    if !path.exists() {
        return Ok(MetadataMap::default());
    }
    let content = fs::read_to_string(path)?;
    let map: MetadataMap = serde_yaml::from_str(&content)?;
    Ok(map)
}
```

## 검증 방법
- `plans/feature-11/map_example.yaml` 내용을 기반으로 테스트 코드를 작성하여 `MetadataMap` 객체가 올바른 구조로 생성되는지 확인합니다.
