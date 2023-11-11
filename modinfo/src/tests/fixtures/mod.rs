pub fn xml_string_v1() -> String {
    r#"
          <ModInfo>
              <Name value="SomeInternalName" />
              <Version value="1.2.3" compat="A99" />
              <Description value="Mod to show format of ModInfo v1" />
              <Author value="Name" />
          </ModInfo>
      "#
    .to_string()
}

pub fn xml_string_v1_no_compat() -> String {
    r#"
          <ModInfo>
              <Name value="SomeInternalName" />
              <Version value="1.2.3" />
              <Description value="Mod to show format of ModInfo v1" />
              <Author value="Name" />
          </ModInfo>
      "#
    .to_string()
}

pub fn xml_string_v2() -> String {
    r#"
          <?xml version="1.0" encoding="UTF-8"?>
          <xml>
              <Name value="SomeInternalName" />
              <DisplayName value="Official Mod Name" />
              <Version value="2.3.4" compat="A99" />
              <Description value="Mod to show format of ModInfo v2" />
              <Author value="Name" />
              <Website value="HP" />
          </xml>
      "#
    .to_string()
}

pub fn xml_string_v2_no_compat() -> String {
    r#"
          <?xml version="1.0" encoding="UTF-8"?>
          <xml>
              <Name value="SomeInternalName" />
              <DisplayName value="Official Mod Name" />
              <Version value="2.3.4" />
              <Description value="Mod to show format of ModInfo v2" />
              <Author value="Name" />
              <Website value="HP" />
          </xml>
      "#
    .to_string()
}
