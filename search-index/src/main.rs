use std::arch::x86_64::_bittest;
use std::fs::{self};
use std::fs::File;
use std::env;
use std::hash::Hash;
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
    let dir_ls = fs::read_dir(&posts_dir)
        .expect("Problem getting post names from directory");
    // Building inverted index shell 
    let mut inverted_index: HashMap<String, HashMap<String, i32>> = HashMap::new();

    let stop_words: HashSet<&str> = HashSet::from(["a","the","an","#","is"]);

    for file in dir_ls{

        let mut hm: HashMap<String, i32> = HashMap::new();

        let t = String::from("token");

        let score = 0;

        hm.insert(file?.path().into_os_string().into_string().unwrap(), score);
        
        inverted_index.insert(t, hm);

    }
    
    write_to_file(&output_file,serde_json::to_string_pretty(&inverted_index)
        .expect("Problem converting to json"));
    
    return Ok(())
    // Convert into tokens
    // Write to a flat file in a serverable folder
}
