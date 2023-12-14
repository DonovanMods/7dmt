use convert_case::{Case, Casing};
use quick_xml::events::{BytesText, Event};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
    io::Write,
    str::from_utf8,
};

// Modlet types that require additional lines to be added after the Start event
pub const COLLECTION_COMMANDS: [&str; 3] = ["append", "insertafter", "insertbefore"];
// Modlet types that are empty tags
pub const EMPTY_COMMANDS: [&str; 2] = ["remove", "removeattribute"];
// Modlet types that require additional TEXT lines added
pub const TEXT_COMMANDS: [&str; 3] = ["csv", "set", "setattribute"];

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CsvInstruction {
    Add(char),
    Remove(char),
}

impl CsvInstruction {
    pub fn delim(&self) -> &char {
        match self {
            CsvInstruction::Add(delim) => delim,
            CsvInstruction::Remove(delim) => delim,
        }
    }

    pub fn op(&self) -> &str {
        match self {
            CsvInstruction::Add(_) => "add",
            CsvInstruction::Remove(_) => "remove",
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct InstructionSet {
    pub attribute: Option<Vec<u8>>,
    pub csv_op: Option<CsvInstruction>,
    pub values: Vec<Event<'static>>,
    pub xpath: Vec<u8>,
}

impl InstructionSet {
    pub fn new() -> Self {
        Self::default()
    }

    fn values_to_strings(&self) -> Vec<String> {
        self.values
            .iter()
            .map(|e| from_utf8(e.to_vec().as_slice()).unwrap_or_default().to_owned())
            .collect()
    }

    fn xpath_attribute(&self) -> (&[u8], &[u8]) {
        (b"xpath".as_ref(), self.xpath.as_slice())
    }
}

/// Represents a modlet command instruction
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Append(InstructionSet),
    Comment(Cow<'static, str>),
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

impl Command {
    pub fn from_str(input_str: &str) -> Self {
        let match_string = input_str.to_case(Case::Flat);
        match match_string.as_str() {
            "append" => Command::Append(InstructionSet::new()),
            "comment" => Command::Comment(Cow::Owned(String::new())),
            "csv" => Command::Csv(InstructionSet::new()),
            "insertafter" => Command::InsertAfter(InstructionSet::new()),
            "insertbefore" => Command::InsertBefore(InstructionSet::new()),
            "noop" => Command::NoOp,
            "remove" => Command::Remove(InstructionSet::new()),
            "removeattribute" => Command::RemoveAttribute(InstructionSet::new()),
            "set" => Command::Set(InstructionSet::new()),
            "setattribute" => Command::SetAttribute(InstructionSet::new()),
            "starttag" => Command::StartTag(None),
            _ => Command::Unknown,
        }
    }

    pub fn set(self, instruction_set: InstructionSet) -> Self {
        match self {
            Command::Append(_) => Self::Append(instruction_set),
            Command::Comment(_) => Self::Comment(Cow::Owned(instruction_set.values_to_strings().join(","))),
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
            Command::Append(is) | Command::InsertAfter(is) | Command::InsertBefore(is) => {
                writer
                    .create_element(&self.to_string())
                    .with_attribute(is.xpath_attribute())
                    .write_inner_content(move |writer| {
                        for event in &is.values {
                            writer.write_event(event)?;
                        }
                        Ok::<(), eyre::Error>(())
                    })?;
            }
            Command::Comment(comment) => {
                let comment = BytesText::from_escaped(comment.clone());
                writer.write_event(Event::Comment(comment))?
            }
            Command::Csv(is) => {
                writer
                    .create_element(&self.to_string())
                    .with_attributes([
                        is.xpath_attribute(),
                        (
                            b"delim".as_ref(),
                            is.csv_op.as_ref().unwrap().delim().to_string().as_bytes(),
                        ),
                        (b"op".as_ref(), is.csv_op.as_ref().unwrap().op().as_bytes()),
                    ])
                    .write_text_content(BytesText::new(is.values_to_strings().join(",").as_ref()))?;
            }
            Command::Remove(is) | Command::RemoveAttribute(is) => {
                writer
                    .create_element(&self.to_string())
                    .with_attribute(is.xpath_attribute())
                    .write_empty()?;
            }
            Command::Set(is) => {
                writer
                    .create_element(&self.to_string())
                    .with_attribute(is.xpath_attribute())
                    .write_text_content(BytesText::new(is.values_to_strings().join(",").as_ref()))?;
            }
            Command::SetAttribute(is) => {
                writer
                    .create_element(&self.to_string())
                    .with_attributes([
                        is.xpath_attribute(),
                        (b"name".as_ref(), is.attribute.as_ref().unwrap().to_vec().as_slice()),
                    ])
                    .write_text_content(BytesText::new(is.values_to_strings().join(",").as_ref()))?;
            }
            Command::StartTag(_) => (),
            _ => (),
        }

        Ok(())
    }
}

impl AsRef<str> for Command {
    fn as_ref(&self) -> &str {
        match self {
            Command::Append(_) => "append",
            Command::Comment(_) => "comment",
            Command::Csv(_) => "csv",
            Command::InsertAfter(_) => "insertafter",
            Command::InsertBefore(_) => "insertbefore",
            Command::NoOp => "noop",
            Command::Remove(_) => "remove",
            Command::RemoveAttribute(_) => "removeattribute",
            Command::Set(_) => "set",
            Command::SetAttribute(_) => "setattribute",
            Command::StartTag(_) => "starttag",
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
            Command::InsertAfter(_) => write!(f, "insertAfter"),
            Command::InsertBefore(_) => write!(f, "insertBefore"),
            Command::NoOp => write!(f, "no_op"),
            Command::Remove(_) => write!(f, "remove"),
            Command::RemoveAttribute(_) => write!(f, "removeAttribute"),
            Command::Set(_) => write!(f, "set"),
            Command::SetAttribute(_) => write!(f, "setAttribute"),
            Command::StartTag(_) => write!(f, "start_tag"),
            Command::Unknown => write!(f, "unknown"),
        }
    }
}
