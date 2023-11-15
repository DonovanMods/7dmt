use super::super::CommandResult;
use super::commands;
use clap::{Args, Parser, Subcommand};
use std::{fmt, path::PathBuf};

#[derive(Debug, Parser)]
#[command(about, author, version, long_about = None)]
pub struct Cli {
    #[arg(short, long, global = true, value_name = "FILE")]
    /// Specify a custom config file
    config: Option<PathBuf>,

    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    /// Verbose mode (may be repeated for increased verbosity)
    verbose: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    /// Bump the version of a modlet
    Bump {
        /// The modlet path to operate on
        paths: Vec<PathBuf>,

        #[command(flatten)]
        /// The version to set
        vers: Vers,
    },
    /// Initialize a new modlet
    Init {
        /// The name of the modlet to create
        name: String,

        #[command(flatten)]
        requested_version: Option<RequestedVersion>,
    },
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Commands::Bump { .. } => write!(f, "Bump"),
            Commands::Init { .. } => write!(f, "Init"),
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

#[derive(Debug)]
pub enum CliError {
    NoModletPath,
    InvalidArg(String),
    Unknown(String),
}

pub fn run() -> CommandResult {
    let cli = Cli::parse();
    let mut result = CommandResult {
        verbose: cli.verbose,
        ..Default::default()
    };

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
        Commands::Init {
            name,
            requested_version,
        } => {
            if name.is_empty() {
                result
                    .errors
                    .push(CliError::Unknown(String::from("No modlet name specified")));
            } else {
                match commands::init::run(name.clone(), requested_version) {
                    Ok(true) => result.messages.push(format!("Created Modlet {}", name)),
                    Ok(false) => result.messages.push("Cancelled".to_owned()),
                    Err(err) => result.errors.push(CliError::Unknown(err.to_string())),
                }
            }
        }
    };

    result
}

mod tests {
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        super::Cli::command().debug_assert()
    }
}
