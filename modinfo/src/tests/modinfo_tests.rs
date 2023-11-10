use crate::tests::fixtures;
use crate::*;

mod version_tests {
    use crate::*;

    #[test]
    fn get_version_test() {
        let modinfo = Modinfo::default();

        assert_eq!(modinfo.get_version(), &semver::Version::new(1, 2, 3));
    }

    #[test]
    fn set_version_test() {
        let mut modinfo = Modinfo::default();
        modinfo.set_version("1.2.3").unwrap();

        assert_eq!(modinfo.get_version(), &semver::Version::new(1, 2, 3));
    }

    #[test]
    fn bump_version_test() {
        let mut modinfo = Modinfo::default();
        modinfo.set_version("1.2.3").unwrap();
        modinfo.bump_major();

        assert_eq!(modinfo.get_version(), &semver::Version::new(2, 0, 0));
    }
}

#[test]
fn value_key_test() {
    let modinfo = Modinfo::from_string(fixtures::xml_string_v1());

    assert_eq!(modinfo.value_key(), "value");
}

#[test]
fn get_value_for_test() {
    let modinfo = Modinfo::default();

    assert_eq!(modinfo.get_value_for("Name").unwrap(), "SomeInternalName");
    assert_eq!(modinfo.get_value_for("compat").unwrap(), "1.2.3");
    assert_eq!(
        modinfo.get_value_for("Description").unwrap(),
        "Some description"
    );
    assert_eq!(modinfo.get_value_for("Author").unwrap(), "Some author");
    assert_eq!(modinfo.get_value_for("Website").unwrap(), "Some website");
}
