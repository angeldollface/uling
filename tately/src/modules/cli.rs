/*
TATELY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Imports the "App"
/// entity from the 
/// "Cleasy" crate to help
/// build a CLI.
use cleasy::App;

/// We import the "file_is"
/// method from the "Coutils"
/// crate.
use coutils::file_is;

/// We import the "read_file"
/// method from the "Coutils"
/// crate.
use coutils::read_file;

/// We import the "clean_split"
/// method from the "Coutils"
/// crate.
use coutils::clean_split;

/// We import the "create_file"
/// method from the "Coutils"
/// crate.
use coutils::create_file;

/// We import the "write_to_file"
/// method from the "Coutils"
/// crate.
use coutils::write_to_file;

/// We import the "Utterance"
/// entity from the "utterances"
/// module.
use super::utterances::Utterance;

/// We import the "Annotations"
/// entity from the "annotations"
/// module.
use super::annotations::Annotation;

/// We import the "AnnotatedUtterance"
/// entity from the "annotate"
/// module.
use super::annotate::AnnotatedUtterance;

/// We import the method to annotate utterances
/// from the "annotate" module.
use super::annotate::annotate_utterances;

/// We import the method to deserialize
/// utterances in XML format from the 
/// "utterances" module.
use super::utterances::deserialize_utterances;

/// We import the method to deserialize
/// annotations in XML format from the 
/// "annotations" module.
use super::annotations::deserialize_annotations;

/// A function to process passed command-line arguments.
pub fn process_args(
    serialized_speech_file: &String,
    annotation_xml_file: &String
) -> () {
    if file_is(serialized_speech_file) && file_is(annotation_xml_file){
        let name_base: &String = &clean_split(
            &serialized_speech_file, 
            &String::from(".xml")
        )[0];
        let utterance_contents: String = read_file(serialized_speech_file);
        let annotation_contents: String = read_file(annotation_xml_file);

        let new_file_name: String = format!("{}_annotated.xml", &name_base);
        let utterances: Vec<Utterance> = deserialize_utterances(
            &utterance_contents
        );
        let annotations: Vec<Annotation> = deserialize_annotations(
            &annotation_contents
        );
        let annotated_utterances: Vec<AnnotatedUtterance> = annotate_utterances(
            &utterances,
            &annotations
        );
        let mut code_vec: Vec<String> = Vec::new();
        for annotated_utterance in annotated_utterances {
            code_vec.push(annotated_utterance.to_xml());
        }
        let code: String = code_vec.join("\n");
        create_file(&new_file_name);
        write_to_file(&new_file_name, &code);
    }
    else {
        println!("Files not found. Aborting.");
    }
}

/// Tately's tiny-ass CLI.
pub fn cli() -> () {
    let mut tately: App = App::new(
        &"Tately",
        &"1.0.0",
        &"Angel Dollface"
    );
    tately.add_arg(
        &"sld",
        &" Path to the XML file.",
        &"true"
    );
    tately.add_arg(
        &"ano",
        &" Path to the annotation file.",
        &"true"
    );
    if tately.version_is() == true {
        println!(
            "{}", 
            tately.version()
        );
    }
    else if tately.help_is() == true {
        println!(
            "{}", 
            tately.help()
        );
    }
    else if tately.arg_was_used(&"sld") && 
        tately.arg_was_used(&"ano") {
        let serialized_speech_file: String = tately.get_arg_data(&"sld");
        let annotation_file: String = tately.get_arg_data(&"ano");
        process_args(
            &serialized_speech_file,
            &annotation_file
        );
    }      
    else {
        println!(
            "{}", 
            tately.help()
        );
    }
}