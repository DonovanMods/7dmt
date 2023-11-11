use super::*;

mod version_tests {
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
}

#[test]
fn get_value_for_test() {
    let modinfo = Modinfo::from_string(fixtures::xml_string_v2());

    assert_eq!(
        modinfo.get_value_for("name"),
        Some(&String::from("SomeInternalName"))
    );
    assert_eq!(
        modinfo.get_value_for("display_name"),
        Some(&String::from("Official Mod Name"))
    );
    assert_eq!(modinfo.get_value_for("author"), Some(&String::from("Name")));
    assert_eq!(modinfo.get_value_for("compat"), Some(&String::from("A99")));
    assert_eq!(
        modinfo.get_value_for("description"),
        Some(&String::from("Mod to show format of ModInfo v2"))
    );
    assert_eq!(modinfo.get_value_for("website"), Some(&String::from("HP")));
    assert_eq!(modinfo.get_value_for("foo"), None);
}