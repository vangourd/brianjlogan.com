use std::fs::{self};
use std::fs::File;
use std::env;
use std::io::{Write};
use std::collections::{HashMap,HashSet};
use serde_json::Result;
use tfidf::{TfIdf, TfIdfDefault, tf};

fn tokens_from_file(path: &String, stop_words: HashSet<&str>) -> Result<Vec<String>> {
    let tokens = fs::read_to_string(path)
        .expect("Unable to read file")
        .split_whitespace()
        .filter(|t| !stop_words.contains(t))
        .map(|s| s.to_string())
        .collect();
    return Ok(tokens)
}

fn write_to_file(path: &str, data: String) {
    let mut file = File::create(&path)
        .expect("Unable to create file. Does index directory exist or have correct permissions?");
    file.write_all(data.as_bytes())
        .expect("Unable to write index file");
}

fn main() -> std::io::Result<()> {
    // Arg Parsing
    let args: Vec<String> = env::args().collect();
    let posts_dir = &args.get(1)
        .expect("Posts dir not specified");
    // Output file destination
    let mut output_file = String::new();
    output_file.push_str("index/main.json");
    // Collecting file names of posts
    let names_from_dir = fs::read_dir(&posts_dir)
        .expect("Problem getting post names from directory");
    
    // Building inverted index shell 
    let mut inverted_index = HashMap::new();

    // Iterating through posts
    for path in names_from_dir {

        let current_path = path.unwrap().path().display().to_string();

        // Grabbing tokens from file ommitting stop words
        let tokens = tokens_from_file(
            &current_path,
            HashSet::from(["a","the","an","#","is"])
        );
        // TF-IDF Calculation
        // # of times term t appears in doc d
        // collection, doc, term
        // idf(t)
            // a = 1 + # of all docs
            // b = 1 + # of all documents containing term t
        // tf
            // how many times t appears in doc d
        // Inserting tokens into index
        for token in &tokens.expect("Problem getting tokens from file"){
            let tf = &tokens.expect("Problem getting tokens from file")
                .clone()
                .into_iter()
                .fold(String::new(),|acc, s| {
                    if s == token.to_string() {
                        acc + &s
                    } else {
                        acc
                    }
                });
            // Build a hashmap
            let mut hm = HashMap::new();
            let score = 0;
            hm.insert(current_path.clone(), score);
            inverted_index.insert(token.to_lowercase(),hm);
            
        }
    }
    
    write_to_file(&output_file,serde_json::to_string_pretty(&inverted_index)
        .expect("Problem converting to json"));
    
    return Ok(())
    // Convert into tokens
    // Write to a flat file in a serverable folder
}
