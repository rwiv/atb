# Task 2.2: `post_transform`을 통한 `.codex/config.toml` 생성

## 1. 목표
모든 개별 리소스 변환이 끝난 후, 생성된 에이전트들의 메타데이터와 경로 정보를 모아 프로젝트 전역 설정 파일인 `.codex/config.toml` 파일을 자동으로 생성합니다.

## 2. 작업 내용
1. `src/transformer/codex.rs`에서 `Transformer::post_transform` 오버라이드 구현:
   - 인자로 받은 `resources` 목록을 순회하며 `ResourceType::Agent`에 해당하는 리소스들만 필터링합니다.
   - TOML 객체 트리를 생성하여 `[agents.<name>]` 블록을 구성합니다.
   - 각 블록에 `description` 필드(원본 `ResourceData`의 메타데이터에서 추출)와 `config_file` 필드(`"agents/<name>.toml"`)를 매핑합니다.
   - 최종 완성된 TOML 문자열을 `TransformedFile` 래퍼로 감싸 반환합니다. (출력 경로는 `.codex/config.toml`)
2. `src/builder/mod.rs`의 `Builder::build` 프로세스 업데이트:
   - 각 리소스에 대해 `transformer.transform`을 호출한 후, 모은 리소스 배열을 이용해 `transformer.post_transform`을 호출합니다.
   - `post_transform`에서 반환된 추가 파일(`Vec<TransformedFile>`)도 최종 생성 목록(`TransformedResource` 등)에 병합하여 Emitter로 넘겨줍니다.

## 3. 성공 기준
- 빌드 파이프라인에서 `.codex/config.toml` 파일이 생성되어 반환됨.
- `cargo test` 실행 시 에러가 없어야 하며, 가능하다면 통합/e2e 빌드 테스트에서 `.codex/config.toml` 생성을 검증해야 합니다.
