use crate::cli::RequestedVersion;
use modinfo::{Modinfo, ModinfoError};
use std::{fs, path::PathBuf};

pub fn run<'m>(
    name: String,
    use_version: &Option<RequestedVersion>,
) -> Result<(), ModinfoError<'m>> {
    let config_path: PathBuf = [".", &name, "Config/.keep"].iter().collect();
    let modinfo_path: PathBuf = [".", &name, "ModInfo.xml"].iter().collect();
    let readme_path: PathBuf = [".", &name, "README.md"].iter().collect();

    let modinfo_version = match use_version {
        Some(ver) => match ver {
            _ if ver.v1 => modinfo::ModinfoVersion::V1,
            _ if ver.v2 => modinfo::ModinfoVersion::V2,
            _ => modinfo::ModinfoVersion::V2,
        },
        None => modinfo::ModinfoVersion::V2,
    };

    fs::create_dir_all(config_path)?;
    fs::write(readme_path, format!("# {}", name))?;

    let mut modinfo = Modinfo::new();
    modinfo.set_modinfo_version(modinfo_version);
    modinfo.set_name(&name);
    modinfo.set_display_name(&name);
    match modinfo.write(Some(&modinfo_path)) {
        Ok(_) => Ok(()),
        Err(_) => Err(ModinfoError::WriteError),
    }
}
