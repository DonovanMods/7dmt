mod dmt;
use dmt::cli;
use std::process::exit;

fn main() {
    let result = cli::run();

    dbg!(&result);
    match result {
        Ok(cli) => {
            if cli.verbose() >= 3 {
                println!("Success");
            }
            exit(0)
        }
        Err(errors) => {
            for error in errors {
                match error {
                    cli::CliError::NoModletPath => println!("No modlet path specified"),
                    cli::CliError::Unknown(msg) => println!("Unknown error: {}", msg),
                }
            }
            exit(1)
        }
    }
}
