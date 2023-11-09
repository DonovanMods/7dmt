use convert_case::{Case, Casing};
use quick_xml::{events::*, reader::Reader, writer::Writer};
use std::{borrow::Cow, collections::HashMap, error, fs, io::Cursor, path::Path};

// Tests Module
#[cfg(test)]
mod modinfo_from_string_tests;

#[cfg(test)]
mod modinfo_to_string_tests;

trait FromString<'m> {
    fn from_string(xml: String) -> Modinfo<'m>;
}

#[derive(Debug)]
pub enum ModinfoError {
    IoError(std::io::Error),
    XMLError(quick_xml::Error),
    FsNotFound,
    NoModinfo,
    NoModinfoAuthor,
    NoModinfoDescription,
    NoModinfoDisplayName,
    NoModinfoName,
    NoModinfoVersion,
    NoModinfoVersionValue,
    NoModinfoVersionCompat,
    NoModinfoWebsite,
    UnknownTag(String),
}

impl error::Error for ModinfoError {}
impl std::fmt::Display for ModinfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModinfoError::IoError(err) => write!(f, "I/O error occurred: {}", err),
            ModinfoError::XMLError(err) => write!(f, "XML error occurred: {}", err),
            ModinfoError::FsNotFound => write!(f, "File not found"),
            ModinfoError::NoModinfo => write!(f, "No modinfo.xml found"),
            ModinfoError::NoModinfoAuthor => write!(f, "No Author found in modinfo.xml"),
            ModinfoError::NoModinfoDescription => write!(f, "No Description found in modinfo.xml"),
            ModinfoError::NoModinfoDisplayName => {
                write!(f, "No DisplayName found in modinfo.xml")
            }
            ModinfoError::NoModinfoName => write!(f, "No Name found in modinfo.xml"),
            ModinfoError::NoModinfoVersion => write!(f, "No Version found in modinfo.xml"),
            ModinfoError::NoModinfoVersionValue => {
                write!(f, "No Version value found in modinfo.xml")
            }
            ModinfoError::NoModinfoVersionCompat => {
                write!(f, "No Version compat found in modinfo.xml")
            }
            ModinfoError::NoModinfoWebsite => write!(f, "No Website found in modinfo.xml"),
            ModinfoError::UnknownTag(err) => write!(f, "{}", err),
        }
    }
}
impl From<std::io::Error> for ModinfoError {
    fn from(err: std::io::Error) -> Self {
        ModinfoError::IoError(err)
    }
}
impl From<quick_xml::Error> for ModinfoError {
    fn from(err: quick_xml::Error) -> Self {
        ModinfoError::XMLError(err)
    }
}

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
#[derive(Debug, PartialEq)]
enum ModinfoVersion {
    V1,
    V2,
}

#[derive(Debug, PartialEq)]
enum ModinfoValues {
    Author {
        value: Option<String>,
    },
    Description {
        value: Option<String>,
    },
    DisplayName {
        value: Option<String>,
    },
    Name {
        value: Option<String>,
    },
    Version {
        value: Option<String>,
        compat: Option<String>,
    },
    Website {
        value: Option<String>,
    },
}

#[derive(Debug)]
struct ModinfoMeta<'m> {
    version: ModinfoVersion,
    path: &'m Path,
}

#[derive(Debug)]
pub struct Modinfo<'m> {
    author: ModinfoValues,
    description: ModinfoValues,
    display_name: ModinfoValues,
    name: ModinfoValues,
    version: ModinfoValues,
    website: ModinfoValues,
    meta: ModinfoMeta<'m>,
}

impl<'m> Default for Modinfo<'m> {
    fn default() -> Self {
        Modinfo {
            author: ModinfoValues::Author { value: None },
            description: ModinfoValues::Description { value: None },
            display_name: ModinfoValues::DisplayName { value: None },
            name: ModinfoValues::Name { value: None },
            version: ModinfoValues::Version {
                value: None,
                compat: None,
            },
            website: ModinfoValues::Website { value: None },
            meta: ModinfoMeta {
                version: ModinfoVersion::V1,
                path: Path::new(""),
            },
        }
    }
}

impl<'m> ToString for Modinfo<'m> {
    fn to_string(&self) -> String {
        let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);
        let is_v2 = ModinfoVersion::V2 == self.meta.version;

        let root_str = match is_v2 {
            true => String::from("xml"),
            false => String::from("ModInfo"),
        };

        if is_v2 {
            writer
                .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
                .unwrap();
        }
        writer
            .write_event(Event::Start(BytesStart::new(&root_str)))
            .unwrap();

