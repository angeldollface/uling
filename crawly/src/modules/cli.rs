/*
CRAWLY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// We import the CLI
/// entity from my library, Cleasy.
use cleasy::App;

/// We import the "CleasyError"
/// struct for handling results
/// and errors.
use cleasy::CleasyError;

/// Importing the method to
/// check whether a file exists.
use coutils::file_is;

/// Importing the method to
/// read string data from a 
/// file.
use coutils::read_file;

/// Importing the struct
/// to represent word entities.
use super::search::Word;

/// Importing the function 
/// to create text files.
use coutils::create_file;

/// Importing the function to
/// split strings.
use coutils::clean_split;

/// Importing the function 
/// to write to text files.
use coutils::write_to_file;

/// Importing the function
/// to do the actual crawling.
use super::search::find_word;

/// A function to process the args passed and run a search.
pub fn process_args(file: &String, word: &String, mode: &String) -> () {
    if file_is(file) {
        if mode == &String::from("xml") {
            let contents: String = read_file(&file);
            let name_base: &String = &clean_split(&file, &String::from(".xml"))[0];
            let new_file_name: String = format!("{}_{}.xml", &name_base, word);
            let finding: Word = find_word(&contents, &word, &file);
            create_file(&new_file_name);
            write_to_file(&new_file_name, &finding.to_xml());
        }
        else if mode == &String::from("txt") {
            let contents: String = read_file(&file);
            let name_base: &String = &clean_split(&file, &String::from(".txt"))[0];
            let new_file_name: String = format!("{}_{}.xml", &name_base, word);
            let finding: Word = find_word(&contents, &word, &file);
            create_file(&new_file_name);
            write_to_file(&new_file_name, &finding.to_xml());
        }
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
    crawly.add_arg(
        &"mod",
        &" File type: XML or TXT.",
        &"false"
    );
    if crawly.version_is() == true {
        println!(
            "{}", 
            crawly.version_info()
        );
    }
    else if crawly.help_is() == true {
        println!(
            "{}", 
            crawly.help_info()
        );
    }
    else if crawly.arg_was_used(&"inf") &&  
        crawly.arg_was_used(&"wrd") &&
        crawly.arg_was_used(&"mod") {
        let file: Result<String, CleasyError> = crawly.get_arg_data(&"inf");
        match file {
            Ok(file_path) => {
                let word: Result<String, CleasyError> = crawly.get_arg_data(&"wrd");
                match word {
                    Ok(word_ok) => {
                        let mode: Result<String, CleasyError> = crawly.get_arg_data(&"mod");
                        match mode {
                            Ok(mode_ok) => {
                                process_args(&file_path, &word_ok, &mode_ok);
                            },
                            Err(mode_error) => {
                                println!("{}", mode_error);
                            }
                        }
                    },
                    Err(word_error) => {
                        println!("{}", word_error);
                    }
                }
            },
            Err(file_error) => {
                println!("{}", file_error);
            }
        }
    }
    else if crawly.arg_was_used(&"inf") &&  
        crawly.arg_was_used(&"wrd") {
        let file: Result<String, CleasyError> = crawly.get_arg_data(&"inf");
        match file {
            Ok(file_path) => {
                let word: Result<String, CleasyError> = crawly.get_arg_data(&"wrd");
                match word {
                    Ok(word_ok) => {
                        let default_mode: String = String::from("xml");
                        process_args(&file_path, &word_ok, &default_mode);
                    },
                    Err(word_error) => {
                        println!("{}", word_error);
                    }
                }
            },
            Err(file_error) => {
                println!("{}", file_error);
            }
        }
    }         
    else {
        println!(
            "{}", 
            crawly.help_info()
        );
    }
}
