use super::*;

pub trait VersionTools {
    fn set_version(&mut self, version: &str);
    fn bump_major(&mut self);
    fn bump_minor(&mut self);
    fn bump_patch(&mut self);
    fn add_pre(&mut self, pre: &str);
    fn add_build(&mut self, build: &str);
}

impl VersionTools for Version {
    fn set_version(&mut self, version: &str) {
        *self = match lenient_semver::parse_into::<Version>(version) {
            Ok(result) => result,
            Err(err) => {
                lenient_semver::parse_into::<Version>(format!("0.0.0+{}", err).as_ref()).unwrap()
            }
        };
    }

    fn bump_major(&mut self) {
        self.major += 1;
        self.minor = 0;
        self.patch = 0;
        self.pre = Prerelease::EMPTY;
        self.build = BuildMetadata::EMPTY;
    }

    fn bump_minor(&mut self) {
        self.minor += 1;
        self.patch = 0;
        self.pre = Prerelease::EMPTY;
        self.build = BuildMetadata::EMPTY;
    }

    fn bump_patch(&mut self) {
        self.patch += 1;
        self.pre = Prerelease::EMPTY;
        self.build = BuildMetadata::EMPTY;
    }

    fn add_build(&mut self, build: &str) {
        self.build = BuildMetadata::new(build).unwrap();
    }

    fn add_pre(&mut self, pre: &str) {
        self.pre = Prerelease::new(pre).unwrap();
    }
}
