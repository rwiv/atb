use regex::Regex;

pub struct MdPatcher {
    raw_content: String,
}

impl MdPatcher {
    pub fn new(content: &str) -> Self {
        Self {
            raw_content: content.to_string(),
        }
    }

    /// description 필드만 업데이트 (기존 update_description 로직)
    /// 멀티라인 description이 감지되면 에러를 반환합니다.
    pub fn update_description(&mut self, new_desc: &str) -> anyhow::Result<()> {
        if new_desc.contains('\n') {
            anyhow::bail!("Multi-line description is not supported in sync");
        }

        let content = self.raw_content.trim_start();
        if !content.starts_with("---") {
            // Frontmatter가 없는 경우 새로 생성
            self.raw_content = format!("---\ndescription: {}\n---\n\n{}", new_desc, self.raw_content);
            return Ok(());
        }

        let rest = &content[3..];
        let end_offset = match rest.find("---") {
            Some(offset) => offset,
            None => {
                // 닫는 ---가 없는 경우 (잘못된 형식), 안전하게 앞에 추가
                self.raw_content = format!("---\ndescription: {}\n---\n\n{}", new_desc, self.raw_content);
                return Ok(());
            }
        };

        let yaml_part = rest[..end_offset].trim();
        let pure_content = rest[end_offset + 3..].trim_start();

        let mut lines: Vec<String> = yaml_part.lines().map(|s| s.to_string()).collect();
        let mut found_idx = None;

        // description: 키를 찾아 교체 (공백 및 인용부호 허용)
        let re = Regex::new(r#"^(\s*description:\s*)(?:'[^']*'|"[^"]*"|.*)$"#).unwrap();

        for (i, line) in lines.iter().enumerate() {
            if re.is_match(line) {
                // 멀티라인 마커 (| 또는 >) 감지
                let trimmed = line.trim();
                if trimmed.ends_with('|') || trimmed.ends_with('>') {
                    anyhow::bail!("Multi-line description (YAML marker) detected in source");
                }

                // 다음 줄 들여쓰기 감지 (멀티라인 데이터 감지)
                if i + 1 < lines.len() && lines[i + 1].starts_with(' ') {
                    anyhow::bail!("Multi-line description (Indentation) detected in source");
                }

                found_idx = Some(i);
                break;
            }
        }

        if let Some(i) = found_idx {
            let prefix = re.captures(&lines[i]).unwrap().get(1).unwrap().as_str();
            lines[i] = format!("{}{}", prefix, new_desc);
        } else {
            // 못 찾았다면 마지막에 추가
            lines.push(format!("description: {}", new_desc));
        }

        self.raw_content = format!("---\n{}\n---\n\n{}", lines.join("\n"), pure_content);

        Ok(())
    }

    /// 본문 영역만 교체 (기존 replace_content 로직)
    pub fn replace_body(&mut self, new_body: &str) {
        let content = self.raw_content.trim_start();
        if !content.starts_with("---") {
            self.raw_content = new_body.to_string();
            return;
        }

        let rest = &content[3..];
        if let Some(end_offset) = rest.find("---") {
            let yaml_part = rest[..end_offset].trim();
            // Frontmatter 영역을 유지하고 본문만 교체
            // new_body 앞에 개행 문자가 중복되는 것을 방지하기 위해 trim_start_matches 사용
            let new_body = new_body.trim_start_matches(['\r', '\n']);
            self.raw_content = format!("---\n{}\n---\n\n{}", yaml_part, new_body);
        } else {
            self.raw_content = new_body.to_string();
        }
    }

    /// 최종 마크다운 문자열 렌더링
    pub fn render(&self) -> String {
        self.raw_content.clone()
    }

    /// 본문 영역만 추출합니다.
    fn get_body(&self) -> &str {
        let content = self.raw_content.trim_start();
        if !content.starts_with("---") {
            return content;
        }

        let rest = &content[3..];
        if let Some(end_offset) = rest.find("---") {
            let pure_content = &rest[end_offset + 3..];
            pure_content.trim_start_matches(['\r', '\n'])
        } else {
            content
        }
    }

    /// 텍스트가 1글자라도 다르면 true를 반환합니다. (기존 diff_content 로직)
    pub fn has_changed(&self, other: &str) -> bool {
        self.get_body() != other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_changed() {
        let patcher = MdPatcher::new("hello");
        assert!(patcher.has_changed("world"));
        assert!(!patcher.has_changed("hello"));
        assert!(patcher.has_changed("hello\n"));
    }

    #[test]
    fn test_replace_body() {
        let source = "---
name: test
---
# Old Content";
        let mut patcher = MdPatcher::new(source);
        patcher.replace_body("# New Content");
        let updated = patcher.render();
        assert!(updated.contains("name: test"));
        assert!(updated.contains("# New Content"));
        assert!(!updated.contains("# Old Content"));
    }

    #[test]
    fn test_update_description_existing() {
        let source = "---
name: test
description: old description
---
# Content";
        let mut patcher = MdPatcher::new(source);
        patcher.update_description("new description").unwrap();
        let updated = patcher.render();
        assert!(updated.contains("description: new description"));
        assert!(updated.contains("name: test"));
        assert!(updated.contains("# Content"));
    }

    #[test]
    fn test_update_description_quoted() {
        let source = "---
description: 'old quoted description'
---";
        let mut patcher = MdPatcher::new(source);
        patcher.update_description("\"new quoted description\"").unwrap();
        let updated = patcher.render();
        assert!(updated.contains("description: \"new quoted description\""));
    }

    #[test]
    fn test_update_description_with_comments() {
        let source = "---
name: test # name comment
description: old # desc comment
# overall comment
---";
        let mut patcher = MdPatcher::new(source);
        patcher.update_description("new").unwrap();
        let updated = patcher.render();
        assert!(updated.contains("name: test # name comment"));
        assert!(updated.contains("description: new"));
        assert!(updated.contains("# overall comment"));
    }

    #[test]
    fn test_update_description_error_on_multiline_input() {
        let mut patcher = MdPatcher::new("# Content");
        let result = patcher.update_description("line1\nline2");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Multi-line description"));
    }

    #[test]
    fn test_update_description_error_on_marker_in_source() {
        let source = "---
description: |
  multi
---";
        let mut patcher = MdPatcher::new(source);
        let result = patcher.update_description("new");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("YAML marker"));
    }

    #[test]
    fn test_update_description_error_on_indentation_in_source() {
        let source = "---
description: 
  multi
---";
        let mut patcher = MdPatcher::new(source);
        let result = patcher.update_description("new");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Indentation"));
    }

    #[test]
    fn test_replace_body_preserves_frontmatter_exactly() {
        let source = "---
name: test
# comment
description: desc
---
# Old Body";
        let mut patcher = MdPatcher::new(source);
        patcher.replace_body("# New Body");
        let updated = patcher.render();
        assert!(updated.contains("name: test"));
        assert!(updated.contains("# comment"));
        assert!(updated.contains("description: desc"));
        assert!(updated.contains("# New Body"));
        assert!(updated.contains("---\n\n# New Body")); // 개행 보장 확인
    }

    #[test]
    fn test_update_description_missing() {
        let source = "---
name: test
---
# Content";
        let mut patcher = MdPatcher::new(source);
        patcher.update_description("new description").unwrap();
        let updated = patcher.render();
        assert!(updated.contains("description: new description"));
        assert!(updated.contains("name: test"));
    }

    #[test]
    fn test_update_description_no_frontmatter() {
        let source = "# Content";
        let mut patcher = MdPatcher::new(source);
        patcher.update_description("new description").unwrap();
        let updated = patcher.render();
        assert!(updated.contains("description: new description"));
        assert!(updated.contains("# Content"));
        assert!(updated.starts_with("---"));
    }

    #[test]
    fn test_patch_empty_source() {
        let source = "";
        let mut patcher = MdPatcher::new(source);
        patcher.update_description("new").unwrap();
        patcher.replace_body("# New Body");
        let updated = patcher.render();
        assert!(updated.contains("description: new"));
        assert!(updated.contains("# New Body"));
    }

    #[test]
    fn test_newline_accumulation_prevention() {
        let source = "---\nname: test\ndescription: old\n---\n\n# Body";
        let mut patcher = MdPatcher::new(source);

        // 여러 번 업데이트 수행
        patcher.update_description("new1").unwrap();
        patcher.update_description("new2").unwrap();
        patcher.replace_body("# New Body");
        patcher.update_description("new3").unwrap();

        let updated = patcher.render();

        // "---" 바로 다음에 "\n\n"이 오지 않는지 확인 (정상적이라면 "\nname")
        assert!(updated.starts_with("---\nname"));
        assert!(!updated.contains("---\n\nname"));

        // 전체적인 구조가 깨지지 않았는지 확인
        assert!(updated.contains("description: new3"));
        assert!(updated.contains("# New Body"));
    }
}
