/// This module contains the implementation of the `ModletXML` struct and related types.
/// The `ModletXML` struct represents an XML file containing modlet instructions.
/// It provides methods for loading the XML file and extracting the commands from it.
use eyre::eyre;
use quick_xml::{events::Event, reader::Reader};
use std::{
    borrow::Cow,
    collections::VecDeque,
    path::{Path, PathBuf},
    str::{self},
};

mod command;
use command::{Command, CsvInstruction, InstructionSet};

#[derive(Debug)]
pub struct ModletXML {
    pub commands: Vec<Command>,
    pub path: PathBuf,
}

impl ModletXML {
    pub fn load(mut self) -> eyre::Result<Self> {
        if !self.path.exists() {
            return Err(eyre!("Modlet XML {}: file not found", self.path.display()));
        }
        self.commands = load_xml(self.path.as_ref())?;

        Ok(self)
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

fn load_xml(path: &Path) -> eyre::Result<Vec<Command>> {
    let mut commands = Vec::new();
    let mut buf = Vec::new();
    let mut reader = Reader::from_file(path)?;
    let mut stack = VecDeque::<Command>::new();
    // The modlet we're building
    let mut instruction = InstructionSet::new();
    let mut start_tag = String::new();
    reader.trim_text(true);

    loop {
        let last_command = stack.get(0).unwrap_or(&Command::NoOp).as_ref();

        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),

            // Found a comment
            Ok(Event::Comment(e)) => {
                let comment = e.unescape().unwrap_or_default().trim().to_string();

                if !comment.is_empty() {
                    commands.push(Command::Comment(comment));
                }
            }

            // Found a start tag
            Ok(Event::Start(e)) => {
                let tag_name = e.name();
                let tag_name = str::from_utf8(tag_name.as_ref())?;
                let mut command = Command::from_str(tag_name);

                if start_tag.is_empty() && command.as_ref() == "unknown" && last_command == "no_op" {
                    start_tag = tag_name.to_string();
                    command = Command::StartTag(Some(tag_name.to_string()));
                }

                if command::COLLECTION_COMMANDS.contains(&last_command) {
                    let tag_string = str::from_utf8(e.as_ref())?;
                    instruction.values.push(tag_string.to_string());
                } else if command.as_ref() != "unknown" && command.as_ref() != "no_op" {
                    // println!("[STARTING] tag {:?} ({command})", str::from_utf8(e.name().as_ref()).unwrap());

                    // We don't want to add the start_tag command to the stack
                    if command.as_ref() == "start_tag" {
                        continue;
                    }

                    let delim: char = get_attribute(&e, "delim")
                        .unwrap_or(String::from(","))
                        .chars()
                        .next()
                        .unwrap();

                    instruction.xpath = get_attribute(&e, "xpath");
                    instruction.csv_op = match get_attribute(&e, "op") {
                        Some(op) => match op.as_str() {
                            "add" => Some(CsvInstruction::Add(delim)),
                            "remove" => Some(CsvInstruction::Remove(delim)),
                            _ => None,
                        },
                        None => None,
                    };
                    stack.push_back(command);
                }
            }

            // This is an empty tag (likely remove or remove_attribute)
            Ok(Event::Empty(e)) => {
                let tag_name = e.name();
                let tag_name = str::from_utf8(tag_name.as_ref())?;
                let value = str::from_utf8(e.as_ref())?;

                if command::EMPTY_COMMANDS.contains(&tag_name) || command::COLLECTION_COMMANDS.contains(&last_command) {
                    instruction.values.push(value.to_string());
                } else {
                    panic!("Unhandled empty tag received: {value}");
                }
            }

            // Found text between tags, add it to our struct's value.
            Ok(Event::Text(e)) => {
                let value = str::from_utf8(&e)?;
                let value = value.to_string();

                if command::TEXT_COMMANDS.contains(&last_command) {
                    instruction.values.push(value);
                } else {
                    panic!("Unhandled text tag received: {value}");
                }
            }

            // Found an end tag
            Ok(Event::End(e)) => {
                let tag = str::from_utf8(e.as_ref())?;
                let mut command = Command::from_str(tag);

                if command.as_ref() == "unknown" && !start_tag.is_empty() {
                    command = Command::StartTag(Some(start_tag.clone()));
                }

                if command::COLLECTION_COMMANDS.contains(&last_command) && command.as_ref() != last_command {
                    instruction.values.push(tag.to_string());
                } else {
                    // println!("[ENDING] tag {tag} ({command}) / {last_command}");

                    commands.push(command.set(instruction));
                    stack.clear();
                    instruction = InstructionSet::new();
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
