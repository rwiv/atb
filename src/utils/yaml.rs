use serde_json::json;

/// 마크다운 본문에서 Frontmatter를 추출하여 (메타데이터, 순수 본문) 쌍으로 반환합니다.
pub fn extract_frontmatter(content: &str) -> (serde_json::Value, String) {
    let content = content.trim_start();
    if !content.starts_with("---") {
        return (json!({}), content.to_string());
    }

    // 첫 번째 "---" 이후부터 두 번째 "---"를 찾음
    let rest = &content[3..];
    if let Some(end_offset) = rest.find("---") {
        let yaml_str = &rest[..end_offset];
        let pure_content = &rest[end_offset + 3..];

        match serde_yaml::from_str::<serde_json::Value>(yaml_str) {
            Ok(metadata) => {
                // 앞쪽 개행문자만 제거 (Frontmatter와 본문 사이의 구분용 개행)
                let pure_content = pure_content.trim_start_matches(['\r', '\n']);
                (metadata, pure_content.to_string())
            }
            Err(_) => (json!({}), content.to_string()),
        }
    } else {
        (json!({}), content.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extract_standard() {
        let input = "---
name: test
---
Hello world";
        let (meta, content) = extract_frontmatter(input);
        assert_eq!(meta, json!({"name": "test"}));
        assert_eq!(content, "Hello world");
    }

    #[test]
    fn test_extract_no_frontmatter() {
        let input = "Hello world";
        let (meta, content) = extract_frontmatter(input);
        assert_eq!(meta, json!({}));
        assert_eq!(content, "Hello world");
    }

    #[test]
    fn test_extract_invalid_yaml() {
        let input = "---
name: : invalid
---
Hello world";
        let (meta, content) = extract_frontmatter(input);
        assert_eq!(meta, json!({}));
        assert_eq!(content, input.trim_start());
    }

    #[test]
    fn test_extract_empty_frontmatter() {
        let input = "---
---
Hello world";
        let (_meta, content) = extract_frontmatter(input);
        assert_eq!(content, "Hello world");
    }
}
