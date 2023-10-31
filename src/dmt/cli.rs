use super::commands::*;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

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
enum Commands {
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
    },
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

impl Cli {
    pub fn verbose(&self) -> u8 {
        self.verbose
    }
}

#[derive(Debug)]
pub enum CliError {
    NoModletPath,
    Unknown(String),
}

pub fn run() -> Result<Cli, Vec<CliError>> {
    let cli = Cli::parse();
    let mut errors: Vec<CliError> = Vec::new();

    match &cli.command {
        Commands::Bump { paths, vers } => {
            if paths.is_empty() {
                errors.push(CliError::NoModletPath);
            }

            bump::run(paths, vers);
        }
        Commands::Init { name } => {
            if name.is_empty() {
                errors.push(CliError::Unknown(String::from("No modlet name specified")));
            }

            println!("Initializing modlet {}", name);
        }
    };

    if errors.len() > 0 {
        Err(errors)
    } else {
        Ok(cli)
    }
}

mod tests {
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        super::Cli::command().debug_assert()
    }
}
