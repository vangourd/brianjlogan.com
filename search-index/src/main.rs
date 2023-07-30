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

    let doc_entries: Vec<_> = dir_ls.collect();

    let total_doc_count = doc_entries.len();

    // Building inverted index shell 
    let mut inverted_index: HashMap<
                                String, // token
                                HashMap<
                                    String, // file
                                    HashMap<
                                        String, // score type
                                        f32     // score
                                    >
                                >   
                            > = HashMap::new();

    let stop_words: HashSet<&str> = HashSet::from(["a","the","an","#","is"]);

    for file in doc_entries {

        let mut hm: HashMap<String, HashMap<String, f32>> = HashMap::new();

        let path = file?.path().into_os_string().into_string().unwrap();

        let token_list = tokens_from_file(&path, stop_words.clone()).unwrap();

        let mut count = HashMap::new(); 

        let mut scores: HashMap<String, f32> = HashMap::new();

        for mut token in token_list.clone() {

            token = token.to_lowercase();

            // if token already in count += 1
            if let Some(c) = count.get_mut(&token) {
                *c += 1;
            } else {
                count.insert(token.clone(), 1);
            }

            // tf
            // # of times a token appears in document d, divided by all tokens in document
            let tf: f32 = (*count.get(&token).unwrap() as f32) / (count.len() as f32);
            //let tf: f32 = <i16 as TryInto<f32>>::try_into(*count.get(&token).unwrap()).unwrap() / <usize as TryInto<f32>>::try_into(count.len()).unwrap();

            scores.insert("tf".to_string(), tf);

            hm.insert(path.clone(), scores.clone());
        
            inverted_index.insert(token.clone(), hm.clone());

        }

        let mut idf_scores = HashMap::new();

        for token in inverted_index.keys() {

            let docs_with_token = inverted_index.get(token).unwrap().len();

            let idf: f32 = (total_doc_count as f32 / docs_with_token as f32).log(10.0);

            idf_scores.insert(token.clone(), idf);

        }

        let mut final_index = inverted_index.clone();
        
        for mut token in inverted_index.keys() {
            for post in inverted_index.get("key")
                .expect("Problem accessing token in index").keys() {

                    let f = final_index.get_mut(token)
                                .expect("Problem getting token of final index")
                                .get_mut(post)
                                .expect("Problem getting post of final index");
                    
                    let tfidf = f.get("tf").unwrap() * idf_scores.get(token).unwrap();

                    f.insert(String::from("tfidf"), tfidf);

                }
        }

        // multipl the tf and idf for each token to derive the tfidf score

        

    }
    
    write_to_file(&output_file,serde_json::to_string_pretty(&inverted_index)
        .expect("Problem converting to json"));
    
    return Ok(())
    // Convert into tokens
    // Write to a flat file in a serverable folder
}
