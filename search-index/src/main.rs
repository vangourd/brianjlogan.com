use std::fs;
use std::fs::File;
use std::env;
use std::io::Split;
use std::str;
use std::collections::{HashMap,HashSet};

fn main() {
    let args: Vec<String> = env::args().collect();
    // Grab posts from posts dir
    let paths = fs::read_dir(&args[1]).unwrap();
    let mut inverted_index = HashMap::new();

    for path in paths {

        let path_str = path.unwrap().path().display().to_string();
        let data = fs::read_to_string(&path_str).expect("Unable to read file");
        // Split into tokens for indexing
        let tokens = str::split_whitespace(&data);
        println!("Tokens for :{}",&path_str);
        let stop_words = HashSet::from(["a","the","an"]);
        let new_path_str = path_str.clone();
        for token in tokens {
            if stop_words.contains(token) {
                continue
            }
            inverted_index.insert(token.to_string(), new_path_str.to_string());
        }

    }

    println!("{:?}", inverted_index)
    // Convert into tokens
    // Write to a flat file in a serverable folder
}
