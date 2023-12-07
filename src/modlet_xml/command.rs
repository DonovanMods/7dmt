use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use std::{
    fmt::{Display, Formatter},
    io::Write,
};

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CsvInstruction {
    Add(char),
    Remove(char),
}

#[derive(Debug, Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct InstructionSet {
    pub attribute: Option<String>,
    pub csv_op: Option<CsvInstruction>,
    pub values: Vec<String>,
    pub xpath: Option<String>,
}

impl InstructionSet {
    pub fn new() -> Self {
        Self::default()
    }
}

// Modlet types that require additional lines to be added after the Start event
pub const COLLECTION_COMMANDS: [&str; 3] = ["append", "insert_after", "insert_before"];
// Modlet types that require additional TEXT lines added
pub const TEXT_COMMANDS: [&str; 3] = ["csv", "set", "set_attribute"];
// Modlet types that are empty tags
pub const EMPTY_COMMANDS: [&str; 2] = ["remove", "remove_attribute"];

/// Represents a modlet command instruction
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
    pub fn from_str(cmd: impl AsRef<[u8]>) -> Self {
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

    pub fn set(&mut self, instruction_set: InstructionSet) -> Self {
        match self {
            Command::Append(_) => Self::Append(instruction_set),
            Command::Comment(_) => Self::Comment(instruction_set.values.join("")),
            Command::Csv(_) => Self::Csv(instruction_set),
            Command::InsertAfter(_) => Self::InsertAfter(instruction_set),
            Command::InsertBefore(_) => Self::InsertBefore(instruction_set),
            Command::NoOp => Self::NoOp,
            Command::Remove(_) => Self::Remove(instruction_set),
            Command::RemoveAttribute(_) => Self::RemoveAttribute(instruction_set),
            Command::Set(_) => Self::Set(instruction_set),
            Command::SetAttribute(_) => Self::SetAttribute(instruction_set),
            Command::StartTag(_) => Self::StartTag(None),
            Command::Unknown => Self::Unknown,
        }
    }

    pub fn write(&self, writer: &mut quick_xml::Writer<impl Write>) -> eyre::Result<()> {
        match self {
            Command::Append(_) => (),
            Command::Comment(comment) => {
                writer.write_event(Event::Comment(BytesText::new(comment)))?;
            }
            Command::Csv(_) => (),
            Command::InsertAfter(_) => (),
            Command::InsertBefore(_) => (),
            Command::NoOp => (),
            Command::Remove(_) => (),
            Command::RemoveAttribute(_) => (),
            Command::Set(_) => (),
            Command::SetAttribute(_) => (),
            Command::StartTag(_) => (),
            Command::Unknown => (),
        }

        Ok(())
    }
}
