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
use std::collections::HashMap;
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
    pub command: CmdCommands,
}

#[derive(Subcommand, Serialize, Deserialize, Debug)]
pub enum CmdCommands {
    Create {
        #[arg(short, long)]
        jira: Option<String>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
struct FileOptions {
    branch_name_template: Option<String>,
}

#[derive(Debug)]
pub struct Configuration {
    pub branch_name_template: String,

    pub verbose: u8,

    pub command: Commands,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Commands {
    Create {
        branch_name_parameters: HashMap<String, String>,
    },
}

impl From<CmdCommands> for Commands {
    fn from(c: CmdCommands) -> Self {
        match c {
            CmdCommands::Create { jira } => match jira {
                Some(v) => Self::Create {
                    branch_name_parameters: HashMap::from([(
                        "jira".to_string(),
                        v,
                    )]),
                },
                None => Self::Create {
                    branch_name_parameters: HashMap::new(),
                },
            },
        }
    }
}

fn first_of(
    one: Option<String>,
    two: Option<String>,
    three: Option<String>,
    name: &str,
) -> Result<String> {
    if let Some(v) = one {
        return Ok(v);
    }
    if let Some(v) = two {
        return Ok(v);
    }
    if let Some(v) = three {
        return Ok(v);
    }
    Err(Error::BadParameter(name.to_string()))
}

fn merge(
    file_options: FileOptions,
    cmd_options: CmdOptions,
) -> Result<Configuration> {
    Ok(Configuration {
        branch_name_template: first_of(
            cmd_options.branch_name_template,
            file_options.branch_name_template,
            Some("{{summary}}".to_string()),
            "branch_name_template",
        )?,
        verbose: cmd_options.verbose,
        command: Commands::from(cmd_options.command),
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
