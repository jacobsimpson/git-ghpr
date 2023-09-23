use std::process::ExitCode;

use crate::configuration::Commands;
use crate::result::Message;
use crate::result::Result;

mod common;
mod configuration;
mod create;
mod result;
mod verbose;

#[tokio::main]
async fn main() -> ExitCode {
    let options = configuration::load();

    verbose::init(options.verbose);

    match execute(options).await {
        Ok(m) => {
            match m {
                Message::Empty => (),
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::FAILURE
        }
    }
}

async fn execute(options: configuration::Options) -> Result<Message> {
    match options.command {
        Commands::Create { branch_prefix: _ } => {
            create::create_pull_request().await
        }
    }
}
