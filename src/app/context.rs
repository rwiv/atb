use crate::app::{Config, load_config};
use crate::core::{CONFIG_FILE_NAME, ResourceType};
use crate::loader;
use crate::loader::registry::Registry as LoaderRegistry;
use crate::transformer::Transformer;
use crate::transformer::TransformerFactory;
use anyhow::Context;
use glob::Pattern;
use log::info;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub struct AppContext {
    pub config: Config,
    pub registry: LoaderRegistry,
    pub transformer: Box<dyn Transformer>,
    pub source_dir: PathBuf,
    pub output_dir: PathBuf,
    pub exclude_patterns: Vec<Pattern>,
}

impl AppContext {
    pub fn init(config_file: &str) -> anyhow::Result<Self> {
        let config_path = Path::new(config_file);
        if !config_path.exists() {
            anyhow::bail!("Config file not found: {}", config_file);
        }
        let output_dir = config_path.parent().unwrap_or(Path::new(".")).to_path_buf();

        info!("Loading config: {}", config_file);
        let cfg = load_config(config_file)?;
        let source_dir = PathBuf::from(&cfg.source);

        if !source_dir.exists() {
            anyhow::bail!("Source directory does not exist: {}", source_dir.display());
        }

        // 설정으로부터 식별자 추출 (타입 포함)
        let mut target_identifiers = HashSet::new();
        if let Some(cmds) = &cfg.resources.commands {
            for cmd in cmds {
                target_identifiers.insert((ResourceType::Command, cmd.clone()));
            }
        }
        if let Some(agents) = &cfg.resources.agents {
            for agent in agents {
                target_identifiers.insert((ResourceType::Agent, agent.clone()));
            }
        }
        if let Some(skills) = &cfg.resources.skills {
            for skill in skills {
                target_identifiers.insert((ResourceType::Skill, skill.clone()));
            }
        }

        let exclude_strings = cfg.exclude.as_ref().cloned().unwrap_or_default();
        let mut exclude_patterns = Vec::new();
        for p in &exclude_strings {
            exclude_patterns.push(Pattern::new(p).with_context(|| format!("Invalid glob pattern: {}", p))?);
        }

        // ResourceLoader를 통한 리소스 로드 및 필터링
        info!("Scanning and loading resources from {}...", source_dir.display());
        let loader = loader::ResourceLoader::new(&source_dir, exclude_patterns.clone(), cfg.target)?;

        info!("Validating and registering resources...");
        let all_resources = loader.load()?;
        let mut registry = LoaderRegistry::new();
        let mut found_identifiers = HashSet::new();

        for resource in all_resources {
            let identifier = format!("{}:{}", resource.plugin(), resource.name());
            let key = (resource.r_type(), identifier);
            if target_identifiers.contains(&key) {
                found_identifiers.insert(key.clone());
                registry.register(resource)?;
            }
        }

        // 누락된 리소스 확인
        let missing: Vec<_> = target_identifiers.difference(&found_identifiers).collect();

        if !missing.is_empty() {
            let mut msg = format!("Missing resources specified in {}:\n", CONFIG_FILE_NAME);
            for (r_type, id) in missing {
                msg.push_str(&format!("  - {}: '{}' (Not found)\n", r_type, id));
            }
            anyhow::bail!(msg);
        }

        let transformer = TransformerFactory::create(&cfg.target);

        Ok(Self {
            config: cfg,
            registry,
            transformer,
            source_dir,
            output_dir,
            exclude_patterns,
        })
    }
}
