use clap::{ArgAction, Parser, Subcommand};

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = r#"
An extension of git-branchless to make it easier to turn small commits into
Github PRs that are stacked on each other."#
)]
pub struct Options {
    #[arg(short, long, action = ArgAction::Count, help = r#"Increase the debugging output of the command. Accepted multiple times
for more information."#)]
    pub verbose: u8,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Create {
        #[arg(short, long)]
        branch_prefix: Option<String>,
    },
}

pub fn load() -> Options {
    Options::parse()
}
