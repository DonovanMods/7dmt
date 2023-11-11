use super::*;

#[test]
fn get_version_test() {
    let modinfo = Modinfo::default();

    assert_eq!(modinfo.get_version(), &semver::Version::new(0, 1, 0));
}

#[test]
fn set_version_test() {
    // Needed for compat field
    let mut modinfo = Modinfo::from_string(fixtures::xml_string_v2());
    modinfo.set_version("5.6.7").ok();

    assert_eq!(modinfo.get_version(), &semver::Version::new(5, 6, 7));
    assert!(modinfo.version.compat.is_some());
}

#[test]
fn bump_version_major_test() {
    let mut modinfo = Modinfo::new();
    modinfo.set_version("1.2.3").ok();
    modinfo.bump_version_major();

    assert_eq!(modinfo.get_version(), &semver::Version::new(2, 0, 0));
}

#[test]
fn bump_version_minor_test() {
    let mut modinfo = Modinfo::new();
    modinfo.set_version("1.2.3").ok();
    modinfo.bump_version_minor();

    assert_eq!(modinfo.get_version(), &semver::Version::new(1, 3, 0));
}

#[test]
fn bump_version_patch_test() {
    let mut modinfo = Modinfo::new();
    modinfo.set_version("1.2.3").ok();
    modinfo.bump_version_patch();

    assert_eq!(modinfo.get_version(), &semver::Version::new(1, 2, 4));
}

#[test]
fn add_version_pre_test() {
    let mut modinfo = Modinfo::new();
    modinfo.set_version("1.2.3").ok();
    modinfo.add_version_pre("alpha");

    assert_eq!(
        modinfo.get_version(),
        &semver::Version::parse("1.2.3-alpha").unwrap()
    );
}

#[test]
fn add_version_build_test() {
    let mut modinfo = Modinfo::new();
    modinfo.set_version("1.2.3").ok();
    modinfo.add_version_build("build");

    assert_eq!(
        modinfo.get_version(),
        &semver::Version::parse("1.2.3+build").unwrap()
    );
}

#[test]
fn get_version_string_test() {
    let mut modinfo = Modinfo::new();
    modinfo.set_version("1.2.3").ok();
    modinfo.add_version_pre("alpha");
    modinfo.add_version_build("build");

    assert_eq!(modinfo.get_version().to_string(), "1.2.3-alpha+build");
}
