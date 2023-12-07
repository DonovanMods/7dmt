use crate::cli::RequestedVersion;
use dialoguer::{theme::ColorfulTheme, Confirm};
use modinfo::{Modinfo, ModinfoError};
use std::{
    fs,
    path::{Path, PathBuf},
};

struct ModletPaths {
    config: PathBuf,
    modinfo: PathBuf,
    readme: PathBuf,
}

impl ModletPaths {
    fn new(name: &str) -> Self {
        let root = Path::new(".").join(name);
        let config = root.join("Config/.keep");
        let modinfo = root.join("ModInfo.xml");
        let readme = root.join("README.md");

        Self {
            config,
            modinfo,
            readme,
        }
    }
}

pub fn run(name: impl ToString, requested_version: Option<&RequestedVersion>) -> Result<bool, ModinfoError> {
    let name = name.to_string();
    let modlet_paths = ModletPaths::new(&name);
    if modlet_paths.modinfo.exists()
        && !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Modlet {} already exists. Overwrite?", name))
            .default(false)
            .interact()
            .unwrap()
    {
        return Ok(false);
    }

    create(name, requested_version)
}

pub fn create(name: impl ToString, requested_version: Option<&RequestedVersion>) -> Result<bool, ModinfoError> {
    let name = name.to_string();
    let modlet_paths = ModletPaths::new(&name);
    let modinfo_version = super::requested_version_to_modinfo_version(requested_version);

    fs::create_dir_all(modlet_paths.config)?;
    fs::write(modlet_paths.readme, format!("# {}", name))?;

    let mut modinfo = Modinfo::new();
    modinfo.set_modinfo_version(modinfo_version);
    modinfo.set_value_for("name", &name);
    modinfo.set_value_for("display_name", &name);
    match modinfo.write(Some(&modlet_paths.modinfo)) {
        Ok(_) => Ok(true),
        Err(_) => Err(ModinfoError::WriteError),
    }
}
