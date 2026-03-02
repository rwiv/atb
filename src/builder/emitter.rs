use crate::core::{
    AGENTS_MD, CLAUDE_MD, DIR_AGENTS, DIR_CODEX, DIR_COMMANDS, DIR_PROMPTS, DIR_SKILLS, GEMINI_MD, TransformedResource,
};
use crate::utils::fs::ensure_dir;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct Emitter {
    output_path: PathBuf,
}

impl Emitter {
    pub fn new(output_path: impl Into<PathBuf>) -> Self {
        Self {
            output_path: output_path.into(),
        }
    }

    /// 기존에 생성된 디렉터리 및 메인 메모리 파일을 삭제합니다.
    pub fn clean(&self) -> Result<()> {
        let targets = [
            // directories
            DIR_CODEX,
            DIR_COMMANDS,
            DIR_PROMPTS,
            DIR_AGENTS,
            DIR_SKILLS,
            // main memory files
            GEMINI_MD,
            CLAUDE_MD,
            AGENTS_MD,
        ];

        for target in targets {
            let path = self.output_path.join(target);
            if path.exists() {
                if path.is_dir() {
                    fs::remove_dir_all(&path).with_context(|| format!("Failed to remove directory: {:?}", path))?;
                } else {
                    fs::remove_file(&path).with_context(|| format!("Failed to remove file: {:?}", path))?;
                }
            }
        }
        Ok(())
    }

    /// 변환된 리소스들을 파일 시스템에 기록합니다.
    pub fn emit(&self, resources: &[TransformedResource]) -> Result<()> {
        for resource in resources {
            // 1. 변환된 텍스트 파일 쓰기
            for file in &resource.files {
                let full_path = self.output_path.join(&file.path);
                ensure_dir(&full_path)?;
                fs::write(&full_path, &file.content)
                    .with_context(|| format!("Failed to write file: {:?}", full_path))?;
            }

            // 2. 추가 파일 복사
            for extra in &resource.extras {
                let full_target_path = self.output_path.join(&extra.target);
                ensure_dir(&full_target_path)?;
                fs::copy(&extra.source, &full_target_path).with_context(|| {
                    format!(
                        "Failed to copy extra file: {:?} -> {:?}",
                        extra.source, full_target_path
                    )
                })?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{ExtraFile, TransformedFile};
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_clean() -> Result<()> {
        let dir = tempdir()?;
        let root = dir.path();

        // 더미 파일/폴더 생성
        fs::create_dir(root.join(DIR_COMMANDS))?;
        fs::create_dir(root.join(DIR_CODEX))?;
        fs::write(root.join(DIR_COMMANDS).join("foo.toml"), "test")?;
        fs::write(root.join(GEMINI_MD), "test")?;
        fs::write(root.join("other.txt"), "keep me")?;

        let emitter = Emitter::new(root);
        emitter.clean()?;

        assert!(!root.join(DIR_COMMANDS).exists());
        assert!(!root.join(DIR_CODEX).exists());
        assert!(!root.join(GEMINI_MD).exists());
        assert!(root.join("other.txt").exists());

        Ok(())
    }

    #[test]
    fn test_emit_with_extras() -> Result<()> {
        let dir = tempdir()?;
        let root = dir.path();

        // 임시 원본 파일 생성
        let source_file = dir.path().join("source.txt");
        fs::write(&source_file, "extra content")?;

        let emitter = Emitter::new(root);
        let resources = vec![TransformedResource {
            files: vec![TransformedFile {
                path: PathBuf::from(DIR_SKILLS).join("my_skill/SKILL.md"),
                content: "content".to_string(),
            }],
            extras: vec![ExtraFile {
                source: source_file,
                target: PathBuf::from(DIR_SKILLS).join("my_skill/extra.txt"),
            }],
        }];

        emitter.emit(&resources)?;

        assert_eq!(
            fs::read_to_string(root.join(DIR_SKILLS).join("my_skill/SKILL.md"))?,
            "content"
        );
        assert_eq!(
            fs::read_to_string(root.join(DIR_SKILLS).join("my_skill/extra.txt"))?,
            "extra content"
        );

        Ok(())
    }
}
