pub mod dependency;
pub mod emitter;

use self::dependency::DependencyChecker;
use self::emitter::Emitter;
use crate::core::{AGENTS_MD, TransformedResource};
use crate::loader::registry::Registry;
use crate::transformer::Transformer;
use crate::utils::yaml::extract_frontmatter;
use anyhow::Context;
use log::info;
use std::path::Path;

#[derive(Default)]
pub struct Builder;

impl Builder {
    pub fn new() -> Self {
        Self
    }

    pub fn run(
        &self,
        transformer: &dyn Transformer,
        registry: &Registry,
        source_dir: &Path,
        output_dir: &Path,
    ) -> anyhow::Result<()> {
        // 의존성 검사
        let checker = DependencyChecker::new();
        checker.check_dependencies(registry, source_dir)?;

        let all_resources = registry.all_resources();
        let mut transformed_resources = Vec::new();
        for res in &all_resources {
            let transformed_file = transformer
                .transform(res)
                .with_context(|| format!("Failed to transform resource: {}", res.name()))?;

            transformed_resources.push(TransformedResource {
                files: vec![transformed_file],
                extras: res.extras(),
            });
        }

        // Post-transform hook
        let post_files = transformer.post_transform(&all_resources)?;
        if !post_files.is_empty() {
            transformed_resources.push(TransformedResource {
                files: post_files,
                extras: Vec::new(),
            });
        }

        // AGENTS.md 처리
        let agents_md_path = source_dir.join(AGENTS_MD);
        if agents_md_path.exists() {
            info!("  - Found root system prompt: {}", agents_md_path.display());
            let raw_content = std::fs::read_to_string(&agents_md_path)?;
            let (_fm, pure_content) = extract_frontmatter(&raw_content);
            let transformed_file = transformer.transform_root_prompt(&pure_content)?;

            transformed_resources.push(TransformedResource {
                files: vec![transformed_file],
                extras: Vec::new(),
            });
        }

        info!("Emitting files to {}...", output_dir.display());
        let emitter = Emitter::new(output_dir);
        emitter.clean()?;
        emitter.emit(&transformed_resources)?;

        info!("Build successful!");

        Ok(())
    }
}
