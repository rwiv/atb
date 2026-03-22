# PLAN: Metadata Map Implementation & Refactoring

## 목표
- `overrides.yaml`을 통한 타겟별 메타데이터 자동 매핑 기능 구현.
- 메타데이터 병합 로직을 `MetadataMerger` 모듈로 분리하여 설계 개선.

## Phase 1: 데이터 모델 및 기초 작업
- [ ] `src/core/model.rs`에 `MetadataMap` 관련 데이터 구조 정의.
- [ ] `src/core/target.rs`에 `BuildTarget`과 문자열 간의 변환 유틸리티 확인/추가.
- [ ] `overrides.yaml` 파일을 파싱하기 위한 로직 구현 (serde 활용).

## Phase 2: MetadataMerger 구현
- [ ] `src/loader/merger.rs` 파일 신규 생성.
- [ ] `ResourceParser::merge_metadata` 로직을 `MetadataMerger`로 이관.
- [ ] `MetadataMap`을 기반으로 필드 값을 치환하는 `apply_mapping` 메서드 구현.
    - `description` 필드 제외 처리.
    - `string` 타입 데이터만 매핑 적용.
- [ ] `MetadataMerger` 단위 테스트 작성 (매핑 우선순위 및 제약 조건 검증).

## Phase 3: 시스템 통합 및 리팩터링
- [ ] `src/loader/mod.rs`: `ResourceLoader`가 소스 루트에서 `overrides.yaml`을 로드하도록 수정.
- [ ] `src/loader/parser.rs`: `ResourceParser`가 `MetadataMerger`를 사용하도록 수정.
- [ ] 기존 `merge_metadata` 관련 중복 코드 제거 및 정리.
- [ ] `ResourceLoader`와 `ResourceParser` 간의 `MetadataMap` 전달 구조 최적화.

## Phase 4: 검증 및 테스트
- [ ] 통합 테스트(`tests/`)에 `overrides.yaml` 매핑 케이스 추가.
- [ ] 기존 리소스 빌드 결과물에 영향이 없는지 회귀 테스트 수행.
- [ ] `agb sync` 동작 시 매핑된 값이 아닌 원본 데이터의 보존 여부 확인 (설계 상 매핑은 빌드 타겟 생성 시에만 적용되므로 안전해야 함).
