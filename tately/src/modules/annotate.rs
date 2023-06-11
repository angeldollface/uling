/*
TATELY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// We import the "clean_split"
/// method from the "Coutils"
/// crate.
use coutils::clean_split;

/// We import the "Utterance"
/// entity from the "utterances"
/// module.
use super::utterances::Utterance;

/// We import the "Annotations"
/// entity from the "annotations"
/// module.
use super::annotations::Annotation;

/// We define an entity to hold info
/// about annotated utterances.
#[derive(Clone)]
pub struct AnnotatedUtterance {
    pub speaker: String,
    pub id: usize,
    pub speech: String,
    pub annotations: Vec<AnnotatedWord>
}

/// Implementing methods
/// for the "AnnotatedUtterance"
/// entity.
impl AnnotatedUtterance {

    /// Creates a new instance
    /// of "AnnotatedUtterance".
    pub fn new(
        speaker: &String,
        id: &usize,
        speech: &String,
        annotations: &Vec<AnnotatedWord>
    ) -> AnnotatedUtterance {
        return AnnotatedUtterance {
            speaker: speaker.to_owned(),
            id: id.to_owned(),
            speech: speech.to_owned(),
            annotations: annotations.to_vec()
        }
    }

    /// Creates a string representation
    /// of "AnnotatedUtterance".
    pub fn to_string(&self) -> String {
        let mut string_vec: Vec<String> = Vec::new();
        for annotation in &self.annotations {
            string_vec.push(annotation.to_string());
        }
        return format!(
            "Speaker: {}\nID: {}\nSpeech: {}\nANWords:\n{}",
            &self.speaker,
            &self.id,
            &self.speech,
            string_vec.join("\n")
        );
    }

    /// Creates an XML representation
    /// of "AnnotatedUtterance".
    pub fn to_xml(&self) -> String {
        let mut annotated_speech: String = (&self.speech).to_owned();
        for extracted_annotation in &self.annotations {
            annotated_speech = annotated_speech.replace(
                &extracted_annotation.phrase, 
                &extracted_annotation.to_xml()
            );
        }
        let code: String = format!(
            "<Utterance speaker=\"{}\" id=\"{}\">{}</Utterance>",
            &self.speaker,
            &self.id,
            annotated_speech
        );
        return code;
    }
}

/// We define an entity
/// for words to search.
/// The "AnnotatedUtterance"
/// entity depends on this.
#[derive(Clone)]
pub struct AnnotatedWord {
    pub phrase: String,
    pub word_type: String
}

/// Implementing methods
/// for the "AnnotatedWord"
/// entity.
impl AnnotatedWord {

    /// Creates a new instance
    /// of "AnnotatedWord".
    pub fn new(
        phrase: &String,
        word_type: &String
    ) -> AnnotatedWord {
        return AnnotatedWord {
            phrase: phrase.to_owned(),
            word_type: word_type.to_owned()
        }
    }

    /// Creates a string representation
    /// of "AnnotatedWord".
    pub fn to_string(&self) -> String {
        return format!(
            "\tContent: {}\n\tType: {}\n",
            &self.phrase,
            &self.word_type
        );
    }

     /// Creates an XML representation
    /// of "AnnotatedWord".
    pub fn to_xml(&self) -> String {
        return format!(
            "<{}>{}</{}>",
            &self.word_type,
            &self.phrase,
            &self.word_type
        );
    }
}

/// Annotates a single utterance.
/// Gathers info in the "AnnotatedUtterance"
/// entity and returns said entity.
pub fn annotate_utterance(
    utterance: &Utterance,
    annotations: &Vec<Annotation>
) -> AnnotatedUtterance {
    let mut annotation_vec: Vec<AnnotatedWord> = Vec::new();
    let speech: &String = &utterance.speech;
    let speaker: &String = &utterance.speaker;
    let id: &usize = &utterance.id;
    for annotation in annotations {
        let chars: Vec<String> = clean_split(&speech, &String::from(""));
        let mut im_chars: Vec<String> = Vec::new();
        for im_char in chars {
            im_chars.push(im_char);
            let joined_string: String = im_chars.join("");
            if joined_string.contains(&annotation.content) {
                im_chars = Vec::new();
                let annotated_word: AnnotatedWord = AnnotatedWord::new(
                    &annotation.content,
                    &annotation.word_type
                );
                annotation_vec.push(annotated_word);
            }
            else {}
        }
    }
    let result: AnnotatedUtterance = AnnotatedUtterance::new(
        speaker,
        id,
        speech,
        &annotation_vec
    );
    return result;
}

/// Annotates a whole vector of utterances.
/// Returns a vector of the "AnnotatedUtterance"
/// entity.
pub fn annotate_utterances(
    utterances: &Vec<Utterance>,
    annotations: &Vec<Annotation>
) -> Vec<AnnotatedUtterance> {
    let mut annotated_utterance_vec: Vec<AnnotatedUtterance> = Vec::new();
    for utterance in utterances {
        let annotated_utterance: AnnotatedUtterance = annotate_utterance(
            &utterance, annotations
        );
        annotated_utterance_vec.push(annotated_utterance);
    }
    return annotated_utterance_vec;
}