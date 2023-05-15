/*
CRAWLY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the "get_index"
/// method from the "Coutils"
/// crate.
use coutils::get_index;

/// Importing the "clean_split"
/// method from the "Coutils"
/// crate.
use coutils::clean_split;

/// Importing Rust's map API.
use std::collections::HashMap;

/// A struct to represent 
/// words as entites.
pub struct Word {    
    pub file_name: String,
    pub word: String,
    pub count: usize,
    pub line_info: HashMap<usize, String>
}

/// Implementing methods
/// for the "Word" entity.
impl Word {

    /// A function to instantiate
    /// the "Word" entity.
    pub fn new(
        file_name: &String,
        word: &String,
        count: &usize,
        line_info: &HashMap<usize, String>
    ) -> Word {
        return Word {
            file_name: file_name.to_owned(),
            word: word.to_owned(),
            count: count.to_owned(),
            line_info: line_info.to_owned()
        };
    }

    /// A function to return a string
    /// representation of the "Word" entity.
    pub fn to_string(&self) -> String {
        return format!(
            "File: {}\nWord: {}\nCount: {:?}\nLines: {:?}\n",
            &self.file_name,
            &self.word,
            &self.count,
            &self.line_info
        );
    }

    /// A function to return an XML
    /// representation of the "Word" entity.
    pub fn to_xml(&self) -> String {
        let mut line_info_vec: Vec<String> = Vec::new();
        let mut result: String = String::from("");
        if self.line_info.is_empty() {
            result = format!(
                "<Word entity=\"{}\">\n<File>{}</File>\n<Count>{:?}</Count>\n</Word>",
                &self.word,
                &self.file_name,
                &self.count
            );
        }
        else {
            for (k,v) in self.line_info.clone().into_iter() {
                let xml_string: String = format!("<Line number=\"{}\">{}</Line>", k, v);
                line_info_vec.push(xml_string);
            }
            let line_info_xml: String = line_info_vec.join("\n");
            result = format!(
                "<Word entity=\"{}\">\n<File>{}</File>\n<Count>{:?}</Count>\n{}\n</Word>",
                &self.word,
                &self.file_name,
                &self.count,
                line_info_xml
            );
        }
        return result;        
    }
}

/// Finds a certain word
/// in a string. Returns
/// the "Word" entity.
pub fn find_word(
    text: &String, 
    word: &String, 
    file_name: &String
) -> Word {
    let lines: Vec<String> = clean_split(
        text,
        &String::from("\n")
    );
    let mut occurence_count: usize = 0;
    let mut line_info_map: HashMap<usize, String> = HashMap::new();
    for line in &lines {
        if line.contains(word) {
            occurence_count = occurence_count + 1;
            let index: usize = get_index(&lines, &line);
            line_info_map.insert(index, line.to_string());
        }
        else {}
    }
    let result: Word = Word::new(
        file_name,
        word,
        &occurence_count,
        &line_info_map
    );
    return result;
} 