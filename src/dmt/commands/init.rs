use crate::cli::RequestedVersion;
use dialoguer::{theme::ColorfulTheme, Confirm};
use modinfo::{Modinfo, ModinfoError};
use std::{fs, path::PathBuf};

pub fn run(name: String, requested_version: &Option<RequestedVersion>) -> Result<bool, ModinfoError> {
    let config_path: PathBuf = [".", &name, "Config/.keep"].iter().collect();
    let modinfo_path: PathBuf = [".", &name, "ModInfo.xml"].iter().collect();
    let readme_path: PathBuf = [".", &name, "README.md"].iter().collect();

    if modinfo_path.exists()
        && !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Modlet {} already exists. Overwrite?", name))
            .default(false)
            .interact()
            .unwrap()
    {
        return Ok(false);
    }

    let modinfo_version = super::requested_version_to_modinfo_version(requested_version);

    fs::create_dir_all(config_path)?;
    fs::write(readme_path, format!("# {}", name))?;

    let mut modinfo = Modinfo::new();
    modinfo.set_modinfo_version(modinfo_version);
    modinfo.set_value_for("name", &name);
    modinfo.set_value_for("display_name", &name);
    match modinfo.write(Some(&modinfo_path)) {
        Ok(_) => Ok(true),
        Err(_) => Err(ModinfoError::WriteError),
    }
}
