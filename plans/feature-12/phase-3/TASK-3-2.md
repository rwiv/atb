# TASK 3-2: Implement Detailed Error Reporting

## 개요 (Description)
의존성 누락 시 사용자에게 친절하고 상세한 오류 메시지를 출력합니다.

## 수정 파일 (Files to Modify)
- `src/builder/dependency.rs`

## 상세 지침 (Actionable Instructions)
1. 수집된 누락 의존성 목록을 바탕으로 에러 메시지를 생성합니다.
2. 메시지 형식 예시:
   ```text
   Dependency check failed:
     - agent 'p1:a1' requires skill 'p2:s1' but it is missing in toolkit.yaml.
     - skill 'p2:s2' requires command 'p3:c1' but it is missing in toolkit.yaml.
   ```
3. `anyhow::bail!` 또는 `anyhow::anyhow!`를 사용하여 최종 에러를 반환합니다.

## 검증 방법 (Verification)
- 의존성이 누락된 시나리오에서 출력되는 에러 메시지가 의도한 형식과 일치하는지 확인합니다.
