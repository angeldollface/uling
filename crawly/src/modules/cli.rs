/*
CRAWLY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// We import the CLI
/// entity from my library, Cleasy.
use cleasy::App;

/// Importing the struct
/// to represent word entities.
use super::search::Word;

/// Importing the function
/// to do the actual crawling.
use super::search::find_word;

/// Importing the method to
/// check whether a file exists.
use coutils::file_is;

/// Importing the method to
/// read string data from a 
/// file.
use coutils::read_file;

/// Importing the function 
/// to create text files.
use coutils::create_file;

/// Importing the function to
/// split strings.
use coutils::clean_split;

/// Importing the function 
/// to write to text files.
use coutils::write_to_file;

pub fn process_args(file: &String, word: &String) -> () {
    if file_is(file) {
        let contents: String = read_file(&file);
        let name_base: &String = &clean_split(&file, &String::from(".txt"))[0];
        let new_file_name: String = format!("{}.xml", &name_base);
        let finding: Word = find_word(&contents, &word, &file);
        create_file(&new_file_name);
        write_to_file(&new_file_name, &finding.to_xml());
    }
    else {
        println!("File \"{}\" not found. Aborting.", file);
    }
}

/// Crawly's tiny-ass CLI.
pub fn cli() {
    let mut crawly: App = App::new(
        &"Crawly",
        &"1.0.0",
        &"Angel Dollface"
    );
    crawly.add_arg(
        &"inf",
        &" Path to the text file.",
        &"true"
    );
    crawly.add_arg(
        &"wrd",
        &" Word to search for.",
        &"true"
    );
    if crawly.version_is() == true {
        println!(
            "{}", 
            crawly.version()
        );
    }
    else if crawly.help_is() == true {
        println!(
            "{}", 
            crawly.help()
        );
    }
    else if crawly.arg_was_used(&"inf") &&  crawly.arg_was_used(&"wrd") {
        let file: String = crawly.get_arg_data(&"inf");
        let word: String = crawly.get_arg_data(&"wrd");
        process_args(&file, &word);
    }         
    else {
        println!(
            "{}", 
            crawly.help()
        );
    }
}
