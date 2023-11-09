use super::*;

fn xml_string_v1() -> String {
    r#"
          <ModInfo>
              <Name value="SomeInternalName" />
              <Version value="1.0.0" compat="A99" />
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
              <Version value="1.0.0" />
              <Description value="Mod to show format of ModInfo v1" />
              <Author value="Name" />
          </ModInfo>
      "#
    .to_string()
}

fn xml_string_v2() -> String {
    r#"
          <?xml version="1.0" encoding="UTF-8"?>
          <xml>
              <Name value="SomeInternalName" />
              <DisplayName value="Official Mod Name" />
              <Version value="2.0.0" compat="A99" />
              <Description value="Mod to show format of ModInfo v2" />
              <Author value="Name" />
              <Website value="HP" />
          </xml>
      "#
    .to_string()
}

fn xml_string_v2_no_compat() -> String {
    r#"
          <?xml version="1.0" encoding="UTF-8"?>
          <xml>
              <Name value="SomeInternalName" />
              <DisplayName value="Official Mod Name" />
              <Version value="2.0.0" />
              <Description value="Mod to show format of ModInfo v2" />
              <Author value="Name" />
              <Website value="HP" />
          </xml>
      "#
    .to_string()
}

fn strip_ws(s: &str) -> String {
    s.split_whitespace().collect()
}

#[test]
fn to_string_v1_test() {
    let xml = xml_string_v1();
    let result = Modinfo::from_string(xml.clone()).to_string();

    assert_eq!(strip_ws(&result), strip_ws(&xml));
}

#[test]
fn to_string_v1_no_compat_test() {
    let xml = xml_string_v1_no_compat();
    let result = Modinfo::from_string(xml.clone()).to_string();

    assert_eq!(strip_ws(&result), strip_ws(&xml));
}

#[test]
fn to_string_v2_test() {
    let xml = xml_string_v2();
    let result = Modinfo::from_string(xml.clone()).to_string();

    assert_eq!(strip_ws(&result), strip_ws(&xml));
}

#[test]
fn to_string_v2_no_compat_test() {
    let xml = xml_string_v2_no_compat();
    let result = Modinfo::from_string(xml.clone()).to_string();

    assert_eq!(strip_ws(&result), strip_ws(&xml));
}
