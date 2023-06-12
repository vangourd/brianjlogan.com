use std::fs::{self};
use std::fs::File;
use std::env;
use std::io::{Write};
use std::collections::{HashMap,HashSet};
use serde_json::Result;

fn tokens_from_file(path: &String, stop_words: HashSet<&str>) -> Result<Vec<String>> {
    let tokens = fs::read_to_string(path)
        .expect("Unable to read file")
        .split_whitespace()
        .filter(|t| stop_words.contains(t))
        .map(|s| s.to_string())
        .collect();
    return Ok(tokens)
}

fn write_to_file(path: &str, data: String) {
    let mut file = File::create(&path).unwrap();
    file.write_all(data.as_bytes())
        .expect("Unable to write index file");
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let index_dir = &args.get(2)
        .expect("Index dir not specified");
    let posts_dir = &args.get(1)
        .expect("Posts dir not specified");
    let mut output_file = String::new();
  
    output_file.push_str(&index_dir);
    output_file.push_str("index.json");

    let names_from_dir = fs::read_dir(&posts_dir)
        .expect("Problem getting post names from directory");
    
    let mut inverted_index = HashMap::new();

    for path in names_from_dir {

        let current_path = path.unwrap().path().display().to_string();

        let tokens = tokens_from_file(
            &current_path,
            HashSet::from(["a","the","an","#"])
        
        );
        for token in tokens.expect("Problem getting tokens from file"){
            inverted_index.insert(token, current_path.clone());
        }
            
    }

    
    write_to_file(&output_file,serde_json::to_string_pretty(&inverted_index)
        .expect("Problem converting to json"));
    
    return Ok(())
    // Convert into tokens
    // Write to a flat file in a serverable folder
}
