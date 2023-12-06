use modinfo::Modinfo;
use std::fmt;
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};
// use eyre::eyre;
use glob::glob;

mod modlet_xml;
use modlet_xml::ModletXML;

/// Represents a modlet
#[derive(Debug, Default)]
pub struct Modlet {
    pub path: PathBuf,
    pub modinfo: Modinfo,
    pub xmls: Vec<ModletXML>,
}

impl fmt::Display for Modlet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Modlet {
    pub fn new(path: impl AsRef<Path>) -> eyre::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let mut xmls = Vec::new();
        let modinfo = if path.join("ModInfo.xml").exists() {
            modinfo::parse(path.join("ModInfo.xml"))?
        } else {
            Modinfo::new()
        };
        let glob_pattern = path.join("config/**/*.xml");
        for file in glob(glob_pattern.to_str().unwrap())? {
            let file = file?;
            xmls.push(ModletXML::new(file).load()?);
        }

        Ok(Self { path, modinfo, xmls })
    }

    /// Returns the name of the modlet
    pub fn name(&self) -> Cow<str> {
        self.path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .into()
    }
}
