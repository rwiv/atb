# TASK: Implement apply_mapping in MetadataMerger

## 개요
`MetadataMap`을 사용하여 필드 값을 타겟에 맞게 치환하는 핵심 로직을 구현합니다.

## 작업 상세

### 1. `apply_mapping` 메서드 구현
- `MetadataMerger` 클래스에 `apply_mapping`을 추가합니다.
- 제약 조건:
    - 필드 이름이 `description`이 아니어야 함.
    - 필드 값이 `string` 타입이어야 함.
    - `MetadataMap`에 해당 필드와 원본 값이 정의되어 있어야 함.
    - 현재 타겟(`BuildTarget`)에 대한 매핑 값이 존재해야 함.

```rust
fn apply_mapping(&self, base: &mut Value, map: &MetadataMap) -> Result<()> {
    if let Some(obj) = base.as_object_mut() {
        for (field, value) in obj.iter_mut() {
            if field == "description" { continue; }
            
            if let Some(val_str) = value.as_str() {
                if let Some(field_mappings) = map.mappings.get(field) {
                    if let Some(target_mappings) = field_mappings.get(val_str) {
                        if let Some(mapped_val) = target_mappings.get(&self.target) {
                            *value = Value::String(mapped_val.clone());
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
```

### 2. `merge` 메서드에 통합
- `merge` 메서드 시작 부분에서 `apply_mapping`을 호출하도록 수정합니다.

## 검증 방법
- `overrides.yaml` 설정과 다양한 `base` 값을 시뮬레이션하여 매핑이 올바르게 적용되는지 확인하는 단위 테스트를 작성합니다.
