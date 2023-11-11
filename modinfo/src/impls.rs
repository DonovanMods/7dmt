use super::*;

impl<'m> Modinfo<'m> {
    pub fn new() -> Self {
        Modinfo::default()
    }

    pub fn write(&self) -> Result<(), ModinfoError> {
        fs::write(self.meta.path, self.to_string())?;

        Ok(())
    }

    pub fn get_version(&self) -> &Version {
        &self.version.value
    }

    pub fn get_value_for(&self, field: &str) -> Option<&String> {
        match field.to_lowercase().as_ref() {
            "author" => self.author.value.as_ref(),
            "description" => self.description.value.as_ref(),
            "display_name" => self.display_name.value.as_ref(),
            "name" => self.name.value.as_ref(),
            "website" => self.website.value.as_ref(),
            "compat" => self.version.compat.as_ref(),
            _ => None,
        }
    }

    pub fn set_version(&mut self, version: &'m str) {
        self.version.value.set_version(version)
        // self.version.value = match lenient_semver::parse_into::<Version>(version) {
        //     Ok(result) => result,
        //     Err(err) => return Err(ModinfoError::InvalidVersion(err)),
        // };

        // Ok(())
    }

    pub fn bump_version_major(&mut self) {
        self.version.value.bump_major()
    }

    pub fn bump_version_minor(&mut self) {
        self.version.value.bump_minor()
    }

    pub fn bump_version_patch(&mut self) {
        self.version.value.bump_patch()
    }

    pub fn add_version_pre(&mut self, pre: &'m str) {
        self.version.value.add_pre(pre)
    }

    pub fn add_version_build(&mut self, build: &'m str) {
        self.version.value.add_build(build)
    }
}
