# TASK: Define MetadataMap Model

## 개요
`overrides.yaml`의 계층 구조를 메모리 모델로 표현하기 위해 `src/core/model.rs`에 `MetadataMap` 구조체를 정의합니다.

## 작업 상세

### 1. `MetadataMap` 구조체 정의
- `src/core/model.rs` 파일에 다음 구조를 추가합니다.
- 계층 구조: `Field Name -> Original Value -> BuildTarget -> Mapped Value`
- `BuildTarget`을 키로 사용할 수 있도록 `std::collections::HashMap`을 활용합니다.

```rust
use std::collections::HashMap;
use crate::core::BuildTarget;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MetadataMap {
    /// Field mappings: { field_name: { original_value: { target: mapped_value } } }
    pub mappings: HashMap<String, HashMap<String, HashMap<BuildTarget, String>>>,
}
```

## 검증 방법
- 컴파일 에러가 없는지 확인합니다.
- 필요 시 간단한 단위 테스트를 추가하여 `BuildTarget`을 키로 하는 `HashMap`이 정상적으로 동작하는지 확인합니다.
