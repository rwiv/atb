use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "atb")]
#[command(about = "Agent ToolKit Builder: Multi-agent workflow resource manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Build the agent resources based on atb.yaml
    Build {
        /// Optional path to the config file
        #[arg(short, long)]
        config: Option<String>,
    },
    /// Sync changes from target back to source
    Sync {
        /// Optional path to the config file
        #[arg(short, long)]
        config: Option<String>,
    },
}
