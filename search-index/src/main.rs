use std::fs::{self, DirEntry};
use std::fs::File;
use std::env;
use std::hash::Hash;
use std::io::{Split,Write};
use std::io::prelude::*;
use std::str::{self, SplitWhitespace};
use std::collections::{HashMap,HashSet};
use serde::{Serialize, Deserialize};
use serde_json::Result;

fn tokens_from_file(path: Result<DirEntry, Error>, stop_words: HashSet<&str>) -> Result<SplitWhitespace, Error> {

    let path_str = path?.path().display().to_string();
    let file_data = fs::read_to_string(&path_str).expect("Unable to read file");
    let tokens = str::split_whitespace(&file_data);
    tokens.filter(|token| !stop_words.contains(token)).collect();
    Ok((tokens))
}

fn write_index_to_file() {

}

fn index_as_json(s: SplitWhitespace) -> Result<String, Error>{

}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let names_from_dir = fs::read_dir(&args[1])
        .expect("Problem getting post names from directory");
    
    let mut inverted_index = HashMap::new();

    for path in names_from_dir {
        tokens = tokens_from_file(path, stop_words=HashSet::from(["a","the","an","#"]))
        inverted_index.insert(token.to_string(), path.to_string());
    }

    write_index_to_file(index_as_json)

    let json = serde_json::to_string_pretty(&inverted_index);
    let mut output_file = String::new();
    output_file.push_str(&args[2]);
    output_file.push_str("index.json");
    let mut file = File::create(&output_file).unwrap();
    file.write_all(json.unwrap().as_bytes());
    Ok(())
    // Convert into tokens
    // Write to a flat file in a serverable folder
}
