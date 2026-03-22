# TASK-1-2: 로더 및 패스 리졸버 수정

## 작업 내용

- [ ] `src/loader/mod.rs` 수정:
    - `ResourceLoader::new`에서 `PLUGINS_DIR_NAME`("plugins") 디렉터리 존재 여부 체크 로직 제거.
    - `self.root`를 `source_root`로 설정.
- [ ] `src/loader/resolver.rs` 수정 및 검증:
    - `resolve` 함수 내에서 `components.len() < 3` 체크가 루트 파일들을 적절히 무시하는지 확인.
    - `AGENTS.md` 등 루트 파일들이 실수로 플러그인 이름으로 해석되지 않도록 방어 로직 추가.
- [ ] `src/loader/mod.rs` 및 `src/loader/resolver.rs` 관련 기본 유닛 테스트 수정.

## 검증 방법

- `cargo test loader` 실행하여 리소스 로딩 로직 검증.
- `plugins/` 디렉터리 없이 루트에서 바로 리소스를 찾는 통합 테스트 케이스 추가.
