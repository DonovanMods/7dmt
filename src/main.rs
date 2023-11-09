mod dmt;
use dmt::cli;
use std::process::exit;

#[derive(Default, Debug)]
pub struct CommandResult {
    errors: Vec<cli::CliError>,
    messages: Vec<String>,
    verbose: u8,
}

fn main() {
    let result = cli::run();

    dbg!(&result);

    if result.errors.is_empty() {
        if result.verbose >= 1 {
            for message in result.messages {
                println!("{}", message);
            }
        }
        exit(0)
    } else {
        for error in result.errors {
            match error {
                cli::CliError::NoModletPath => eprintln!("No modlet path specified"),
                cli::CliError::InvalidArg(msg) => eprintln!("Invalid argument: {}", msg),
                cli::CliError::Unknown(msg) => eprintln!("Unknown error: {}", msg),
            }
        }
        exit(1)
    }
}
