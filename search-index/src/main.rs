use std::fs;
use std::fs::File;
use std::env;
use std::io::Split;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Grab posts from posts dir
    let paths = fs::read_dir(&args[1]).unwrap();

    for path in paths {

        let path_str = path.unwrap().path().display().to_string();
        let data = fs::read_to_string(&path_str).expect("Unable to read file");
        let tokens = str::split_whitespace(&data);
        println!("Tokens for :{}",path_str);
        for token in tokens {
            println!("{}", token)
        }

    }
    // Convert into tokens
    // Write to a flat file in a serverable folder
}
