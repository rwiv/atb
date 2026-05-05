use crate::core::{
    AGENTS_MD, BuildTarget, CLAUDE_MD, DIR_AGENTS, DIR_AGENTS_SKILLS, DIR_COMMANDS, DIR_SKILLS, GEMINI_MD, SKILL_MD,
    TransformedResource,
};
use crate::utils::fs::ensure_dir;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct Emitter {
    output_path: PathBuf,
    target: BuildTarget,
}

impl Emitter {
    pub fn new(output_path: impl Into<PathBuf>, target: BuildTarget) -> Self {
        Self {
            output_path: output_path.into(),
            target,
        }
    }

    /// commands/, agents/, skills/ 등 모든 출력 디렉터리와 전역 파일을 전부 삭제합니다.
    pub fn clean_all(&self) -> Result<()> {
        let dirs: &[&str] = if self.target == BuildTarget::Codex {
            &[DIR_COMMANDS, DIR_AGENTS]
        } else {
            &[DIR_COMMANDS, DIR_AGENTS, DIR_SKILLS]
        };
        let files = [GEMINI_MD, CLAUDE_MD, AGENTS_MD];

        for dir in dirs {
            let path = self.output_path.join(dir);
            if path.exists() {
                fs::remove_dir_all(&path).with_context(|| format!("Failed to remove directory: {:?}", path))?;
            }
        }

        if self.target == BuildTarget::Codex {
            let path = self.output_path.join(DIR_AGENTS_SKILLS);
            if path.exists() {
                fs::remove_dir_all(&path).with_context(|| format!("Failed to remove directory: {:?}", path))?;
            }
        }

        for file in files {
            let path = self.output_path.join(file);
            if path.exists() {
                fs::remove_file(&path).with_context(|| format!("Failed to remove file: {:?}", path))?;
            }
        }

        Ok(())
    }

