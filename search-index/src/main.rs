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
    
    // Building inverted index shell 
    let mut inverted_index: HashMap<String, HashMap<String, i32>> = HashMap::new();

    let stop_words: HashSet<&str> = HashSet::from(["a","the","an","#","is"]);

    let names_from_dir: Vec<Option<&str>> = vec![];

    // Iterating through posts
    for path in names_from_dir.clone() {
        if let Some(current_path) = path {
            let current_path = current_path.to_string();
            let tokens = tokens_from_file(
                &current_path, 
                stop_words.clone()).expect("Problem getting tokens from file");

            // Calculate term frequency (TF) for each term in the document 
            let tf_map: HashMap<String, i32> = tokens.into_iter().fold(HashMap::new(), |mut acc, token | {
                *acc.entry(token).or_insert(0) +=1 ;
                acc
            });

            // Calculate the document frequency (DF) for each term in the document
            let df_map: HashMap<String, i32> = tf_map.keys().fold(HashMap::new(), |mut acc, token| {
                *acc.entry(token.to_lowercase()).or_insert(0) += 1;
                acc
            });

            // Calculate the total number of documents in the collection
            let total_documents = names_from_dir.len() as i32;

            // Calculate the IDF for each term and update the inverted index
            for (token, tf) in tf_map {
                let idf = 1.0 + (total_documents as f64 / (1.0 + df_map[&token.to_lowercase()] as f64)).ln();
                let tf_idf_score = (tf as f64) * idf;

                inverted_index
                    .entry(token.to_lowercase())
                    .or_insert_with(HashMap::new)
                    .insert(current_path.clone(), tf_idf_score as i32);
            }
        }
 
    }
    
    write_to_file(&output_file,serde_json::to_string_pretty(&inverted_index)
        .expect("Problem converting to json"));
    
    return Ok(())
    // Convert into tokens
    // Write to a flat file in a serverable folder
}
