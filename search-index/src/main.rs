use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Grab posts from posts dir
    let paths = fs::read_dir(&args[1]).unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    // Convert into tokens
    // Write to a flat file in a serverable folder
}
