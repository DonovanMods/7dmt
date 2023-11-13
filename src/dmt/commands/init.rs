use modinfo::{Modinfo, ModinfoError};
use std::{fs, path::PathBuf};

pub fn run<'m>(name: String) -> Result<(), ModinfoError<'m>> {
    let config_path: PathBuf = [".", &name, "Config/.keep"].iter().collect();
    let modinfo_path: PathBuf = [".", &name, "ModInfo.xml"].iter().collect();
    let readme_path: PathBuf = [".", &name, "README.md"].iter().collect();

    fs::create_dir_all(config_path)?;
    fs::write(readme_path, format!("# {}", name))?;

    let modinfo = Modinfo::new();
    match modinfo.write(Some(&modinfo_path)) {
        Ok(_) => Ok(()),
        Err(_) => Err(ModinfoError::WriteError),
    }
}