        // inject the attributes here
        for field in [
            "name",
            "display_name",
            "version",
            "description",
            "author",
            "website",
        ] {
            if !is_v2 && (field == "website" || field == "display_name") {
                continue;
            }

            let field_name = field.to_owned().to_case(Case::Pascal);
            let mut elem = BytesStart::new(field_name);
            let hash = self.get(field);

            elem.push_attribute(attributes::Attribute {
                key: quick_xml::name::QName(b"value"),
                value: Cow::from(hash["value"].as_bytes()),
            });

            if hash.contains_key("compat") {
                elem.push_attribute(attributes::Attribute {
                    key: quick_xml::name::QName(b"compat"),
                    value: Cow::from(hash["compat"].as_bytes()),
                });
            };

            writer.write_event(Event::Empty(elem)).unwrap();
        }

        writer
            .write_event(Event::End(BytesEnd::new(&root_str)))
            .unwrap();

        String::from_utf8(writer.into_inner().into_inner()).unwrap()
    }
}

impl<'m> FromString<'m> for Modinfo<'m> {
    fn from_string(xml: String) -> Self {
        let mut modinfo = Modinfo::default();
        let mut buf: Vec<u8> = Vec::new();
        let mut reader = Reader::from_str(&xml);
        reader.trim_text(true);

        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                // Root Element
                Ok(Event::Start(e)) => {
                    if e.name().as_ref() == b"xml" {
                        modinfo.meta.version = ModinfoVersion::V2;
                    }
                }
                // Child Elements (because they have no children)
                Ok(Event::Empty(e)) => {
                    let attributes = parse_attributes(e.attributes());
                    let value = attributes["value"].clone();

                    match e.name().as_ref() {
                        b"Author" => modinfo.author = ModinfoValues::Author { value: Some(value) },
                        b"Description" => {
                            modinfo.description = ModinfoValues::Description { value: Some(value) }
                        }
                        b"DisplayName" => {
                            modinfo.display_name = ModinfoValues::DisplayName { value: Some(value) }
                        }
                        b"Name" => modinfo.name = ModinfoValues::Name { value: Some(value) },
                        b"Version" => {
                            let mut compat = None;

                            if attributes.contains_key("compat") {
                                compat = Some(attributes["compat"].clone());
                            }
                            modinfo.version = ModinfoValues::Version {
                                value: Some(value),
                                compat,
                            }
                        }
                        b"Website" => {
                            modinfo.website = ModinfoValues::Website { value: Some(value) }
                        }
                        _ => (),
                    }
                }
                Ok(_) => (),
            }

            buf.clear();
        }

        modinfo
    }
}

impl Modinfo<'_> {
    fn get(&self, field: &str) -> HashMap<String, String> {
        let mut return_value = HashMap::new();
        let value_key = String::from("value");

        let attrib = match field {
            "author" => &self.author,
            "description" => &self.description,
            "display_name" => &self.display_name,
            "name" => &self.name,
            "version" => &self.version,
            "website" => &self.website,
            _ => panic!("Invalid field"),
        };

        match attrib {
            ModinfoValues::Author { value: Some(value) } => {
                return_value.insert(value_key, value.to_owned())
            }
            ModinfoValues::Description { value: Some(value) } => {
                return_value.insert(value_key, value.to_owned())
            }
            ModinfoValues::DisplayName { value: Some(value) } => {
                return_value.insert(value_key, value.to_owned())
            }
            ModinfoValues::Name { value: Some(value) } => {
                return_value.insert(value_key, value.to_owned())
            }
            ModinfoValues::Website { value: Some(value) } => {
                return_value.insert(value_key, value.to_owned())
            }
            ModinfoValues::Version {
                value: Some(value),
                compat,
            } => {
                return_value.insert(value_key, value.to_owned());
                match compat {
                    Some(value) => return_value.insert(String::from("compat"), value.to_owned()),
                    None => None,
                }
            }
            _ => return_value.insert(value_key, String::new()),
        };

        return_value
    }

    pub fn write(&self) -> Result<(), ModinfoError> {
        let filename = format!("{}.new", self.meta.path.display());
        fs::write(filename, self.to_string()).unwrap();

        Ok(())
    }
}

fn parse_attributes(input: attributes::Attributes) -> HashMap<String, String> {
    let mut attributes = HashMap::new();

    input.map(|a| a.unwrap()).for_each(|a| {
        let key: String = String::from_utf8_lossy(a.key.as_ref()).to_lowercase();
        let value = String::from_utf8(a.value.into_owned()).unwrap();

        attributes.insert(key, value);
    });

    attributes
}

pub fn parse(file: &Path) -> Result<Modinfo, ModinfoError> {
    let mut modinfo = match Path::try_exists(file) {
        Ok(true) => Modinfo::from_string(fs::read_to_string(file)?),
        Ok(false) => return Err(ModinfoError::FsNotFound),
        Err(err) => return Err(ModinfoError::IoError(err)),
    };

    // store the original file path in the metadata
    modinfo.meta.path = file;

    Ok(modinfo)
}
