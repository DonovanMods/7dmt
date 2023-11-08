use quick_xml::{
    events::{attributes::Attributes, Event},
    reader::Reader,
};
use std::{collections::HashMap, path::Path};

/// The version of the modinfo.xml file
///
/// V1:
/// ```xml
/// <ModInfo>
///   <Name value="BFT2020_AllInOneMod" />
///   <Description value="MyMod" />
///   <Author value="BFT2020" />
///   <Version value="0.1" />
/// </ModInfo>
/// ```
///
/// V2:
/// ```xml
/// <?xml version="1.0" encoding="utf-8"?>
/// <xml>
///   <Name value="SomeInternalName" />
///   <DisplayName value="Official Mod Name" />
///   <Version value="1.0.0.0" />
///   <Description value="Mod to show format of ModInfo v2" />
///   <Author value="Name" />
///   <Website value="HP" />
/// </xml>
/// ```
#[derive(Debug)]
enum ModinfoVersion {
    V1,
    V2,
}

#[derive(Debug)]
#[allow(dead_code)] // we're returning a struct using these
enum ModinfoValues {
    Author { value: String },
    Description { value: String },
    DisplayName { value: String },
    Name { value: String },
    Version { value: String, compat: String },
    Website { value: String },
}

#[derive(Debug)]
pub struct Modinfo {
    author: ModinfoValues,
    description: ModinfoValues,
    display_name: ModinfoValues,
    name: ModinfoValues,
    version: ModinfoValues,
    website: ModinfoValues,
    modinfo_version: ModinfoVersion,
}

impl Default for Modinfo {
    fn default() -> Self {
        Modinfo {
            author: ModinfoValues::Author {
                value: String::new(),
            },
            description: ModinfoValues::Description {
                value: String::new(),
            },
            display_name: ModinfoValues::DisplayName {
                value: String::new(),
            },
            name: ModinfoValues::Name {
                value: String::new(),
            },
            version: ModinfoValues::Version {
                value: String::new(),
                compat: String::new(),
            },
            website: ModinfoValues::Website {
                value: String::new(),
            },
            modinfo_version: ModinfoVersion::V1,
        }
    }
}

impl Modinfo {
    pub fn read(file: &Path) -> Modinfo {
        let mut modinfo = Modinfo::default();
        let mut buf: Vec<u8> = Vec::new();
        let mut reader = Reader::from_file(file)
            .unwrap_or_else(|e| panic!("Unable to read from {}: {}", file.display(), e));
        // reader.trim_text(true);

        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                // Root Element
                Ok(Event::Start(e)) => {
                    if e.name().as_ref() == b"xml" {
                        modinfo.modinfo_version = ModinfoVersion::V2;
                    }
                }
                // Child Elements (because they have no children)
                Ok(Event::Empty(e)) => {
                    let attributes = parse_attributes(e.attributes());
                    let value = attributes["value"].clone();

                    match e.name().as_ref() {
                        b"Author" => modinfo.author = ModinfoValues::Author { value },
                        b"Description" => {
                            modinfo.description = ModinfoValues::Description { value }
                        }
                        b"DisplayName" => {
                            modinfo.display_name = ModinfoValues::DisplayName { value }
                        }
                        b"Name" => modinfo.name = ModinfoValues::Name { value },
                        b"Version" => {
                            let compat = attributes["compat"].clone();
                            modinfo.version = ModinfoValues::Version { value, compat }
                        }
                        b"Website" => modinfo.website = ModinfoValues::Website { value },
                        _ => println!(
                            "Found unused Tag: {:?}, attributes: {:?}",
                            String::from_utf8_lossy(e.name().as_ref()),
                            &attributes
                        ),
                    }
                }
                Ok(_) => (),
            }

            buf.clear();
        }

        modinfo
    }
}

fn parse_attributes(input: Attributes) -> HashMap<String, String> {
    let mut attributes = HashMap::new();

    input.map(|a| a.unwrap()).for_each(|a| {
        let key: String = String::from_utf8_lossy(a.key.as_ref()).into_owned();
        let value = String::from_utf8(a.value.into_owned()).unwrap();

        attributes.insert(key, value);
    });

    attributes
}

pub fn parse(file: &Path) -> Modinfo {
    Modinfo::read(file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = false;
        assert!(result);
    }
}
