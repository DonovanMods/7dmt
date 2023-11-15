use crate::cli::RequestedVersion;
use modinfo::ModinfoError as Error;
use std::path::Path;

pub fn run(path: impl AsRef<Path>, requested_version: &Option<RequestedVersion>) -> Result<(), Error> {
    let modinfo_version = super::requested_version_to_modinfo_version(requested_version);
    let mut modinfo = modinfo::parse(path)?;

    if modinfo.get_modinfo_version() == modinfo_version {
        Ok(())
    } else {
        modinfo.set_modinfo_version(modinfo_version);
        modinfo.write(None)
    }
}
