# TASK 4-2: Configure context loading for `Syncer`

## 개요 (Description)
`agb sync`가 실행될 때 빌드 컨텍스트(`Config`)를 정확하게 로드하고, `Syncer`가 타겟 디렉터리에 접근할 수 있도록 경로 구성을 설정합니다.

## 수정 파일 (Files to Modify)
- `src/builder/config.rs` (필요 시 수정)
- `src/main.rs` (기존 로직 재사용)

## 상세 지침 (Actionable Instructions)
1. `Config` 로직에서 소스 디렉터리와 타겟 디렉터리의 위치가 명확히 분리되어 로드되는지 확인합니다.
2. `toolkit.yaml` 파일에서 `source`와 `target` 필드를 읽어 `Syncer`에 전달할 준비를 합니다.
3. `Syncer` 실행 시 빌드 정보(`BuildTarget`)가 정확히 일치하는지 확인합니다. (예: `target: gemini-cli`면 `GeminiTransformer` 활용)
4. 소스 디렉터리에 접근 가능한지 권한 체크를 수행합니다.

## 검증 방법 (Verification)
- `cargo run -- build`와 `cargo run -- sync`가 동일한 `toolkit.yaml`을 기반으로 일관된 경로를 인식하는지 테스트합니다.
- `exclude` 패턴이 `Syncer` 로딩 시에도 정확히 전달되는지 확인합니다.
