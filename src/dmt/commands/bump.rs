use std::path::Path;

#[derive(Debug)]
pub enum BumpOptions {
    Major,
    Minor,
    Patch,
    Set(String),
}

pub fn run(modlet: &Path, opts: &Vec<BumpOptions>) -> Result<String, String> {
    dbg!(opts);

    Ok(format!(
        "Bumped version of {} from {} to {}",
        modlet.display(),
        "old_ver",
        "new_ver"
    ))
}
