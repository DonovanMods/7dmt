use color_eyre::eyre::{eyre, Result};
use console::{pad_str_with, style, Alignment, Term};
use rayon::prelude::*;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

pub fn verified_paths(paths: &Vec<PathBuf>) -> Result<Vec<PathBuf>> {
    let mut sanitized_paths = Vec::new();

    for path in paths {
        let path = path.canonicalize()?;

        if !path.exists() {
            return Err(eyre!("Path does not exist: {:?}", path));
        }

        if path.is_dir() && path.join("ModInfo.xml").exists() {
            sanitized_paths.push(path);
        }
    }

    if sanitized_paths.is_empty() {
        return Err(eyre!("No valid modlets found in {}", paths[0].display()));
    }

    Ok(sanitized_paths)
}

pub fn validate(path: impl AsRef<Path>, padding: usize, outbuf: &Term) -> Result<()> {
    let file_name = path.as_ref().file_name().unwrap_or(OsStr::new("")).to_str().unwrap();

    // print!("Validating {}{:padding$} ", path.as_ref().file_name().unwrap_or(OsStr::new("")).to_str().unwrap(), "...");

    outbuf.write_line(
        format!(
            "Validating {} ",
            pad_str_with(file_name, padding + 3, Alignment::Left, None, '.')
        )
        .as_ref(),
    )?; // .pad_to_width_with_char(padding + 3, '.'));

    thread::sleep(Duration::from_millis(1000));
    Ok(())
}

pub fn run(dirty_paths: &Vec<PathBuf>) -> Result<()> {
    let verified_paths = verified_paths(dirty_paths)?;
    let outbuf = Term::stdout();
    let padding = verified_paths
        .iter()
        .map(|p| p.as_path().file_name().unwrap().len())
        .max()
        .unwrap_or(0);

    // Using `par_iter()` to parallelize the validation of each modlet.
    verified_paths
        .par_iter()
        .for_each(|path| match validate(path, padding, &outbuf) {
            Ok(_) => outbuf
                .write_line(format!("{}", style("OK").green().bold()).as_ref())
                .unwrap(),
            Err(err) => outbuf
                .write_line(format!("{}", style(err).red().bold()).as_ref())
                .unwrap(),
        });

    Ok(())
}
