use crate::dmt::{commands, SETTINGS};
use color_eyre::eyre::eyre;
use console::{style, Term};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use modlet::modlet::Modlet;
use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Writer,
};
use rayon::prelude::*;
use std::{
    collections::{btree_map, BTreeMap},
    fs::{self, File},
    path::{Path, PathBuf},
};

/// Reads a modlet's xml files
fn load(path: impl AsRef<Path>, padding: usize, pb: &ProgressBar) -> eyre::Result<Modlet> {
    let path = path.as_ref().canonicalize().unwrap_or_default();
    let file_name = path.file_name().unwrap_or_default().to_str().unwrap();
    let verbose = SETTINGS.read().unwrap().verbosity > 0;
    if verbose {
        pb.set_prefix(format!("Loading {file_name:.<padding$}"));
    }

    let config_dir = path.join("Config");
    if !(config_dir.exists() && config_dir.is_dir()) {
        return Err(eyre!(
            "Invalid Modlet {}: Config directory does not exist",
            config_dir.display()
        ));
    }

    let modlet = Modlet::new(path)?;

    Ok(modlet)
}

fn package(
    file: &Path,
    modlets: Vec<&Modlet>,
    output_modlet: &Path,
    padding: usize,
    pb: &ProgressBar,
) -> eyre::Result<()> {
    let verbose = SETTINGS.read().unwrap().verbosity > 0;
    let config_dir = output_modlet.join("Config");
    let config_file = config_dir.join(file);

    if config_file.exists() {
        fs::remove_file(&config_file)?;
    } else {
        fs::create_dir_all(config_file.parent().unwrap())?;
    };

    let config_file = File::create(&config_file)?;
    let mut writer = Writer::new_with_indent(&config_file, b' ', 4);

    writer.write_event(Event::Start(BytesStart::new("bundle")))?;

    if verbose {
        pb.set_prefix(format!("Packaging {:.<padding$}", file.display()));
    }

    for modlet in modlets {
        if verbose {
            pb.inc(1);
        }

        // Inject a comment to indicate which modlet the xml came from
        writer.write_event(Event::Comment(BytesText::new(
            format!(" Included from {} ", modlet.name()).as_str(),
        )))?;

        modlet.write_xmls(&mut writer, file)?;
    }

    Ok(writer.write_event(Event::End(BytesEnd::new("bundle")))?)
}

fn file_map(modlets: &[Modlet]) -> BTreeMap<PathBuf, Vec<&Modlet>> {
    let mut files = BTreeMap::<PathBuf, Vec<&Modlet>>::new();
    for modlet in modlets {
        for file in modlet.xml_files() {
            let file = file.as_ref().to_owned();
            if let btree_map::Entry::Vacant(e) = files.entry(file.clone()) {
                e.insert(vec![modlet]);
            } else {
                files.get_mut(&file).unwrap().push(modlet);
            }
        }
    }

    files
}

/// Packages one or more modlets into a single modlet
///
/// # Arguments
///
/// * `modlets` - A list of modlet(s) to package
/// * `modlet` - The path to the modlet to package into
///
/// # Errors
///
/// * If the game directory is invalid
/// * If the modlet path is invalid
///
pub fn run(modlets: &[PathBuf], output_modlet: &Path) -> eyre::Result<()> {
    let verbose = SETTINGS.read().unwrap().verbosity > 0;
    let modlet_count = modlets.len() as u64;
    let mp = MultiProgress::new();
    let spinner_style = ProgressStyle::with_template("{prefix:.cyan.bright} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    let mut padding = modlets
        .iter()
        .map(|p| p.as_path().file_name().unwrap().len())
        .max()
        .unwrap_or(0)
        + 3;
    let term = Term::stdout();
    let config_dir = output_modlet.join("Config");
    let output_modlet_name = output_modlet.file_name().unwrap().to_str().unwrap();
    if padding < output_modlet_name.len() {
        padding = output_modlet_name.len() + 3;
    }

    if verbose {
        term.clear_screen()?;
        term.write_line(
            style(format!(
                "Packaging {modlet_count} modlet(s) into {}...\n",
                output_modlet.display()
            ))
            .yellow()
            .to_string()
            .as_ref(),
        )?;
    }

    // Using `par_iter()` to parallelize the packaging of each modlet.
    let mut loaded_modlets: Vec<Modlet> = modlets
        .par_iter()
        .fold(Vec::<Modlet>::new, |mut vf, path| {
            let pb = mp.add(ProgressBar::new(modlet_count));
            pb.set_style(spinner_style.clone());

            match load(path, padding, &pb) {
                Ok(modlet) => {
                    if verbose {
                        pb.finish_with_message(style("OKAY").green().bold().to_string());
                    }
                    vf.push(modlet);
                }

                Err(err) => {
                    if verbose {
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
        .reduce(Vec::<Modlet>::new, |mut vf, mut v| {
            vf.append(&mut v);
            vf
        });

    if (loaded_modlets.len() as u64) == modlet_count {
        // Create the output modlet if necessary
        if !output_modlet.exists() {
            commands::init::create(output_modlet_name, None)?;
        }

        if config_dir.exists() {
            if config_dir.is_dir() {
                fs::remove_dir_all(&config_dir)?;
            } else {
                return Err(eyre!(
                    "Invalid Modlet {}: Config directory is not a directory",
                    config_dir.display()
                ));
            }
        }

        // Sort modlets by name to ensure consistent packaging
        loaded_modlets.sort_by(|a, b| a.name().cmp(&b.name()));

        let modlets = loaded_modlets.clone();
        let files = file_map(&modlets);
        let files_count = files.len() as u64;

        if config_dir.exists() {
            if config_dir.is_dir() {
                fs::remove_dir_all(&config_dir)?;
            } else {
                return Err(eyre!(
                    "Invalid Modlet {}: Config directory is not a directory",
                    config_dir.display()
                ));
            }
        }

        // Write XML files
        files
            .into_par_iter()
            .try_for_each(|(file, modlets)| -> eyre::Result<()> {
                let pb = mp.add(ProgressBar::new(files_count));
                pb.set_style(spinner_style.clone());

                match package(&file, modlets, output_modlet, padding - 2, &pb) {
                    Ok(_) => {
                        if verbose {
                            pb.finish_with_message(style("OKAY").green().bold().to_string());
                        }
                    }
                    Err(err) => {
                        if verbose {
                            pb.finish_with_message(format!(
                                "{} {}",
                                style("FAIL").red().bold(),
                                style(format!("({err})")).red()
                            ));
                        }
                    }
                }

                Ok(())
            })?;

        // Write other files
        let pb = mp.add(ProgressBar::new(1));
        pb.set_style(spinner_style.clone());

        if verbose {
            let padding = padding - 2;
            pb.set_prefix(format!("Packaging {:.<padding$}", "additional files"));
        }

        for modlet in loaded_modlets {
            if verbose {
                pb.inc(1);
            }

            modlet.write_files(output_modlet)?;
        }
        pb.finish_with_message(style("OKAY").green().bold().to_string());

        term.write_line(
            style(format!(
                "\n\n{modlet_count} modlet(s) successfully packaged into {}\n",
                output_modlet.file_name().unwrap_or_default().to_str().unwrap()
            ))
            .green()
            .to_string()
            .as_ref(),
        )?;
    } else {
        term.write_line(
            style(format!(
                "\n\n{count} modlet(s) failed to package!\n",
                count = modlet_count - (loaded_modlets.len() as u64)
            ))
            .red()
            .to_string()
            .as_ref(),
        )?;
    }

    Ok(())
}
