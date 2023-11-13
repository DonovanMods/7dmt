use convert_case::{Case, Casing};
use quick_xml::{events::*, reader::Reader, writer::Writer};
use semver::{BuildMetadata, Prerelease, Version};
use std::{
    borrow::Cow,
    collections::HashMap,
    error, fmt, fs,
    io::Cursor,
    path::{Path, PathBuf},
    str::FromStr,
};

// Include Modules
mod impls;
pub use impls::*;

mod version_tools;
pub use version_tools::*;

// Include tests
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum ModinfoError<'m> {
    IoError(std::io::Error),
    InvalidVersion(lenient_semver_parser::Error<'m>),
    FsNotFound,
    NoModinfo,
    NoModinfoAuthor,
    NoModinfoDescription,
    NoModinfoDisplayName,
    NoModinfoName,
    NoModinfoVersion,
    NoModinfoValueVersion,
    NoModinfoVersionCompat,
    NoModinfoWebsite,
    UnknownTag(String),
    WriteError,
    XMLError(quick_xml::Error),
}

impl<'m> error::Error for ModinfoError<'m> {}
impl<'m> fmt::Display for ModinfoError<'m> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModinfoError::IoError(err) => write!(f, "I/O error occurred: {}", err),
            ModinfoError::InvalidVersion(err) => write!(f, "Invalid version: {}", err),
            ModinfoError::FsNotFound => write!(f, "File not found"),
            ModinfoError::NoModinfo => write!(f, "No modinfo.xml found"),
            ModinfoError::NoModinfoAuthor => write!(f, "No Author found in modinfo.xml"),
            ModinfoError::NoModinfoDescription => write!(f, "No Description found in modinfo.xml"),
            ModinfoError::NoModinfoDisplayName => {
                write!(f, "No DisplayName found in modinfo.xml")
            }
            ModinfoError::NoModinfoName => write!(f, "No Name found in modinfo.xml"),
            ModinfoError::NoModinfoVersion => write!(f, "No Version found in modinfo.xml"),
            ModinfoError::NoModinfoValueVersion => {
                write!(f, "No Version value found in modinfo.xml")
            }
            ModinfoError::NoModinfoVersionCompat => {
                write!(f, "No Version compat found in modinfo.xml")
            }
            ModinfoError::NoModinfoWebsite => write!(f, "No Website found in modinfo.xml"),
            ModinfoError::UnknownTag(err) => write!(f, "{}", err),
            ModinfoError::WriteError => write!(f, "Could not write modinfo.xml"),
            ModinfoError::XMLError(err) => write!(f, "XML error occurred: {}", err),
        }
    }
}
impl<'m> From<std::io::Error> for ModinfoError<'m> {
    fn from(err: std::io::Error) -> Self {
        ModinfoError::IoError(err)
    }
}
impl<'m> From<quick_xml::Error> for ModinfoError<'m> {
    fn from(err: quick_xml::Error) -> Self {
        ModinfoError::XMLError(err)
    }
}

