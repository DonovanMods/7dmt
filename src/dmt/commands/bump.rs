use super::super::cli::Vers;
use std::path::PathBuf;

pub fn run(paths: &Vec<PathBuf>, vers: &Vers) -> Result<(), String> {
    println!("Bumping version to {:?}", vers);
    println!("Bumping modlet(s) at {:?}", paths);

    Ok(())
}
