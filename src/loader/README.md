# Resource Loader 모듈

이 모듈은 파일 시스템에서 플러그인 구조를 스캔하고, 분산된 파일들을 읽어 `Resource` 객체로 로드하는 핵심 로직을 담당합니다.

## 핵심 역할

1.  **플러그인 스캔**: 소스 디렉터리를 재귀적으로 탐색하여 유효한 파일 목록을 수집합니다.
2.  **파일 필터링**: 설정된 제외 패턴과 보안 규칙에 따라 불필요하거나 금지된 파일을 걸러냅니다.
3.  **리소스 그룹화**: 파일 이름과 경로 구조를 분석하여 흩어져 있는 Markdown 본문과 메타데이터 파일을 하나의 리소스로 결합합니다.
4.  **메타데이터 파싱**: YAML, YML 형식의 메타데이터를 통합된 데이터 모델로 변환합니다.

## 주요 구성 요소

### 1. ResourceLoader 및 모듈 인터페이스 (`mod.rs`)

모듈의 엔트리포인트로, 스캔, 해석(`ResourcePathResolver`), 조립(`ResourceParser`) 단계를 오케스트레이션합니다.

- **`ResourceLoader` (Public API)**: 플러그인 디렉터리를 탐색하고 실제 리소스 객체를 조립하는 핵심 객체입니다.
    - `load()`: 모든 유효한 리소스를 로드하여 반환.

### 2. Loader 모델 (`model.rs`)

- **`ScannedResource`**: 파일 스캔 단계에서 생성되며 플러그인, 이름, 그리고 원시 경로 정보를 담습니다.
- **`ScannedPaths`**: 리소스 타입별 파일 경로 구성을 강제하는 Enum입니다.

### 3. MetadataMerger (`merger.rs`)

Frontmatter, Metadata Map, 외부 메타데이터를 통합하는 단일 책임 모듈입니다.

- **필드 매핑**: `overrides.yaml` 규칙에 따라 필드 값을 타겟별로 치환합니다.
- **오버라이트**: 타겟 전용 예약어 섹션 병합을 수행합니다.

### 4. ResourcePathResolver (`resolver.rs`)

파일 경로를 분석하여 리소스 단위(`ScannedResource`)로 그룹화합니다.

- **Commands & Agents**: `[plugin]/[type]/[name].{md,yaml,yml}` 구조를 분석합니다.
- **Skills**: `[plugin]/skills/[skill_name]/` 폴더 내의 파일들을 그룹화합니다.
- **포맷 충돌 검증**: 동일 리소스에 대해 YAML과 YML 메타데이터가 공존할 경우 에러를 발생시킵니다.

### 5. ResourceParser (`parser.rs`)

`ScannedResource` 정보를 기반으로 메타데이터 파싱과 최종 `Resource` 객체 조립을 담당합니다.

- **메타데이터 통합**: `MetadataMerger`를 사용하여 본문(Frontmatter)과 외부 메타데이터, 전역 매핑 규칙을 병합합니다.

### 6. Registry (`registry.rs`)

로드된 `Resource` 객체들을 메모리에 보관하고 관리하는 중앙 저장소 역할을 합니다. 동일 타입 내 이름 중복을 방지합니다.

## 리소스 그룹화 규칙


| 리소스 타입 | 구조 | 메타데이터 매칭 | 본문 매칭 | 추가 파일 |
| :--- | :--- | :--- | :--- | :--- |
| **Commands/Agents** | 파일 기반 | 파일 이름이 동일한 YAML/YML | 파일 이름이 동일한 `.md` | 지원하지 않음 |
| **Skills** | 폴더 기반 | `SKILL.yaml/yml` | `SKILL.md` | 폴더 내 기타 모든 파일 (extras) |


## 구현 상세 및 정책

- **필터링**: 파일 유효성 검사는 `core::FileFilter`를 사용하며, 숨김 파일 및 보안상 금지된 파일을 엄격히 걸러냅니다.
- **중복 검증**: 동일한 리소스 이름에 대해 여러 메타데이터 형식이 발견되면 즉시 빌드를 중단합니다.
