//! Handles loading and merging configuration from all the different sources
//! that are valid.
//!
//! In order of precedence (highest to lowest):
//! - command line args
//!     - verbose and command are only valid here
//!     - branch name template parameters
//! - environment variables
//! - Github client file
//! - git configuration
//!     - github-pull-request configuration
//!     - branchless configuration
//! - dot file in the user's home directory
//! - config file in the XDG configuration directory
//!
//! - ssh key file location.
//! - Github client key
//! - branch name template
//! - name of mainline branch
//!
//! I think there is going to be 3, very similar structures.
//! - command line parser, with nearly everything optional
//! - file parser, with everything optional
//! - exported structure, with all the mandatory pieces mandatory
//!
use clap::{ArgAction, Parser, Subcommand};
use directories::{BaseDirs, ProjectDirs};
use figment::providers::{Env, Format, Toml};
use figment::Figment;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::result::Error;
use crate::result::Result;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = r#"
An extension of git-branchless to make it easier to turn small commits into
Github PRs that are stacked on each other."#
)]
struct CmdOptions {
    #[arg(short, long, action = ArgAction::Count, help = r#"Increase the debugging output of the command. Accepted multiple times
for more information."#)]
    pub verbose: u8,

    #[arg(
        short,
        long,
        help = r#"Template for naming branches that are created for pull requests."#
    )]
    pub branch_name_template: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Serialize, Deserialize, Debug)]
pub enum Commands {
    Create {
        #[arg(short, long)]
        branch_prefix: Option<String>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
struct FileOptions {
    branch_name_template: Option<String>,
}

#[derive(Debug)]
pub struct Configuration {
    branch_name_template: String,

    pub verbose: u8,

    pub command: Commands,
}

fn or(one: Option<String>, two: Option<String>, name: &str) -> Result<String> {
    if one.is_some() {
        return Ok(one.unwrap());
    }
    if two.is_some() {
        return Ok(two.unwrap());
    }
    Err(Error::BadParameter(name.to_string()))
}

fn merge(
    file_options: FileOptions,
    cmd_options: CmdOptions,
) -> Result<Configuration> {
    Ok(Configuration {
        branch_name_template: or(
            cmd_options.branch_name_template,
            file_options.branch_name_template,
            "branch_name_template",
        )?,
        verbose: cmd_options.verbose,
        command: cmd_options.command,
    })
}

const CONFIG_FILE: &str = "gh-pull-request.toml";

pub fn load() -> Result<Configuration> {
    let mut f = Figment::new();

    if let Some(pd) =
        ProjectDirs::from("org", "git tools", "github-pull-request")
    {
        let mut p = pd.config_dir().to_path_buf();
        p.push(CONFIG_FILE);
        f = f.merge(Toml::file(p));
    } else {
        error!("Could not get the project directories for this OS.");
        if let Some(bd) = BaseDirs::new() {
            let mut p = bd.config_dir().to_path_buf();
            p.push("github-pull-request");
            p.push(CONFIG_FILE);
            f = f.merge(Toml::file(p));
        } else {
            error!("Could not get the configuration directory for this OS.");
        }
    }

    if let Some(bd) = BaseDirs::new() {
        let mut p = bd.home_dir().to_path_buf();
        p.push(format!(".{}", CONFIG_FILE));
        f = f.merge(Toml::file(p));
    } else {
        error!("Could not get the user home directory for this OS.");
    }

    let file_options: FileOptions =
        f.merge(Env::prefixed("GH_PR_")).extract()?;
    let cmd_options = CmdOptions::parse();

    merge(file_options, cmd_options)
}
