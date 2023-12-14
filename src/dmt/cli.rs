use super::commands;
use crate::dmt::helpers::verify_modlet_paths;
use crate::CommandResult;
use clap::{Args, Parser, Subcommand};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{fmt, path::PathBuf, sync::RwLock};
use thiserror::Error;

#[derive(Debug, Parser)]
#[command(about, author, version, long_about = None)]
pub struct Cli {
    /// Specify a custom config file
    #[arg(short, long, global = true, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Verbose mode (may be repeated for increased verbosity)
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    verbose: u8,

    #[arg(short, long, global = true, value_name = "PATH")]
    game_directory: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Bump the version of a modlet
    #[command(arg_required_else_help = true)]
    Bump {
        /// The modlet path to operate on
        paths: Vec<PathBuf>,

        #[command(flatten)]
        /// The version to set
        vers: Vers,
    },
    /// Convert a ModInfo.xml from V1 to V2 (or vice versa)
    #[command(arg_required_else_help = true)]
    Convert {
        /// The modlet path(s) to operate on
        paths: Vec<PathBuf>,

        /// [Optionally] the ModInfo version to convert to (default: V2)
        #[command(flatten)]
        requested_version: Option<RequestedVersion>,
    },
    /// Initialize a new modlet
    #[command(arg_required_else_help = true)]
    Init {
        /// The name of the modlet to create
        name: String,

        /// [Optionally] the ModInfo version to use (default: V2)
        #[command(flatten)]
        requested_version: Option<RequestedVersion>,
    },
    // Future: We'll process instructions in special `dmt` xml sections to create
    // larger modlets -- ala lessgrind.
    /// Package Modlet(s)
    #[command(arg_required_else_help = true)]
    Package {
        /// The modlet to package into
        #[arg(short, long, value_name = "PATH")]
        output: PathBuf,

        /// The modlet path(s) to operate on
        modlets: Vec<PathBuf>,
    },
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Commands::Bump { .. } => write!(f, "Bump"),
            Commands::Convert { .. } => write!(f, "Convert"),
            Commands::Init { .. } => write!(f, "Init"),
            Commands::Package { .. } => write!(f, "Package"),
        }
    }
}

#[derive(Args, Debug)]
#[group(required = true, multiple = true)]
pub struct Vers {
    /// set version manually
    #[arg(long, value_name = "VER")]
    ver: Option<String>,

    /// auto inc major
    #[arg(long)]
    major: bool,

    /// auto inc minor
    #[arg(long)]
    minor: bool,

    /// auto inc patch
    #[arg(long)]
    patch: bool,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
pub struct RequestedVersion {
    /// Use ModInfo.xml V1 (old) Version
    #[arg(long, value_name = "V1")]
    pub v1: bool,
    /// Use ModInfo.xml V2 Version (default)
    #[arg(long, value_name = "V2")]
    pub v2: bool,
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Config {
    #[serde(default)]
    pub game_directory: Option<PathBuf>,
    pub verbosity: u8,
}

lazy_static! {
    pub static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
}

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Invalid argument: {0}")]
    InvalidArg(String),
    #[error("No game directory specified")]
    NoGameDirectory,
    #[error("No modlet path specified")]
    NoModletPath,
    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub fn run() -> eyre::Result<CommandResult> {
    let cli = Cli::parse();
    let mut result = CommandResult::default();

    SETTINGS.write().unwrap().game_directory = cli.game_directory;
    SETTINGS.write().unwrap().verbosity = cli.verbose;

    match &cli.command {
        Commands::Bump { paths, vers } => {
            if paths.is_empty() {
                result.errors.push(CliError::NoModletPath);
            } else {
                let mut opts: Vec<commands::bump::BumpOptions> = Vec::new();

                opts.push(commands::bump::BumpOptions::Verbosity(cli.verbose));

                if let Some(ver) = &vers.ver {
                    opts.push(commands::bump::BumpOptions::Set(ver.clone()));
                } else {
                    if vers.major {
                        opts.push(commands::bump::BumpOptions::Major);
                    }
                    if vers.minor {
                        opts.push(commands::bump::BumpOptions::Minor);
                    }
                    if vers.patch {
                        opts.push(commands::bump::BumpOptions::Patch);
                    }
                }

                for path in paths {
                    match commands::bump::run(path.clone(), opts.clone()) {
                        Ok(msg) => result.messages.push(msg),
                        Err(err) => result.errors.push(CliError::InvalidArg(err)),
                    }
                }
            }
        }
        Commands::Convert {
            paths,
            requested_version,
        } => {
            if paths.is_empty() {
                result.errors.push(CliError::NoModletPath);
            } else {
                for path in paths {
                    match commands::convert::run(path, requested_version.as_ref()) {
                        Ok(_) => result
                            .messages
                            .push(format!("Successfully converted {}", path.display())),
                        Err(err) => result.errors.push(CliError::InvalidArg(err.to_string())),
                    }
                }
            }
        }
        Commands::Init {
            name,
            requested_version,
        } => {
            if name.is_empty() {
                result
                    .errors
                    .push(CliError::Unknown(String::from("No modlet name specified")));
            } else {
                match commands::init::run(name.clone(), requested_version.as_ref()) {
                    Ok(true) => result.messages.push(format!("Created Modlet {}", name)),
                    Ok(false) => result.messages.push("Cancelled".to_owned()),
                    Err(err) => result.errors.push(CliError::Unknown(err.to_string())),
                }
            }
        }
        Commands::Package { modlets, output } => {
            if SETTINGS.read().unwrap().game_directory.is_none() {
                result.errors.push(CliError::NoGameDirectory);
            } else if modlets.is_empty() {
                result.errors.push(CliError::NoModletPath);
            } else {
                let verified_paths = verify_modlet_paths(modlets)?;
                commands::package::run(&verified_paths, output)?
            }
        }
    };

    Ok(result)
}

mod tests {
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        super::Cli::command().debug_assert()
    }
}
