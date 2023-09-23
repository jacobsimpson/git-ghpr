use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = r#"
This utility is an extension of git-branchless to make it easier to turn small
commits into Github PRs that are stacked on each other."#
)]
pub struct Options {
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
