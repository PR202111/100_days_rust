use std::env; // for getting command line arguments
use std::fs; // for reading files
use serde_json::Value; // for parsing JSON

fn main(){
    let args: Vec<String> = env::args().collect(); // collect command line arguments into a vector
    if args.len() != 2 { // check if the number of arguments is correct
        eprintln!("Usage: {} <file_path>", args[0]);
        return; // print usage message
    }

    let path = &args[1];
    match fs::read_to_string(path){
        Ok(content) => match serde_json::from_str::<Value>(&content) {
            Ok(json) => println!("Parsed JSON: {:#?}", json), // print the parsed JSON in a pretty format
            Err(e) => eprintln!("Error parsing JSON: {}", e), // print error if JSON parsing fails
        },
        Err(e) => eprintln!("Error reading file: {}", e), // print error if file reading fails
    }
}