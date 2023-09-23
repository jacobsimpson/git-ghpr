use anyhow::Result;
use clap::Parser;

mod configuration;

#[tokio::main]
async fn main() -> Result<()> {
    let _options = configuration::Options::parse();
    Ok(())
}
