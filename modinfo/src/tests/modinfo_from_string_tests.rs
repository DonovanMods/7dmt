use super::*;

#[test]
fn from_string_v1_test() {
    let result = Modinfo::from_string(fixtures::xml_string_v1());
    let version = lenient_semver::parse("1.2.3").unwrap();

    assert_eq!(
        result.name,
        ModinfoValue {
            value: Some("SomeInternalName".to_string())
        }
    );
    assert_eq!(result.display_name, ModinfoValue { value: None });
    assert_eq!(
        result.version,
        ModinfoValueVersion {
            value: version,
            compat: Some("A99".to_string()),
        }
    );
    assert_eq!(
        result.description,
        ModinfoValue {
            value: Some("Mod to show format of ModInfo v1".to_string())
        }
    );
    assert_eq!(
        result.author,
        ModinfoValue {
            value: Some("Name".to_string())
        }
    );
    assert_eq!(result.website, ModinfoValue { value: None });
}

#[test]
fn from_string_v1_no_compat_test() {
    let result = Modinfo::from_string(fixtures::xml_string_v1_no_compat());
    let version = lenient_semver::parse("1.2.3").unwrap();

    assert_eq!(
        result.name,
        ModinfoValue {
            value: Some("SomeInternalName".to_string())
        }
    );
    assert_eq!(result.display_name, ModinfoValue { value: None });
    assert_eq!(
        result.version,
        ModinfoValueVersion {
            value: version,
            compat: None
        }
    );
    assert_eq!(
        result.description,
        ModinfoValue {
            value: Some("Mod to show format of ModInfo v1".to_string())
        }
    );
    assert_eq!(
        result.author,
        ModinfoValue {
            value: Some("Name".to_string())
        }
    );
    assert_eq!(result.website, ModinfoValue { value: None });
}

#[test]
fn from_string_v2_test() {
    let result = Modinfo::from_string(fixtures::xml_string_v2());
    let version = lenient_semver::parse("2.3.4").unwrap();

    assert_eq!(
        result.name,
        ModinfoValue {
            value: Some("SomeInternalName".to_string())
        }
    );
    assert_eq!(
        result.display_name,
        ModinfoValue {
            value: Some("Official Mod Name".to_string())
        }
    );
    assert_eq!(
        result.version,
        ModinfoValueVersion {
            value: version,
            compat: Some("A99".to_string())
        }
    );
    assert_eq!(
        result.description,
        ModinfoValue {
            value: Some("Mod to show format of ModInfo v2".to_string())
        }
    );
    assert_eq!(
        result.author,
        ModinfoValue {
            value: Some("Name".to_string())
        }
    );
    assert_eq!(
        result.website,
        ModinfoValue {
            value: Some("HP".to_string())
        }
    );
}

#[test]
fn from_string_v2_no_compat_test() {
    let result = Modinfo::from_string(fixtures::xml_string_v2_no_compat());
    let version = lenient_semver::parse("2.3.4").unwrap();

    assert_eq!(
        result.name,
        ModinfoValue {
            value: Some("SomeInternalName".to_string())
        }
    );
    assert_eq!(
        result.display_name,
        ModinfoValue {
            value: Some("Official Mod Name".to_string())
        }
    );
    assert_eq!(
        result.version,
        ModinfoValueVersion {
            value: version,
            compat: None
        }
    );
    assert_eq!(
        result.description,
        ModinfoValue {
            value: Some("Mod to show format of ModInfo v2".to_string())
        }
    );
    assert_eq!(
        result.author,
        ModinfoValue {
            value: Some("Name".to_string())
        }
    );
    assert_eq!(
        result.website,
        ModinfoValue {
            value: Some("HP".to_string())
        }
    );
}
