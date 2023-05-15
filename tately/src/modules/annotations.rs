/*
TATELY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

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

/// An entity to represent a word
/// class to be annotated.
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Annotation {
    pub word_type: String,
    #[serde(rename="$value")]
    pub content: String
}

/// Implements methods for the entity above.
impl Annotation {

    /// Returns a string representation of
    /// the "Annotation" entity.
    pub fn to_string(&self) -> String {
        return format!(
            "Type: {}\nMarker: {}",
            &self.word_type,
            &self.content
        );
    }
}

/// Deserializes an annotation from XML code into the "Annotation"
/// entity.
pub fn deserialize_annotation(
    text: &String
) -> Annotation {
    let result: Annotation = from_str(text).unwrap();
    return result;
}

/// Deserializes multiple annotations from XML strings into a vector
/// of "Annotation" entities.
pub fn deserialize_annotations(
    text: &String
) -> Vec<Annotation> {
    let lines: Vec<String> = clean_split(&text, &String::from("\n"));
    let mut annotations: Vec<Annotation> = Vec::new();
    for line in lines {
        let annotation: Annotation = deserialize_annotation(&line);
        annotations.push(annotation);
    }
    return annotations;
}