use super::*;

fn xml_string_v1() -> String {
    r#"
          <ModInfo>
              <Name value="SomeInternalName" />
              <Version value="1.0" compat="A99" />
              <Description value="Mod to show format of ModInfo v1" />
              <Author value="Name" />
          </ModInfo>
      "#
    .to_string()
}

fn xml_string_v1_no_compat() -> String {
    r#"
          <ModInfo>
              <Name value="SomeInternalName" />
              <Version value="1.0" />
              <Description value="Mod to show format of ModInfo v1" />
              <Author value="Name" />
          </ModInfo>
      "#
    .to_string()
}

fn xml_string_v2() -> String {
    r#"
          <?xml version="1.0" encoding="utf-8"?>
          <xml>
              <Name value="SomeInternalName" />
              <DisplayName value="Official Mod Name" />
              <Version value="2.0" compat="A99" />
              <Description value="Mod to show format of ModInfo v2" />
              <Author value="Name" />
              <Website value="HP" />
          </xml>
      "#
    .to_string()
}

fn xml_string_v2_no_compat() -> String {
    r#"
          <?xml version="1.0" encoding="utf-8"?>
          <xml>
              <Name value="SomeInternalName" />
              <DisplayName value="Official Mod Name" />
              <Version value="2.0" />
              <Description value="Mod to show format of ModInfo v2" />
              <Author value="Name" />
              <Website value="HP" />
          </xml>
      "#
    .to_string()
}

#[test]
fn from_string_v1_test() {
    let result = Modinfo::from_string(xml_string_v1());

    assert_eq!(
        result.name,
        ModinfoValues::Name {
            value: Some("SomeInternalName".to_string())
        }
    );
    assert_eq!(
        result.display_name,
        ModinfoValues::DisplayName { value: None }
    );
    assert_eq!(
        result.version,
        ModinfoValues::Version {
            value: Some("1.0".to_string()),
            compat: Some("A99".to_string())
        }
    );
    assert_eq!(
        result.description,
        ModinfoValues::Description {
            value: Some("Mod to show format of ModInfo v1".to_string())
        }
    );
    assert_eq!(
        result.author,
        ModinfoValues::Author {
            value: Some("Name".to_string())
        }
    );
    assert_eq!(result.website, ModinfoValues::Website { value: None });
}

#[test]
fn from_string_v1_no_compat_test() {
    let result = Modinfo::from_string(xml_string_v1_no_compat());

    assert_eq!(
        result.name,
        ModinfoValues::Name {
            value: Some("SomeInternalName".to_string())
        }
    );
    assert_eq!(
        result.display_name,
        ModinfoValues::DisplayName { value: None }
    );
    assert_eq!(
        result.version,
        ModinfoValues::Version {
            value: Some("1.0".to_string()),
            compat: None
        }
    );
    assert_eq!(
        result.description,
        ModinfoValues::Description {
            value: Some("Mod to show format of ModInfo v1".to_string())
        }
    );
    assert_eq!(
        result.author,
        ModinfoValues::Author {
            value: Some("Name".to_string())
        }
    );
    assert_eq!(result.website, ModinfoValues::Website { value: None });
}

#[test]
fn from_string_v2_test() {
    let result = Modinfo::from_string(xml_string_v2());

    assert_eq!(
        result.name,
        ModinfoValues::Name {
            value: Some("SomeInternalName".to_string())
        }
    );
    assert_eq!(
        result.display_name,
        ModinfoValues::DisplayName {
            value: Some("Official Mod Name".to_string())
        }
    );
    assert_eq!(
        result.version,
        ModinfoValues::Version {
            value: Some("2.0".to_string()),
            compat: Some("A99".to_string())
        }
    );
    assert_eq!(
        result.description,
        ModinfoValues::Description {
            value: Some("Mod to show format of ModInfo v2".to_string())
        }
    );
    assert_eq!(
        result.author,
        ModinfoValues::Author {
            value: Some("Name".to_string())
        }
    );
    assert_eq!(
        result.website,
        ModinfoValues::Website {
            value: Some("HP".to_string())
        }
    );
}

#[test]
fn from_string_v2_no_compat_test() {
    let result = Modinfo::from_string(xml_string_v2_no_compat());

    assert_eq!(
        result.name,
        ModinfoValues::Name {
            value: Some("SomeInternalName".to_string())
        }
    );
    assert_eq!(
        result.display_name,
        ModinfoValues::DisplayName {
            value: Some("Official Mod Name".to_string())
        }
    );
    assert_eq!(
        result.version,
        ModinfoValues::Version {
            value: Some("2.0".to_string()),
            compat: None
        }
    );
    assert_eq!(
        result.description,
        ModinfoValues::Description {
            value: Some("Mod to show format of ModInfo v2".to_string())
        }
    );
    assert_eq!(
        result.author,
        ModinfoValues::Author {
            value: Some("Name".to_string())
        }
    );
    assert_eq!(
        result.website,
        ModinfoValues::Website {
            value: Some("HP".to_string())
        }
    );
}