impl<'l> From<lenient_semver_parser::Error<'l>> for ModinfoError<'l> {
    fn from(err: lenient_semver_parser::Error<'l>) -> Self {
        ModinfoError::InvalidVersion(err)
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
pub enum ModinfoVersion {
    V1,
    V2,
}

#[derive(Debug)]
struct ModinfoValueMeta {
    version: ModinfoVersion,
    path: PathBuf,
}

impl Default for ModinfoValueMeta {
    fn default() -> Self {
        ModinfoValueMeta {
            version: ModinfoVersion::V1,
            path: PathBuf::new(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct ModinfoValue {
    value: Option<String>,
}

impl fmt::Display for ModinfoValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value {
            Some(ref value) => write!(f, "{}", value),
            None => write!(f, ""),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ModinfoValueVersion {
    value: Version,
    compat: Option<String>,
}

impl fmt::Display for ModinfoValueVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let version = &self.value.to_string();
        let compat = match &self.compat {
            Some(ref value) => value.to_string(),
            None => String::new(),
        };

        if compat.is_empty() {
            write!(f, "{}", version)
        } else {
            write!(f, "{} ({})", version, compat)
        }
    }
}

impl Default for ModinfoValueVersion {
    fn default() -> Self {
        ModinfoValueVersion {
            value: Version::new(0, 1, 0),
            compat: None,
        }
    }
}

#[derive(Debug, Default)]
pub struct Modinfo {
    author: ModinfoValue,
    description: ModinfoValue,
    display_name: ModinfoValue,
    name: ModinfoValue,
    version: ModinfoValueVersion,
    website: ModinfoValue,
    meta: ModinfoValueMeta,
}

impl ToString for Modinfo {
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
            let value = match field {
                "version" => self.get_version().to_string(),
                _ => match self.get_value_for(field) {
                    Some(value) => value.to_string(),
                    None => String::new(),
                },
            };

            elem.push_attribute(attributes::Attribute {
                key: quick_xml::name::QName(b"value"),
                value: Cow::from(value.clone().into_bytes()),
            });

            if field == "version" && self.version.compat.is_some() {
                elem.push_attribute(attributes::Attribute {
                    key: quick_xml::name::QName(b"compat"),
                    value: Cow::from(self.version.compat.as_ref().unwrap().as_bytes()),
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

impl FromStr for Modinfo {
    type Err = ModinfoError<'static>;

    fn from_str(xml: &str) -> Result<Self, Self::Err> {
        let mut modinfo = Modinfo::default();
        let mut buf: Vec<u8> = Vec::new();
        let mut reader = Reader::from_str(xml);
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
                        b"Author" => modinfo.author = ModinfoValue { value: Some(value) },
                        b"Description" => modinfo.description = ModinfoValue { value: Some(value) },
                        b"DisplayName" => {
                            modinfo.display_name = ModinfoValue { value: Some(value) }
                        }
                        b"Name" => modinfo.name = ModinfoValue { value: Some(value) },
                        b"Version" => {
                            let mut compat = None;

                            if attributes.contains_key("compat") {
                                compat = Some(attributes["compat"].clone());
                            }
                            modinfo.version = ModinfoValueVersion {
                                value: match lenient_semver::parse_into::<Version>(&value) {
                                    Ok(result) => result.clone(),
                                    Err(err) => lenient_semver::parse_into::<Version>(
                                        format!("0.0.0+{}", err).as_ref(),
                                    )
                                    .unwrap(),
                                },
                                compat,
                            }
                        }
                        b"Website" => modinfo.website = ModinfoValue { value: Some(value) },
                        _ => (),
                    }
                }
                Ok(_) => (),
            }

            buf.clear();
        }

        Ok(modinfo)
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

pub fn parse<'m>(file: PathBuf) -> Result<Modinfo, ModinfoError<'m>> {
    let modinfo = match Path::try_exists(&file) {
        Ok(true) => Modinfo::from_str(fs::read_to_string(&file)?.as_ref()),
        Ok(false) => return Err(ModinfoError::FsNotFound),
        Err(err) => return Err(ModinfoError::IoError(err)),
    };

    match modinfo {
        Ok(mut modinfo) => {
            if modinfo.author.value.is_none() {
                return Err(ModinfoError::NoModinfoAuthor);
            }
            if modinfo.description.value.is_none() {
                return Err(ModinfoError::NoModinfoDescription);
            }
            if modinfo.display_name.value.is_none() {
                return Err(ModinfoError::NoModinfoDisplayName);
            }
            if modinfo.name.value.is_none() {
                return Err(ModinfoError::NoModinfoName);
            }
            if modinfo.version.value.to_string().is_empty() {
                return Err(ModinfoError::NoModinfoValueVersion);
            }
            if modinfo.version.compat.is_none() {
                return Err(ModinfoError::NoModinfoVersionCompat);
            }
            if modinfo.website.value.is_none() {
                return Err(ModinfoError::NoModinfoWebsite);
            }

            // store the original file path in the metadata
            modinfo.meta.path = file;

            Ok(modinfo)
        }
        Err(err) => Err(err),
    }
}
