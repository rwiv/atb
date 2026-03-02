use crate::core::FORBIDDEN_FILES;
use anyhow::Result;
use glob::Pattern;
use std::path::Path;

/// 스캔 및 동기화 시 파일을 필터링하는 객체입니다.
#[derive(Debug, Default)]
pub struct FileFilter;

impl FileFilter {
    /// 새로운 FileFilter 인스턴스를 생성합니다.
    pub fn new() -> Self {
        Self
    }

    /// 파일이 필터링을 통과하여 유효한지 확인합니다.
    pub fn is_valid(&self, root: &Path, path: &Path, exclude_patterns: &[Pattern]) -> Result<bool> {
        if !path.is_file() {
            return Ok(false);
        }

        let file_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid file name: {:?}", path))?;

        // 1. 숨김 파일 체크
        if file_name.starts_with('.') {
            return Ok(false);
        }

        // 2. 플러그인 내부 금지된 파일 체크
        if FORBIDDEN_FILES.contains(&file_name) {
            anyhow::bail!("Forbidden file '{}' found in plugin: {:?}", file_name, path);
        }

        // 3. 제외 패턴 체크
        let relative_path = path.strip_prefix(root).unwrap_or(path);
        for pattern in exclude_patterns {
            if pattern.matches_path(relative_path) {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn compile_patterns(patterns: &[&str]) -> Vec<Pattern> {
        patterns.iter().map(|p| Pattern::new(p).unwrap()).collect()
    }

    #[test]
    fn test_file_filter_is_valid() -> Result<()> {
        let dir = tempdir()?;
        let root = dir.path();

        let filter = FileFilter::new();
        let patterns = compile_patterns(&["*.tmp", "ignore/"]);

        // 유효한 파일
        let valid_file = root.join("foo.md");
        fs::write(&valid_file, "content")?;
        assert!(filter.is_valid(root, &valid_file, &patterns)?);

        // 제외 패턴 (*.tmp)
        let tmp_file = root.join("test.tmp");
        fs::write(&tmp_file, "content")?;
        assert!(!filter.is_valid(root, &tmp_file, &patterns)?);

        // 숨김 파일
        let hidden_file = root.join(".git");
        fs::write(&hidden_file, "content")?;
        assert!(!filter.is_valid(root, &hidden_file, &patterns)?);

        Ok(())
    }

    #[test]
    fn test_forbidden_files_error() -> Result<()> {
        let dir = tempdir()?;
        let root = dir.path();
        let filter = FileFilter::new();

        for &f in FORBIDDEN_FILES {
            let path = root.join(f);
            fs::write(&path, "content")?;
            let result = filter.is_valid(root, &path, &[]);
            assert!(result.is_err());
            assert!(
                result
                    .unwrap_err()
                    .to_string()
                    .contains(&format!("Forbidden file '{}'", f))
            );
        }

        Ok(())
    }
}
