# Task 2.1: `App` 구조체 구현 및 명령어 분기 로직 이동

## 1. 개요 (Overview)
`src/app/mod.rs`에 `App` 구조체를 정의하고, `main.rs`의 `Build`/`Sync` 명령어 처리 로직을 `App`의 메서드로 이동합니다.

## 2. 작업 상세 (Implementation Details)

### 2.1. `App` 구조체 및 명령어 분기 로직 이동
`src/app/mod.rs`에 다음 구조를 구현합니다. `Builder`와 `Syncer`에 있던 반복(Iteration) 로직을 이 모듈의 내부 메서드(`build`, `sync`)로 흡수합니다.

```rust
impl App {
    pub fn new() -> Self {
        Self
    }

    /// App의 실행 진입점으로, CLI 입력을 받아 명령어 분기를 수행합니다.
    pub fn run(&self, cli: Cli) -> anyhow::Result<()> {
        let config_file = match &cli.command {
            Commands::Build { config } => config.as_deref().unwrap_or("toolkit.yaml"),
            Commands::Sync { config } => config.as_deref().unwrap_or("toolkit.yaml"),
        };

        let ctx = AppContext::init(config_file)?;

        match &cli.command {
            Commands::Build { .. } => self.build(&ctx),
            Commands::Sync { .. } => self.sync(&ctx),
        }
    }

    /// 빌드 명령어 처리 (기존 Builder::run 로직 이동)
    fn build(&self, ctx: &AppContext) -> anyhow::Result<()> {
        println!("Building resources...");
        // Registry 순회 및 Builder::build_resource 호출 (예시)
        for res in ctx.registry.all_resources() {
             // ...
        }
        Ok(())
    }

    /// 동기화 명령어 처리 (기존 Syncer::run 로직 이동)
    fn sync(&self, ctx: &AppContext) -> anyhow::Result<()> {
        println!("Syncing target changes...");
        // Registry 순회 및 Syncer::sync_resource 호출
        for res in ctx.registry.all_resources() {
             // ...
        }
        Ok(())
    }
}
```

### 2.2. `src/builder/mod.rs` 및 `src/syncer/mod.rs` 리팩토링
- 각 모듈에서 `run` 메서드를 제거합니다.
- 단일 리소스를 처리하는 핵심 함수(예: `sync_resource`)를 `pub`으로 변경하여 `App`에서 접근 가능하도록 합니다.

## 3. 검증 방법 (Verification)
- `cargo build` 명령을 통해 컴파일 에러가 발생하지 않는지 확인합니다.
- `App::run` 메서드가 기존 `main` 함수의 로직과 동일하게 동작하는지 검토합니다.
