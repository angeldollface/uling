/// Importing an extra 
/// standard trait.
use std::fmt::Debug;

/// Importing the method
/// to split strings from
/// the "Coutils" crate.
use coutils::clean_split;

/// Importing Serde's "Serialize"
/// trait.
use serde_derive::Serialize;

/// Importing Serde's "Deserialize"
/// trait.
use serde_derive::Deserialize;

/// Importing the "from_str" Serde's XML
/// handler API.
use serde_xml_rs::de::from_str;

/// An entity to represent speech
/// by a speaker.
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Utterance {
    pub speaker: String,
    pub id: usize,
    #[serde(rename="$value")]
    pub speech: String
}

/// Method(s) for the "Utterance"
/// entity.
impl Utterance {

    /// Returns a string representation
    /// of the "Utterance" entity.
    pub fn to_string(&self) -> String {
        return format!(
            "ID: {}\nSpeaker: {}\nSpeech: {}",
            &self.id,
            &self.speaker,
            &self.speech
        );
    }

}

/// Deserializes an utterance from XML code into the "Utterance"
/// entity.
pub fn deserialize_utterance(
    text: &String
) -> Utterance {
    let result: Utterance = from_str(text).unwrap();
    return result;
}

/// Deserializes multiple utterances from XML strings into a vector
/// of "Utterance" entities.
pub fn deserialize_utterances(
    text: &String
) -> Vec<Utterance> {
    let lines: Vec<String> = clean_split(&text, &String::from("\n"));
    let mut utterances: Vec<Utterance> = Vec::new();
    for line in lines {
        let utterance: Utterance = deserialize_utterance(&line);
        utterances.push(utterance);
    }
    return utterances;
}