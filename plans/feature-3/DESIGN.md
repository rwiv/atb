# Design: Path Expansion and YAML Metadata Support

## 1. 개요
이 문서는 `toolkit.yaml`의 `source` 경로에서 `~` (홈 디렉터리) 확장을 지원하고, 리소스 메타데이터 포맷으로 기존 `.json` 외에 `.yaml`(.yml)을 하이브리드하게 지원하기 위한 설계를 기술합니다.

## 2. 요구 사항
- **경로 확장**: `source` 필드값이 `~/`로 시작하는 경우 시스템의 홈 디렉터리 절대 경로로 치환.
- **YAML 메타데이터 지원**: `.json` 뿐만 아니라 `.yaml` 및 `.yml` 확장자를 가진 메타데이터 파일을 인식하고 파싱.
- **충돌 감지**: 동일한 리소스에 대해 JSON과 YAML 메타데이터가 동시에 존재할 경우 빌드 실패 처리.
- **하이브리드 지원**: 플러그인 내 리소스별로 다른 메타데이터 포맷 사용 가능.

## 3. 아키텍처 및 상세 설계

### 3.1 경로 확장 (Config Layer)
- **라이브러리**: `shellexpand` 크레이트 도입.
- **변경 위치**: `src/builder/config.rs`
- **로직**:
    ```rust
    // Config 로드 직후 확장 수행
    let mut config = parse_config(&content)?;
    config.source = shellexpand::tilde(&config.source).into_owned();
    ```

### 3.2 하이브리드 메타데이터 (Resource Layer)
- **변경 위치**: `src/resource/loader.rs`
- **로직**:
    - **스캔**: `scan_plugins`에서 `yaml`, `yml` 확장자 허용.
    - **그룹화 및 충돌 검사**:
        ```rust
        // groups: HashMap<ResourceKey, (Option<PathBuf>, Option<PathBuf>)>
        // json_path(Option<PathBuf>) 필드에 경로를 할당할 때 이미 존재하면 에러
        if entry.1.is_some() {
            bail!("Conflict: Multiple metadata formats found for resource...");
        }
        ```
    - **파싱**: 확장자에 따라 `serde_json` 또는 `serde_yaml` 호출하여 `serde_json::Value` 생성.

## 4. 테스트 전략
- **경로 확장**: `~/` 문자열이 실제 홈 디렉터리로 치환되는지 단위 테스트.
- **메타데이터**: 
    - YAML 전용 리소스 로드 테스트.
    - JSON + YAML 혼합 사용 테스트.
    - JSON/YAML 동시 존재 시 에러 발생 테스트.
