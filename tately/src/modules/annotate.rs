use coutils::get_index;
use coutils::clean_split;
use super::utterances::Utterance;
use super::annotations::Annotation;

pub struct AnnotatedUtterance {
    pub speaker: String,
    pub id: usize,
    pub speech: String,
    pub annotations: Vec<AnnotatedWord>
}

impl AnnotatedUtterance {
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
    pub fn to_xml(&self) -> String {
        let mut annotated_speech: String = (&self.speech).to_owned();
        for an in &self.annotations {
            annotated_speech = annotated_speech.replace(&an.phrase, &an.to_xml());
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

#[derive(Clone)]
pub struct AnnotatedWord {
    pub start_pos: usize,
    pub end_pos: usize,
    pub phrase: String,
    pub word_type: String
}

impl AnnotatedWord {
    pub fn new(
        start_pos: &usize,
        end_pos: &usize,
        phrase: &String,
        word_type: &String
    ) -> AnnotatedWord {
        return AnnotatedWord {
            start_pos: start_pos.to_owned(),
            end_pos: end_pos.to_owned(),
            phrase: phrase.to_owned(),
            word_type: word_type.to_owned()
        }
    }
    pub fn to_string(&self) -> String {
        return format!(
            "\tStart: {:?}\n\tEnd: {:?}\n\tContent: {}\n\tType: {}\n",
            &self.start_pos,
            &self.end_pos,
            &self.phrase,
            &self.word_type
        );
    }
    pub fn to_xml(&self) -> String {
        return format!(
            "<{}>{}</{}>",
            &self.word_type,
            &self.phrase,
            &self.word_type
        );
    }
}

pub fn find_phrase_cols(phrase: &String, subject: &String) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let chars: Vec<String> = clean_split(phrase, &String::from(""));
    let mut count: usize = 0;
    let mut im_chars: Vec<String> = Vec::new();
    for character in &chars {
        count = count + 1;
        im_chars.push(character.to_string());
        let joined_string: String = im_chars.join("");
        if joined_string.contains(subject){
            im_chars = Vec::new();
            result.push(count-subject.len());
            result.push(count);
        }
        else {}
    }
    return result;
}

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
                let start_pos: usize = find_phrase_cols(
                    &speech,
                    &annotation.content
                )[0];
                let end_pos: usize = find_phrase_cols(
                    &speech,
                    &annotation.content
                )[1];
                let annotated_word: AnnotatedWord = AnnotatedWord::new(
                    &start_pos,
                    &end_pos,
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