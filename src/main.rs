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
    let options = match configuration::load() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
    };

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

async fn execute(options: configuration::Configuration) -> Result<Message> {
    match options.command {
        Commands::Create {
            branch_name_parameters,
        } => {
            create::create_pull_request(
                &options.branch_name_template,
                &branch_name_parameters,
            )
            .await
        }
    }
}
