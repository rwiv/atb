# TASK 4-1: Add `sync` Subcommand to CLI

## 개요 (Description)
`agb` CLI의 서브커맨드로 `sync`를 추가하여 사용자가 명령어를 실행할 수 있도록 통합합니다.

## 수정 파일 (Files to Modify)
- `src/main.rs`

## 상세 지침 (Actionable Instructions)
1. `clap`을 사용하는 `Cli` 구조체(또는 `Commands` 열거형)에 `Sync` 서브커맨드를 추가합니다.
2. `Sync` 커맨드는 `agb build`와 동일하게 빌드 설정 파일(`toolkit.yaml`)을 인자로 받을 수 있도록 합니다.
3. `main.rs`의 매치 구문에 `Commands::Sync` 분기를 추가합니다.
4. `Syncer` 객체를 생성하고 `run()` 메서드를 호출하도록 구현합니다.
5. `Dry Run`의 경우, 현재 단계에서 구현하기보다 인자만 열어두고 실제 적용은 다음 기회로 미룰 수 있습니다. (구현이 필요하다면 태스크를 추가합니다.)

## 검증 방법 (Verification)
- `cargo run -- sync`를 실행하여 도움말(Help) 및 명령어 실행 여부를 확인합니다.
- `toolkit.yaml`을 인자로 넘겨 `sync`가 정상적으로 호출되는지 확인합니다.
