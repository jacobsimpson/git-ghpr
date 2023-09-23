use crate::configuration::Commands;
use anyhow::Result;
use clap::Parser;

mod configuration;
mod create;

#[tokio::main]
async fn main() -> Result<()> {
    let options = configuration::Options::parse();

    match options.command {
        Commands::Create { branch_prefix: _ } => create::create_pull_request().await,
    }
}
