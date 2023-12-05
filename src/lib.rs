use modinfo::Modinfo;
use std::fmt;
use std::path::{Path, PathBuf};
// use eyre::eyre;
use glob::glob;

mod modlet_xml;
use modlet_xml::ModletXML;

#[derive(Debug, Default)]
pub struct Modlet {
    pub name: String,
    pub path: PathBuf,
    pub modinfo: Modinfo,
    pub xmls: Vec<ModletXML>,
}

impl fmt::Display for Modlet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Modlet {
    pub fn new(path: impl AsRef<Path>) -> eyre::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string();
        let mut xmls = Vec::new();
        let modinfo = if path.join("ModInfo.xml").exists() {
            modinfo::parse(path.join("ModInfo.xml"))?
        } else {
            Modinfo::new()
        };
        let glob_pattern = path.join("config/**/*.xml");
        for file in glob(glob_pattern.to_str().unwrap())? {
            let file = file?;
            xmls.push(ModletXML::load(file)?);
        }

        Ok(Self {
            name,
            path,
            modinfo,
            xmls,
        })
    }
}
