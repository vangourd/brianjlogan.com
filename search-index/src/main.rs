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

fn tokens_from_file(path: DirEntry, stop_words: HashSet<&str>) -> Result<Vec<String>> {
    let path_str = path.path().display().to_string();
    let tokens = fs::read_to_string(&path_str)
        .expect("Unable to read file")
        .split_whitespace()
        .filter(|t| stop_words.contains(t))
        .map(|s| s.to_string())
        .collect();
    return Ok(tokens)
}

fn write_to_file(path: &str, data: String,) {
    let mut file = File::create(&path).unwrap();
    file.write_all(data.as_bytes());
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut output_file = String::new();
    output_file.push_str(&args[2]);
    output_file.push_str("index.json");

    let names_from_dir = fs::read_dir(&args[1])
        .expect("Problem getting post names from directory");
    
    let mut inverted_index = HashMap::new();

    for path in names_from_dir {
        let tokens = tokens_from_file(path?, HashSet::from(["a","the","an","#"]));
        for token in tokens {
            inverted_index.insert(token, path.unwrap().path().display().to_string());
        }
    }

    
    write_to_file(&output_file,serde_json::to_string_pretty(&inverted_index)
        .expect("Problem converting to json"));
    
    return Ok(())
    // Convert into tokens
    // Write to a flat file in a serverable folder
}
