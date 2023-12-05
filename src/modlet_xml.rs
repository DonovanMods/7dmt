use eyre::{eyre, Result};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use core::panic;
use std::{
    borrow::Cow,
    collections::VecDeque,
    fmt::{Display, Formatter},
    path::{Path, PathBuf},
    str::{self},
};

#[derive(Clone, Debug, PartialEq)]
pub enum CsvOp {
    Add,
    Remove,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct InstructionSet {
    attribute: Option<String>,
    delim: Option<String>,
    values: Vec<String>,
    op: Option<CsvOp>,
    pub xpath: Option<String>,
}

impl InstructionSet {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Modlet types that require additional lines to be added after the Start event
const COLLECTION_MODLETS: [&str; 3] = [
    "append",
    "insert_after",
    "insert_before",
];

const TEXT_MODLETS: [&str; 3] = [
    "csv",
    "set",
    "set_attribute",
];

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Append(InstructionSet),
    Comment(String),
    Csv(InstructionSet),
    InsertAfter(InstructionSet),
    InsertBefore(InstructionSet),
    NoOp,
    Remove(InstructionSet),
    RemoveAttribute(InstructionSet),
    Set(InstructionSet),
    SetAttribute(InstructionSet),
    StartTag(Option<String>),
    Unknown,
}

impl AsRef<str> for Command {
    fn as_ref(&self) -> &str {
        match self {
            Command::Append(_) => "append",
            Command::Comment(_) => "comment",
            Command::Csv(_) => "csv",
            Command::InsertAfter(_) => "insert_after",
            Command::InsertBefore(_) => "insert_before",
            Command::NoOp => "no_op",
            Command::Remove(_) => "remove",
            Command::RemoveAttribute(_) => "remove_attribute",
            Command::Set(_) => "set",
            Command::SetAttribute(_) => "set_attribute",
            Command::StartTag(_) => "start_tag",
            Command::Unknown => "unknown",
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Append(_) => write!(f, "append"),
            Command::Comment(_) => write!(f, "comment"),
            Command::Csv(_) => write!(f, "csv"),
            Command::InsertAfter(_) => write!(f, "insert_after"),
            Command::InsertBefore(_) => write!(f, "insert_before"),
            Command::NoOp => write!(f, "no_op"),
            Command::Remove(_) => write!(f, "remove"),
            Command::RemoveAttribute(_) => write!(f, "remove_attribute"),
            Command::Set(_) => write!(f, "set"),
            Command::SetAttribute(_) => write!(f, "set_attribute"),
            Command::StartTag(_) => write!(f, "start_tag"),
            Command::Unknown => write!(f, "unknown"),
        }
    }
}

impl Command {
    fn from_str(cmd: impl AsRef<[u8]>) -> Self {
        match cmd.as_ref() {
            b"append" => Command::Append(InstructionSet::new()),
            b"comment" => Command::Comment(String::new()),
            b"csv" => Command::Csv(InstructionSet::new()),
            b"insert_after" => Command::InsertAfter(InstructionSet::new()),
            b"insert_before" => Command::InsertBefore(InstructionSet::new()),
            b"no_op" => Command::NoOp,
            b"remove" => Command::Remove(InstructionSet::new()),
            b"remove_attribute" => Command::RemoveAttribute(InstructionSet::new()),
            b"set" => Command::Set(InstructionSet::new()),
            b"set_attribute" => Command::SetAttribute(InstructionSet::new()),
            b"start_tag" => Command::StartTag(None),
            _ => Command::Unknown,
        }
    }

    fn set(&self, modlet: InstructionSet) -> Self {
        match self {
            Command::Append(_) => Self::Append(modlet),
            Command::Comment(_) => Self::Comment(modlet.values.join("")),
            Command::Csv(_) => Self::Csv(modlet),
            Command::InsertAfter(_) => Self::InsertAfter(modlet),
            Command::InsertBefore(_) => Self::InsertBefore(modlet),
            Command::NoOp => Self::NoOp,
            Command::Remove(_) => Self::Remove(modlet),
            Command::RemoveAttribute(_) => Self::RemoveAttribute(modlet),
            Command::Set(_) => Self::Set(modlet),
            Command::SetAttribute(_) => Self::SetAttribute(modlet),
            Command::StartTag(_) => Self::StartTag(None),
            Command::Unknown => Self::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct ModletXML {
    pub commands: Vec<Command>,
    pub path: PathBuf,
}

impl ModletXML {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        if !path.as_ref().exists() {
            return Err(eyre!("Modlet XML {}: file not found", path.as_ref().display()));
        }
        let path = path.as_ref().to_path_buf();
        let commands = load_xml(path.as_ref())?;

        Ok(Self { commands, path })
    }

    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            commands: Vec::new(),
        }
    }

