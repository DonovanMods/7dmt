use modinfo::ModinfoError;
use std::path::PathBuf;

#[derive(Debug)]
pub enum BumpOptions {
    Major,
    Minor,
    Patch,
    Set(String),
    Verbosity(u8),
}

pub fn run(modlet: PathBuf, opts: &Vec<BumpOptions>) -> Result<String, String> {
    // dbg!(opts);

    let mut verbosity = 0;
    let mut modinfo = match modinfo::parse(modlet.clone()) {
        Ok(result) => result,
        Err(err) => {
            return match err {
                ModinfoError::IoError(err) => {
                    Err(format!("Could not read {}: {}", modlet.display(), err))
                }
                ModinfoError::FsNotFound => Err(format!("{} does not exist", modlet.display())),
                _ => Err(format!("Could not parse modinfo: {:?}", err)),
            }
        }
    };
    let old_ver = modinfo.get_version().to_string();

    for options in opts {
        match options {
            BumpOptions::Set(ver) => modinfo.set_version(ver),
            BumpOptions::Major => modinfo.bump_version_major(),
            BumpOptions::Minor => modinfo.bump_version_minor(),
            BumpOptions::Patch => modinfo.bump_version_patch(),
            BumpOptions::Verbosity(some) => {
                verbosity = *some;
            }
        }
    }

    if verbosity >= 1 {
        dbg!(&modinfo);
    }

    // TODO: bump version here!

    match &modinfo.write(None) {
        Ok(_) => Ok(format!(
            "Bumped version of {} from {} to {}",
            modlet.display(),
            old_ver,
            modinfo.get_version(),
        )),
        Err(err) => Err(format!("{}", err)),
    }
}
