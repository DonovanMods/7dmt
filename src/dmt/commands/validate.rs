use color_eyre::eyre::{eyre, Result};
use console::{pad_str_with, style, Alignment};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
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

pub fn validate(path: impl AsRef<Path>, padding: usize, pb: &ProgressBar, verbosity: u8) -> Result<()> {
    let file_name = path.as_ref().file_name().unwrap_or(OsStr::new("")).to_str().unwrap();
    if verbosity > 0 {
        pb.set_prefix(format!(
            "Validating {} ",
            pad_str_with(file_name, padding + 3, Alignment::Left, None, '.')
        ));
    }

    // TODO: Actually validate the modlet.
    for _ in 0..100 {
        if verbosity > 0 {
            pb.inc(1);
        }
        thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}

pub fn run(dirty_paths: &Vec<PathBuf>, verbosity: u8) -> Result<()> {
    let verified_paths = verified_paths(dirty_paths)?;
    let count = verified_paths.len() as u64;
    let mp = MultiProgress::new();
    let spinner_style = ProgressStyle::with_template("{prefix:.cyan.bright} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    let padding = verified_paths
        .iter()
        .map(|p| p.as_path().file_name().unwrap().len())
        .max()
        .unwrap_or(0);

    // Using `par_iter()` to parallelize the validation of each modlet.
    verified_paths.par_iter().for_each(|path| {
        let pb = mp.add(ProgressBar::new(count));
        pb.set_style(spinner_style.clone());

        match validate(path, padding, &pb, verbosity) {
            Ok(_) => {
                if verbosity > 0 {
                    pb.finish_with_message(style("OKAY").green().bold().to_string());
                }
            }
            Err(err) => {
                if verbosity > 0 {
                    pb.finish_with_message(style(err).red().bold().to_string());
                }
            }
        }
    });

    Ok(())
}