    pub fn filename(&self) -> Cow<str> {
        self.path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .into()
    }
}

fn load_xml(path: &Path) -> Result<Vec<Command>> {
    let mut commands = Vec::new();
    let mut buf = Vec::new();
    let mut reader = Reader::from_file(path)?;
    let mut stack = VecDeque::<Command>::new();
    // The modlet we're building
    let mut modlet = InstructionSet::new();
    let mut start_tag = String::new();
    reader.trim_text(true);

    loop {
        let last_command = stack.get(0).unwrap_or(&Command::NoOp).as_ref();

        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),

            // Found a comment
            Ok(Event::Comment(e)) => {
                commands.push(Command::Comment(e.unescape().unwrap().trim().to_string()));
            }

            // Found a start tag
            Ok(Event::Start(e)) => {
                let tag = str::from_utf8(e.name().as_ref()).unwrap().to_owned();
                let mut command = Command::from_str(e.name());

                if start_tag.is_empty() && command.as_ref() == "unknown" && last_command == "no_op" {
                    start_tag = tag.to_string();
                    command = Command::StartTag(Some(tag));
                }

                if COLLECTION_MODLETS.contains(&last_command) {
                    modlet.values.push(str::from_utf8(&e).unwrap().to_string());
                } else if command.as_ref() != "unknown" && command.as_ref() != "no_op" {
                    // println!("[STARTING] tag {:?} ({command})", str::from_utf8(e.name().as_ref()).unwrap());

                    // We don't want to add the start_tag command to the stack
                    if command.as_ref() == "start_tag" {
                        continue;
                    }

                    modlet.xpath = get_attribute(&e, "xpath");
                    modlet.delim = get_attribute(&e, "delim");
                    modlet.op = match get_attribute(&e, "op") {
                        Some(op) => match op.as_str() {
                            "add" => Some(CsvOp::Add),
                            "remove" => Some(CsvOp::Remove),
                            _ => None,
                        },
                        None => None,
                    };
                    stack.push_back(command);
                }
            }

            // This is an empty tag (likely remove or remove_attribute)
            Ok(Event::Empty(e)) => {
                let value = str::from_utf8(e.as_ref()).unwrap().to_string();

                if COLLECTION_MODLETS.contains(&last_command) {
                    modlet.values.push(value);
                } else {
                    panic!("Unhandled empty tag received: {value}");
                }
            }

            // Found text between tags, add it to our struct's value.
            Ok(Event::Text(e)) => {
                let value = str::from_utf8(&e).unwrap().to_string();

                if TEXT_MODLETS.contains(&last_command) {
                    modlet.values.push(value);
                } else {
                    panic!("Unhandled text tag received: {value}");
                }
            }

            // Found an end tag
            Ok(Event::End(e)) => {
                let tag = str::from_utf8(e.as_ref()).unwrap();
                let mut command = Command::from_str(tag);

                if command.as_ref() == "unknown" && !start_tag.is_empty(){
                    command = Command::StartTag(Some(start_tag.clone()));
                }

                if COLLECTION_MODLETS.contains(&last_command) && command.as_ref() != last_command {
                    modlet.values.push(str::from_utf8(&e).unwrap().to_string());
                } else {
                    // println!("[ENDING] tag {tag} ({command}) / {last_command}");

                    commands.push(command.set(modlet));
                    stack.clear();
                    modlet = InstructionSet::new();
                }
            }

            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,

            // Something unexpected happened. Panic and exit.
            Ok(e) => {
                panic!("[UNKNOWN] event: {:?}", e.as_ref());
            }
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }

    // dbg!(&commands);

    Ok(commands)
}

fn get_attribute(e: &quick_xml::events::BytesStart, attr: &str) -> Option<String> {
    for attribute in e.attributes() {
        let attribute = attribute.unwrap();
        if str::from_utf8(attribute.key.as_ref()) == Ok(attr) {
            return Some(str::from_utf8(attribute.value.as_ref()).unwrap().to_owned());
        }
    }

    None
}
