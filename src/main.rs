use color_eyre::eyre::Result;
use console::{style, Term};
use dmt::cli;
use std::process::exit;

mod dmt;

#[derive(Default, Debug)]
pub struct CommandResult {
    errors: Vec<cli::CliError>,
    messages: Vec<String>,
    verbose: u8,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let stdout = Term::stdout();
    let stderr = Term::stderr();
    let result = cli::run()?;

    // dbg!(&result);

    if result.errors.is_empty() {
        if result.verbose >= 1 {
            for message in result.messages {
                stdout.write_line(&message)?;
            }
        }
        exit(0)
    } else {
        // Err(result.errors.map(|e| e.into()));
        for error in result.errors {
            stderr.write_line(format!("{}", style(&error).red().bold()).as_ref())?;
        }
        exit(1)
    }
}
