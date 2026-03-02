# Task 1.1: `Transformer` Trait 시그니처 변경 및 파급 오류 해결

## 1. 목표
Codex의 다중 에이전트 설정을 지원하기 위해 `Transformer` 트레이트를 확장합니다. 새로운 메서드를 추가하고 기존 메서드의 시그니처를 변경하여, 이후 단계에서 Codex 변환기가 전체 상태를 참조하거나(`post_transform`), 디스크상의 컨텍스트를 읽을 수 있도록(`detransform`) 합니다.

## 2. 작업 내용
1. `src/transformer/mod.rs`에서 `Transformer` 트레이트에 다음 훅을 추가합니다.
   ```rust
   fn post_transform(&self, _resources: &[&Resource]) -> Result<Vec<TransformedFile>> {
       Ok(vec![])
   }
   ```
2. `src/transformer/mod.rs`에서 `Transformer` 트레이트의 `detransform` 시그니처에 `output_dir: &Path`를 추가합니다.
   ```rust
   fn detransform(&self, r_type: ResourceType, file_content: &str, output_dir: &Path) -> Result<ResourceData>;
   ```
3. 위 트레이트 변경으로 인해 빌드가 실패하는 모든 구현체 모듈의 시그니처를 맞춥니다:
   - `src/transformer/codex.rs`
   - `src/transformer/gemini.rs`
   - `src/transformer/default.rs`
4. `src/syncer/mod.rs` 파일 내에서 `detransform`을 호출하는 부분을 수정하여 `output_dir` 인자를 전달합니다.
5. 각 모듈 내의 단위 테스트에서 `detransform`을 사용하는 부분을 찾아 테스트용 더미 `output_dir`을 전달하도록 수정합니다.

## 3. 성공 기준
- 변경된 `Transformer` 트레이트를 바탕으로 전체 코드가 `cargo check` 및 `cargo test`를 통과해야 합니다.
- (기능 동작 변경은 없으며, 시그니처만 일치시킵니다.)
