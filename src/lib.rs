use glob::glob;
use modinfo::Modinfo;
use std::fmt;
use std::{
    borrow::Cow,
    fs::{self, File},
    io::{self, prelude::*, Write},
    path::{Path, PathBuf},
};

mod modlet_xml;
use modlet_xml::ModletXML;

const INCLUDE_EXTENSIONS: [&str; 3] = ["xml", "txt", "dll"];

/// Represents a modlet
#[derive(Debug, Clone, PartialEq)]
pub struct Modlet {
    pub files: Option<Vec<PathBuf>>,
    pub modinfo: Modinfo,
    pub path: PathBuf,
    pub xmls: Vec<ModletXML>,
}

impl fmt::Display for Modlet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Modlet {
    pub fn new(path: impl AsRef<Path>) -> eyre::Result<Self> {
        let mut other_files = Vec::new();
        let path = path.as_ref().to_path_buf();
        let mut xmls = Vec::new();
        let modinfo = if path.join("ModInfo.xml").exists() {
            modinfo::parse(path.join("ModInfo.xml"))?
        } else {
            Modinfo::new()
        };
        let glob_pattern = path.join("config/**/*");
        for file in glob(glob_pattern.to_str().unwrap())? {
            let file = file?;
            if file.is_dir() {
                continue;
            }

            let file_extension = file.extension().unwrap_or_default().to_ascii_lowercase();
            if !INCLUDE_EXTENSIONS.contains(&file_extension.to_str().unwrap()) {
                continue;
            }

            if file_extension == "xml" {
                xmls.push(ModletXML::new(file).load()?);
            } else {
                other_files.push(file);
            }
        }

        let files = if other_files.is_empty() {
            None
        } else {
            Some(other_files)
        };

        Ok(Self {
            files,
            modinfo,
            path,
            xmls,
        })
    }

    pub fn xml_files(&self) -> Vec<Cow<Path>> {
        let mut xml_files = Vec::new();
        for xml in &self.xmls {
            xml_files.push(xml.filename());
        }
        xml_files
    }

    /// Returns the name of the modlet
    pub fn name(&self) -> Cow<str> {
        self.path.file_name().unwrap_or_default().to_str().unwrap().into()
    }

    /// Write XML files
    pub fn write_xmls(&self, writer: &mut quick_xml::Writer<impl Write>, filename: &Path) -> eyre::Result<()> {
        self.xmls
            .iter()
            .filter(|xml| *xml.filename() == *filename)
            .try_for_each(|xml| xml.write(writer))?;

        Ok(())
    }

    /// Write non-xml files
    pub fn write_files(&self, destination: &Path) -> eyre::Result<()> {
        if let Some(files) = self.files.as_ref() {
            for file in files {
                let file = file.strip_prefix(&self.path).unwrap();
                let src = self.path.join(file);
                let dst = destination.join(file);
                if !dst.exists() {
                    fs::create_dir_all(dst.parent().unwrap())?;
                    fs::copy(src, dst)?;
                // If the file is a localization file, and we've already copied it from an existing modlet above,
                // strip the header and append the remaining lines to the existing file
                } else if src.file_name().unwrap_or_default().to_ascii_lowercase() == "localization.txt" {
                    let input = File::open(src)?;
                    let reader = io::BufReader::new(input);
                    let mut output = fs::OpenOptions::new().append(true).open(&dst)?;
                    let mut writer = io::BufWriter::new(&mut output);

                    for line in reader.lines().skip(1) {
                        let line = line?;
                        write!(writer, "{}\r\n", line)?; // We always write localization files with CRLF
                    }
                }
            }
        }

        Ok(())
    }
}
