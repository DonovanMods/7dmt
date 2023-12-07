use glob::glob;
use modinfo::Modinfo;
use std::fmt;
use std::{
    borrow::Cow,
    io::Write,
    path::{Path, PathBuf},
};

mod modlet_xml;
use modlet_xml::ModletXML;

/// Represents a modlet
#[derive(Debug, Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

    pub fn files(&self) -> Vec<Cow<Path>> {
        let mut files = Vec::new();
        for xml in &self.xmls {
            files.push(xml.filename());
        }
        files
    }

    /// Returns the name of the modlet
    pub fn name(&self) -> Cow<str> {
        self.path.file_name().unwrap_or_default().to_str().unwrap().into()
    }

    pub fn write(&self, writer: &mut quick_xml::Writer<impl Write>) -> eyre::Result<()> {
        self.xmls.iter().try_for_each(|xml| xml.write(writer))?;

        Ok(())
    }
}
