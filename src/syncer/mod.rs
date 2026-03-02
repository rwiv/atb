pub mod extra;
pub mod patcher;

use crate::core::Resource;
use crate::syncer::extra::ExtraSyncer;
use crate::syncer::patcher::MdPatcher;
use crate::transformer::Transformer;
use anyhow::{Context, Result};
use glob::Pattern;
use log::info;
use std::fs;
use std::path::Path;

pub struct Syncer {
    extra: ExtraSyncer,
}

impl Syncer {
    pub fn new(exclude_patterns: Vec<Pattern>) -> Self {
        Self {
            extra: ExtraSyncer::new(exclude_patterns),
        }
    }

    /// 단일 리소스를 타겟에서 소스로 동기화합니다.
    pub fn sync_resource(&self, resource: &Resource, transformer: &dyn Transformer, output_dir: &Path) -> Result<()> {
        // 타겟 파일 경로 결정 (get_target_path 사용으로 최적화)
        let relative_target_path = transformer.get_target_path(resource.r_type(), resource.name());
        let target_path = output_dir.join(&relative_target_path);

        if !target_path.exists() {
            return Ok(()); // 타겟 파일이 없으면 변경사항도 없는 것으로 간주
        }

        info!("  Checking resource: {}/{}", resource.r_type(), resource.name());

        // 타겟 파일 내용 읽기
        let target_content = fs::read_to_string(&target_path)
            .with_context(|| format!("Failed to read target file: {:?}", target_path))?;

        // 역변환 (Detransform)
        let detransformed = transformer
            .detransform(resource.r_type(), resource.name(), &target_content, output_dir)
            .with_context(|| format!("Failed to detransform target file: {:?}", target_path))?;

        // 소스 정보 가져오기
        let source_path = resource.main_source_path();
        let source_file_content = fs::read_to_string(&source_path)
            .with_context(|| format!("Failed to read source file: {:?}", source_path))?;
        let current_metadata = resource.metadata();

        let mut patcher = MdPatcher::new(&source_file_content);
        let mut changed = false;

        // 1. Description 동기화
        let old_desc = current_metadata["description"].as_str().unwrap_or_default();
        let new_desc = detransformed.metadata["description"].as_str().unwrap_or_default();

        if old_desc != new_desc {
            patcher.update_description(new_desc)?;
            changed = true;
            info!("    - Updated description in source");
        }

        // 2. Content 동기화
        if patcher.has_changed(&detransformed.content) {
            patcher.replace_body(&detransformed.content);
            changed = true;
            info!("    - Updated content in source");
        }

        // 소스 파일 쓰기
        if changed {
            fs::write(&source_path, patcher.render())?;
        }

        // 3. Skill ExtraFiles 동기화
        if let Resource::Skill(s) = resource {
            let target_skill_dir = target_path
                .parent()
                .ok_or_else(|| anyhow::anyhow!("Failed to get parent directory of {:?}", target_path))?;
            self.extra.sync(&s.base.source_path, target_skill_dir)?;
        }

        Ok(())
    }
}
