/*
XMLIFY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// We import the CLI
/// entity from my library, Cleasy.
use cleasy::App;

/// Importing an extra 
/// standard trait.
use std::fmt::Debug;

/// Importing the "file_is"
/// from the "Coutils" crate.
use coutils::file_is;

/// Importing the "read_file"
/// from the "Coutils" crate.
use coutils::read_file;

/// Importing the "get_index"
/// from the "Coutils" crate.
use coutils::get_index;

/// Importing the "PartialEq"
/// trait.
use std::cmp::PartialEq;

/// Importing the "clean_split"
/// from the "Coutils" crate.
use coutils::clean_split;

/// Importing the "create_file"
/// from the "Coutils" crate.
use coutils::create_file;

/// Importing the "write_to_file"
/// from the "Coutils" crate.
use coutils::write_to_file;

/// A data type to
/// represent an utterance.
pub struct Utterance {
    pub speaker: String,
    pub speech: String,
    pub line_number: usize,
}


/// Implements methods
/// for the "Utterance"
/// entity.
impl Utterance{

    /// Creates a new
    /// instance of this entity.
    pub fn new(
        speaker: &String,
        speech: &String,
        line_number: &usize,
    ) -> Utterance {
        return Utterance {
            speaker: speaker.to_owned(),
            speech: speech.to_owned(),
            line_number: line_number.to_owned()
        }
    }

    /// An XML representation of this
    /// entity.
    pub fn to_xml(&self) -> String {
        return format!(
            "<Utterance speaker=\"{}\" id=\"{}\">\n\t<speech>{}</speech>\n</Utterance>",
            &self.speaker,
            &self.line_number,
            &self.speech
        );
    }

    /// A string representation of this
    /// entity.
    pub fn to_string(&self) -> String {
        return format!(
            "\nLine: {}\nSpeaker: {}\nUtterance: {}\n", 
            &self.line_number,
            &self.speaker, 
            &self.speech
        );
    }
}

/// Serializes raw transcripts into a list of
/// "Utterance" entities and returns these.
pub fn serialize(text: &String) -> Vec<Utterance> {
    let mut result: Vec<Utterance> = Vec::new();
    let lines: Vec<String> = clean_split(
        &text,
        &String::from("\n")
    );
    for line in &lines {
        if !line.contains(":"){}
        else {
            let split_string: Vec<String> = clean_split(
                &line,
                &String::from(":")
            );
            let index: usize = get_index(&lines, &line) + 1;
            let speaker: &String = &split_string[0];
            let speech: &String = &split_string[1];
            result.push(
                Utterance::new(speaker, speech, &index)
            );
        }
    }
    return result;
}

/// A function to process passed command-line arguments.
pub fn process_args(file: &String) -> () {
    if file_is(file) {
        let contents: String = read_file(&file);
        let name_base: &String = &clean_split(&file, &String::from(".txt"))[0];
        let new_file_name: String = format!("{}.xml", &name_base);
        let utterances: Vec<Utterance> = serialize(&contents);
        let mut xml_string_vec: Vec<String> = Vec::new();
        for utterance in utterances {
            let xml_string = utterance.to_xml();
            xml_string_vec.push(xml_string);
        }
        let xml_string: String = xml_string_vec.join("\n");
        create_file(&new_file_name);
        write_to_file(&new_file_name, &xml_string);
    }
    else {
        println!("File \"{}\" not found. Aborting.", file);
    }
}

/// XMLIFY's tiny-ass CLI.
pub fn cli() -> () {
    let mut xmlify: App = App::new(
        &"XMLIFY",
        &"1.0.0",
        &"Angel Dollface"
    );
    xmlify.add_arg(
        &"inf",
        &" Path to the text file.",
        &"true"
    );
    if xmlify.version_is() == true {
        println!(
            "{}", 
            xmlify.version()
        );
    }
    else if xmlify.help_is() == true {
        println!(
            "{}", 
            xmlify.help()
        );
    }
    else if xmlify.arg_was_used(&"inf") {
        let file: String = xmlify.get_arg_data(&"inf");
        process_args(&file);
    }      
    else {
        println!(
            "{}", 
            xmlify.help()
        );
    }
}

/// Main point of entry for the Rust
/// compiler.
fn main() -> () {
    cli();
}