    /// 빌드 대상 리소스에 해당하는 파일/디렉터리를 선택적으로 삭제합니다.
    /// 전역 파일(GEMINI.md, CLAUDE.md, AGENTS.md)은 항상 삭제됩니다.
    pub fn clean(&self, resources: &[TransformedResource]) -> Result<()> {
        // Step 1: 전역 파일 항상 삭제
        for global in [GEMINI_MD, CLAUDE_MD, AGENTS_MD] {
            let path = self.output_path.join(global);
            if path.exists() {
                fs::remove_file(&path).with_context(|| format!("Failed to remove file: {:?}", path))?;
            }
        }

        // Step 2: resources 순회하며 선택적 삭제
        for resource in resources {
            let Some(first_file) = resource.files.first() else {
                continue;
            };

            if first_file.path.starts_with(DIR_SKILLS) || first_file.path.ends_with(SKILL_MD) {
                // Skill 및 SKILL.md 형식 리소스는 부속 파일을 포함할 수 있으므로 디렉터리 단위로 삭제한다.
                if let Some(skill_dir) = first_file.path.parent() {
                    let full_path = self.output_path.join(skill_dir);
                    if full_path.exists() {
                        fs::remove_dir_all(&full_path)
                            .with_context(|| format!("Failed to remove directory: {:?}", full_path))?;
                    }
                }
            } else {
                // Command/Agent/기타: 개별 파일 삭제
                for file in &resource.files {
                    let full_path = self.output_path.join(&file.path);
                    if full_path.exists() {
                        fs::remove_file(&full_path)
                            .with_context(|| format!("Failed to remove file: {:?}", full_path))?;
                    }
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
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn make_command_resource(path: impl Into<PathBuf>) -> TransformedResource {
        TransformedResource {
            files: vec![TransformedFile {
                path: path.into(),
                content: String::new(),
            }],
            extras: vec![],
        }
    }

    fn make_skill_resource(skill_file_path: impl Into<PathBuf>) -> TransformedResource {
        TransformedResource {
            files: vec![TransformedFile {
                path: skill_file_path.into(),
                content: String::new(),
            }],
            extras: vec![],
        }
    }

    fn make_emitter(root: &std::path::Path) -> Emitter {
        Emitter::new(root, BuildTarget::ClaudeCode)
    }

    /// Command 파일만 선택적으로 삭제되고 다른 Command 파일은 유지됨을 확인한다.
    #[test]
    fn test_clean_removes_only_specified_command() -> Result<()> {
        let dir = tempdir()?;
        let root = dir.path();

        fs::create_dir(root.join(DIR_COMMANDS))?;
        fs::write(root.join(DIR_COMMANDS).join("foo.md"), "foo")?;
        fs::write(root.join(DIR_COMMANDS).join("bar.md"), "bar")?;

        let emitter = make_emitter(root);
        let resources = vec![make_command_resource(PathBuf::from(DIR_COMMANDS).join("foo.md"))];
        emitter.clean(&resources)?;

        assert!(!root.join(DIR_COMMANDS).join("foo.md").exists());
        assert!(root.join(DIR_COMMANDS).join("bar.md").exists());

        Ok(())
    }

    /// Skill 리소스에 해당하는 서브디렉터리 전체가 삭제됨을 확인한다.
    #[test]
    fn test_clean_removes_entire_skill_directory() -> Result<()> {
        let dir = tempdir()?;
        let root = dir.path();

        let skill_dir = root.join(DIR_SKILLS).join("my_skill");
        fs::create_dir_all(&skill_dir)?;
        fs::write(skill_dir.join("SKILL.md"), "content")?;
        fs::write(skill_dir.join("extra.py"), "script")?;

        let emitter = make_emitter(root);
        let resources = vec![make_skill_resource(PathBuf::from(DIR_SKILLS).join("my_skill/SKILL.md"))];
        emitter.clean(&resources)?;

        assert!(!skill_dir.exists());

        Ok(())
    }

    /// 빌드 대상이 아닌 출력 디렉터리 내 파일은 삭제되지 않음을 확인한다.
    #[test]
    fn test_clean_preserves_unrelated_files() -> Result<()> {
        let dir = tempdir()?;
        let root = dir.path();

        fs::write(root.join("unrelated.txt"), "keep me")?;
        fs::create_dir(root.join(DIR_COMMANDS))?;
        fs::write(root.join(DIR_COMMANDS).join("other.md"), "other")?;

        let emitter = make_emitter(root);
        // 빌드 대상 없음
        emitter.clean(&[])?;

        assert!(root.join("unrelated.txt").exists());
        assert!(root.join(DIR_COMMANDS).join("other.md").exists());

        Ok(())
    }

    /// resources에 항목이 있더라도 전역 파일(GEMINI.md 등)이 함께 항상 삭제됨을 확인한다.
    #[test]
    fn test_clean_always_removes_global_files() -> Result<()> {
        let dir = tempdir()?;
        let root = dir.path();

        fs::write(root.join(GEMINI_MD), "gemini")?;
        fs::write(root.join(CLAUDE_MD), "claude")?;
        fs::write(root.join(AGENTS_MD), "agents")?;
        fs::create_dir(root.join(DIR_COMMANDS))?;
        fs::write(root.join(DIR_COMMANDS).join("foo.md"), "foo")?;

        let emitter = make_emitter(root);
        let resources = vec![make_command_resource(PathBuf::from(DIR_COMMANDS).join("foo.md"))];
        emitter.clean(&resources)?;

        assert!(!root.join(GEMINI_MD).exists());
        assert!(!root.join(CLAUDE_MD).exists());
        assert!(!root.join(AGENTS_MD).exists());

        Ok(())
    }

    /// 빈 resources 슬라이스로 호출 시 전역 파일만 삭제되고 나머지는 유지됨을 확인한다.
    #[test]
    fn test_clean_with_empty_resources_removes_only_global_files() -> Result<()> {
        let dir = tempdir()?;
        let root = dir.path();

        fs::write(root.join(GEMINI_MD), "gemini")?;
        fs::create_dir(root.join(DIR_COMMANDS))?;
        fs::write(root.join(DIR_COMMANDS).join("foo.md"), "foo")?;

        let emitter = make_emitter(root);
        emitter.clean(&[])?;

        assert!(!root.join(GEMINI_MD).exists());
        assert!(root.join(DIR_COMMANDS).join("foo.md").exists());

        Ok(())
    }

    #[test]
    fn test_emit_with_extras() -> Result<()> {
        let dir = tempdir()?;
        let root = dir.path();

        // 임시 원본 파일 생성
        let source_file = dir.path().join("source.txt");
        fs::write(&source_file, "extra content")?;

        let emitter = make_emitter(root);
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

    /// Codex 타겟의 전체 정리에서 output-dir 외부의 공용 skill 디렉터리도 삭제됨을 확인한다.
    #[test]
    fn test_clean_all_removes_codex_agents_skills_dir() -> Result<()> {
        let dir = tempdir()?;
        let project_root = dir.path();
        let output_dir = project_root.join(".codex");
        let agents_skills_dir = project_root.join(".agents").join("skills");

        fs::create_dir_all(&output_dir)?;
        fs::create_dir_all(&agents_skills_dir)?;
        fs::write(agents_skills_dir.join("keep.md"), "remove")?;

        let emitter = Emitter::new(&output_dir, BuildTarget::Codex);
        emitter.clean_all()?;

        assert!(!agents_skills_dir.exists());

        Ok(())
    }

    /// Codex 타겟의 전체 정리에서도 .codex/skills는 보존함을 확인한다.
    #[test]
    fn test_clean_all_preserves_codex_local_skills_dir() -> Result<()> {
        let dir = tempdir()?;
        let output_dir = dir.path().join(".codex");
        let local_skills_dir = output_dir.join(DIR_SKILLS);

        fs::create_dir_all(&local_skills_dir)?;
        fs::write(local_skills_dir.join("manual.txt"), "keep")?;

        let emitter = Emitter::new(&output_dir, BuildTarget::Codex);
        emitter.clean_all()?;

        assert!(local_skills_dir.exists());
        assert!(local_skills_dir.join("manual.txt").exists());

        Ok(())
    }

    /// Codex가 아닌 타겟의 전체 정리에서는 Codex 공용 skill 디렉터리를 보존함을 확인한다.
    #[test]
    fn test_clean_all_preserves_codex_agents_skills_dir_for_non_codex_target() -> Result<()> {
        let dir = tempdir()?;
        let project_root = dir.path();
        let output_dir = project_root.join(".claude");
        let agents_skills_dir = project_root.join(".agents").join("skills");

        fs::create_dir_all(&output_dir)?;
        fs::create_dir_all(&agents_skills_dir)?;
        fs::write(agents_skills_dir.join("keep.md"), "keep")?;

        let emitter = Emitter::new(&output_dir, BuildTarget::ClaudeCode);
        emitter.clean_all()?;

        assert!(agents_skills_dir.exists());

        Ok(())
    }

    /// output-dir 외부의 SKILL.md 경로도 부모 디렉터리 단위로 삭제됨을 확인한다.
    #[test]
    fn test_clean_removes_parent_dir_for_external_skill_path() -> Result<()> {
        let dir = tempdir()?;
        let project_root = dir.path();
        let output_dir = project_root.join(".codex");
        let skill_dir = project_root.join(".agents").join("skills").join("foo");

        fs::create_dir_all(&output_dir)?;
        fs::create_dir_all(&skill_dir)?;
        fs::write(skill_dir.join(SKILL_MD), "content")?;
        fs::write(skill_dir.join("extra.txt"), "extra")?;

        let emitter = Emitter::new(&output_dir, BuildTarget::Codex);
        let resources = vec![make_skill_resource(
            PathBuf::from(DIR_AGENTS_SKILLS).join("foo").join(SKILL_MD),
        )];
        emitter.clean(&resources)?;

        assert!(!skill_dir.exists());

        Ok(())
    }
}
