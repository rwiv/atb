pub mod cli;
pub mod config;
pub mod context;

pub use cli::{Cli, Commands};
pub use config::*;
pub use context::AppContext;

use crate::builder::Builder;
use crate::core::CONFIG_FILE_NAME;
use crate::syncer::Syncer;
use log::info;

pub struct App;

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        App
    }

    pub fn run(&self, cli: Cli) -> anyhow::Result<()> {
        let config_file = match &cli.command {
            Commands::Build { config } => config.as_deref().unwrap_or(CONFIG_FILE_NAME),
            Commands::Sync { config } => config.as_deref().unwrap_or(CONFIG_FILE_NAME),
        };

        let ctx = AppContext::init(config_file)?;

        match &cli.command {
            Commands::Build { .. } => self.build(&ctx),
            Commands::Sync { .. } => self.sync(&ctx),
        }
    }

    fn build(&self, ctx: &AppContext) -> anyhow::Result<()> {
        let builder = Builder::new();

        info!("Transforming resources for target: {:?}...", ctx.config.target);

        builder.run(
            ctx.transformer.as_ref(),
            &ctx.registry,
            &ctx.source_dir,
            &ctx.output_dir,
        )?;
        info!("  - Target: {:?}", ctx.config.target);
        info!("  - Resources: {} total", ctx.registry.len());
        Ok(())
    }

    fn sync(&self, ctx: &AppContext) -> anyhow::Result<()> {
        info!(
            "Syncing target changes to source for target: {:?}...",
            ctx.config.target
        );

        let syncer = Syncer::new(ctx.config.target, ctx.exclude_patterns.clone());
        for res in ctx.registry.all_resources() {
            syncer.sync_resource(res, ctx.transformer.as_ref(), &ctx.output_dir)?;
        }

        info!("Sync successful!");
        Ok(())
    }
}
