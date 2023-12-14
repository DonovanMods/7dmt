use eyre::{eyre, Result};
use rayon::prelude::*;
use std::path::{Path, PathBuf};

pub fn verify_modlet_path(path: impl AsRef<Path>) -> Option<PathBuf> {
    let path = path
        .as_ref()
        .canonicalize()
        .expect("Failed to canonicalize path {path:?}");

    if path.exists() && path.is_dir() && path.join("modinfo.xml").exists() {
        Some(path)
    } else {
        None
    }
}

pub fn verify_modlet_paths(paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let verified_paths = paths
        .par_iter()
        .filter_map(verify_modlet_path)
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
