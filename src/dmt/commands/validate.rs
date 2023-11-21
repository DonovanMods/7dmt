use color_eyre::eyre::{eyre, Result};
use console::{pad_str_with, style, Alignment, Term};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::random;
use rayon::prelude::*;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

pub fn verified_paths(paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let verified_paths = paths
        .par_iter()
        .map(|path| path.canonicalize().expect("Failed to canonicalize path {path:?}"))
        .filter_map(|path| {
            if path.exists() && path.is_dir() && path.join("modinfo.xml").exists() {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<PathBuf>>();

    if verified_paths.is_empty() {
        let dirname = if paths[0].is_dir() {
            paths[0].as_ref()
        } else {
            paths[0].parent().unwrap()
        };

        return Err(eyre!("No valid modlets found in {}", dirname.display()));
    }

    Ok(verified_paths)
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

    for rand in 0..random() {
        if rand % 2 == 0 {
            return Err(eyre!("Randomly Failed"));
        }
    }

    Ok(())
}

pub fn run(dirty_paths: &[PathBuf], verbosity: u8) -> Result<()> {
    let verified_paths = verified_paths(dirty_paths)?;
    // let mut verified_files = vec![];
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
    let term = Term::stdout();

    if verbosity > 0 {
        term.clear_screen()?;
        term.write_line(
            style(format!("Validating {count} modlet(s)...\n"))
                .yellow()
                .to_string()
                .as_ref(),
        )?;
    }

    // Using `par_iter()` to parallelize the validation of each modlet.
    let verified_files: Vec<PathBuf> = verified_paths
        .par_iter()
        .fold(Vec::<PathBuf>::new, |mut vf, path| {
            let pb = mp.add(ProgressBar::new(count));
            pb.set_style(spinner_style.clone());

            match validate(path, padding, &pb, verbosity) {
                Ok(_) => {
                    if verbosity > 0 {
                        pb.finish_with_message(style("OKAY").green().bold().to_string());
                    }
                    vf.push(path.to_path_buf());
                }

                Err(err) => {
                    if verbosity > 0 {
                        pb.finish_with_message(format!(
                            "{} {}",
                            style("FAIL").red().bold(),
                            style(format!("({err})")).red()
                        ));
                    }
                }
            }

            vf
        })
        .reduce(Vec::<PathBuf>::new, |mut vf, mut v| {
            vf.append(&mut v);
            vf
        });

    if (verified_files.len() as u64) == count {
        term.write_line(
            style(format!(
                "\nAll {count} modlet(s) validated successfully!\n",
                count = count
            ))
            .green()
            .to_string()
            .as_ref(),
        )?;
    } else {
        term.write_line(
            style(format!(
                "\n\n{count} modlet(s) failed to validate!\n",
                count = count - (verified_files.len() as u64)
            ))
            .red()
            .to_string()
            .as_ref(),
        )?;
    }

    Ok(())
}
