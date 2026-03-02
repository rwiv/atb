# Task 3.1: Codex Agent 역변환 시 `.codex/config.toml` 파싱 로직 추가

## 1. 목표
로컬에서 수정된 Codex 프로젝트를 원본 저장소로 `Sync`할 때, 변경된 `description`을 반영하기 위해 `detransform` 과정에서 `.codex/config.toml` 파일의 값을 함께 읽어들여 역변환된 `ResourceData` 객체를 구성합니다.

## 2. 작업 내용
1. `src/transformer/codex.rs` 내부의 `detransform` 메서드 중 `ResourceType::Agent` 브랜치 수정:
   - 기존의 역변환 로직에서 `"developer_instructions"` 키를 찾아 본문(`content`)으로 복원합니다.
   - `output_dir.join(".codex/config.toml")` 경로의 파일을 열고 TOML 파싱합니다. (파일이 존재할 경우만 시도, 없으면 기본값 유지)
   - 해당 TOML에서 `[agents.<name>.description]` 값을 추출합니다. (이때 `<name>`은 타겟 에이전트의 이름으로, `detransform`에서는 파일명이나 컨텍스트를 통해 유추할 수 있도록 파일명이 필요할 수 있습니다. 필요하다면 역변환 시그니처나 로직의 보완을 고려합니다. - *참고: 만약 `detransform`만으로 이름을 알기 어렵다면, 파싱한 `metadata`나 다른 방식을 통해 파일명(이름)을 유추해야 합니다.*)
   - 복원된 `description` 값을 역변환된 `ResourceData`의 `metadata`에 다시 삽입합니다.

*참고사항*: `detransform`은 타겟 파일의 문자열만 받기 때문에 이름을 바로 알기 어려울 수 있습니다. 이 경우 `Syncer`가 파일 경로를 기반으로 역변환을 수행하므로 이름을 인자로 전달하거나, `ResourceData`의 `name` 값을 활용하는 방법을 함께 적용해야 할 수 있습니다. 이에 맞춰 필요한 경우 `detransform` 시그니처에 `name: &str` 등을 추가하는 작업이 Phase 1 또는 여기서 발생할 수 있습니다.

## 3. 성공 기준
- 에이전트의 역변환 결과에 `content`와 `.codex/config.toml`에서 가져온 `description`이 정상적으로 담깁니다.
- 연관된 Sync 단위 테스트나 e2e 테스트에서 변경된 description이 마크다운 파일에 잘 패치되는 것을 확인합니다.
