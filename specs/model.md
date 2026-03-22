# Core Data Models

본 문서는 `atb` 시스템의 핵심 데이터 모델과 내부 상태를 관리하는 구조체들을 정의합니다.

## 1. 리소스 모델 (Core Resources)

`src/core/resource.rs`에 정의된 시스템의 기본 단위입니다.

### 1.1 ResourceType

지원하는 리소스의 타입을 정의합니다.

```rust
pub enum ResourceType {
    Command,
    Agent,
    Skill,
}
```

### 1.2 ResourceData

리소스의 공통 속성을 담는 기본 구조체입니다.

```rust
pub struct ResourceData {
    pub name: String,
    pub plugin: String,
    pub content: String,
    pub metadata: serde_json::Value,
    pub source_path: PathBuf, // Command/Agent: .md 파일 경로, Skill: 디렉터리 경로
}
```

### 1.3 Resource (Main Enum)

도메인 모델의 최상위 Enum입니다.

```rust
pub enum Resource {
    Command(ResourceData),
    Agent(ResourceData),
    Skill(SkillData),
}

pub struct SkillData {
    pub base: ResourceData,
    pub extras: Vec<ExtraFile>,
}

pub struct ExtraFile {
    pub source: PathBuf, // 원본 절대 경로
    pub target: PathBuf, // 대상 상대 경로 (예: skills/foo/extra.txt)
}
```

## 2. 로더 및 레지스트리 모델 (Loader & Registry)

`src/loader/`에서 리소스 스캔 및 보관을 위해 사용하는 모델입니다.

### 2.1 ScannedResource

파일 시스템 스캔 단계에서 생성되는 원시 데이터 구조입니다.

```rust
pub struct ScannedResource {
    pub plugin: String,
    pub name: String,
    pub paths: ScannedPaths,
}

pub enum ScannedPaths {
    Command { md: Option<PathBuf>, metadata: Option<PathBuf> },
    Agent { md: Option<PathBuf>, metadata: Option<PathBuf> },
    Skill { md: Option<PathBuf>, metadata: Option<PathBuf>, extras: Vec<PathBuf> },
}
```

### 2.2 Registry

로드된 리소스들을 메모리에 보관하고 중복을 검증하는 중앙 저장소입니다.

```rust
pub struct Registry {
    /// Key: (ResourceType, Name)
    resources: HashMap<(ResourceType, String), Resource>,
}
```

## 3. 빌드 및 변환 모델 (Build & Transform)

변환 과정과 결과물을 나타내는 모델입니다.

### 3.1 BuildTarget

`src/core/target.rs`에 정의된 빌드 대상 에이전트입니다.

```rust
pub enum BuildTarget {
    #[serde(rename = "gemini-cli")]
    GeminiCli,
    #[serde(rename = "claude-code")]
    ClaudeCode,
    #[serde(rename = "opencode")]
    OpenCode,
}
```

### 3.2 TransformedResource

변환이 완료되어 배포(Emit)를 기다리는 최종 결과물 묶음입니다.

```rust
pub struct TransformedResource {
    pub files: Vec<TransformedFile>, // 텍스트 변환된 파일들
    pub extras: Vec<ExtraFile>,      // 물리적으로 복사될 파일들
}

pub struct TransformedFile {
    pub path: PathBuf,               // 저장될 상대 경로
    pub content: String,             // 변환된 내용
}
```
