use std::arch::x86_64::_bittest;
use std::fs::{self, DirEntry};
use std::fs::File;
use std::env;
use std::hash::Hash;
use std::io::{Write};
use std::collections::{HashMap,HashSet};
use serde_json::Result;
use colored::Colorize;

#[derive(Clone)]
struct Collection {
    docs: Vec<Doc>,
    stop_words: Option<HashSet<String>>
}

impl Collection  {

    fn construct(dir_path: &&String) -> Self
    {
        let file_list = fs::read_dir(dir_path).unwrap();
        let mut docs = Vec::new();

        for file in file_list { 
            match file {
                Ok(d) => {
                    let p = d.path().into_os_string().into_string().unwrap();
                    let text = fs::read_to_string(p.clone());
                    let tokenized = text
                            .unwrap()
                            .split_whitespace()
                            .map(|s| s.to_string())
                            .collect();
                    let doc = Doc { 
                        path: p.clone(),
                        terms: tokenized,
                    };
                    docs.insert(0, doc)
                },
                Err(error) => panic!("Problem geting file list: {:?}", error)
            };
        }
        Self {
            docs: docs,
            stop_words: Some(HashSet::new())
        }

    }

    fn derive_term_frequencies(&self) -> HashMap<String, (String, f32)> {
        let mut hm: HashMap<String, (String, f32)> = HashMap::new();
        let sw = self.stop_words.as_ref().unwrap();
        for doc in &self.docs {
            for term in &doc.terms {
                if sw.contains(term) {
                    continue
                }
                if let Some(val) = hm.get_mut(term) {
                    val.1 += 1.0;
                } else {
                    hm.insert(term.clone(), (doc.clone().path, 0.0));
                }
            }
        }
        hm
    }

    fn add_stop_words(mut self, stop_words: HashSet<&str>) -> Self
    {
        let mut current_stop_words = self.stop_words.take().unwrap_or_else(HashSet::new);
        current_stop_words.extend(stop_words.iter().map(|s| s.to_string()));
        self.stop_words = Some(current_stop_words);
        self
    }

}

#[derive(Clone)]
struct Doc {
    path: String,
    terms: Vec<String>,
}

// fn tokens_from_file(path: &String, stop_words: HashSet<&str>) -> Result<Vec<String>> {
//     let tokens = fs::read_to_string(path)
//         .expect("Unable to read file")
//         .split_whitespace()
//         .filter(|t| !stop_words.contains(t))
//         .map(|s| s.to_string())
//         .collect();
//     return Ok(tokens)
// }

// fn write_to_file(path: &str, data: String) {
//     let mut file = File::create(&path)
//         .expect("Unable to create file. Does index directory exist or have correct permissions?");
//     file.write_all(data.as_bytes())
//         .expect("Unable to write index file");
// }

fn main() -> std::io::Result<()> {

    // Arg Parsing
    let args: Vec<String> = env::args().collect();
    let posts_dir = &args.get(1)
        .expect("Posts dir not specified");

    // Output file destination
    let mut output_file = String::new();
    output_file.push_str("index/main.json");

    // Building a collection of documents
    let mut c = Collection::construct(&posts_dir);

    // Setting stop words
    c = c.add_stop_words(HashSet::from(["a","the","an","#","is"]));

    let tf_map = c.derive_term_frequencies();
    
    for (key, value)in tf_map.iter() {
        println!("{}, {}", format!("{}",key).green(),value.1);
    }

    let index: HashMap<String, Vec<(String,f32)>>;

    // let docs: Vec<_> = dir_ls.collect();

    // let stop_words: HashSet<&str> = HashSet::from(["a","the","an","#","is"]);

    // for doc in docs {

    //     let path = doc?.path().into_os_string().into_string().unwrap();

    //     let token_list = tokens_from_file(&path, stop_words.clone()).unwrap();

    //     let mut count = HashMap::new(); 

    //     let mut tfMap = HashMap::new();

    //     for mut token in token_list.clone() {

    //         token = token.to_lowercase();

            // if token already in count += 1
            // if let Some(c) = count.get_mut(&token) {
            //     *c += 1;
            // } else {
            //     count.insert(token.clone(), 1);
            // }

    //         // tf
    //         // # of times a token appears in document d, divided by all tokens in document
    //         let tf: f32 = (*count.get(&token).unwrap() as f32) / (count.len() as f32);
    //         //let tf: f32 = <i16 as TryInto<f32>>::try_into(*count.get(&token).unwrap()).unwrap() / <usize as TryInto<f32>>::try_into(count.len()).unwrap();

    //         tfMap.insert(token, tf);

    //     }

    //     let mut idfMap = HashMap::new();

    //     for token in token_list.clone() {

    //         let docs_with_token = inverted_index.get(token).unwrap().len();

    //         let idf: f32 = (total_doc_count as f32 / docs_with_token as f32).log(10.0);

    //         idf_scores.insert(token.clone(), idf);

    //     }

    //     let mut final_index = inverted_index.clone();
        
    //     for mut token in inverted_index.keys() {
    //         for post in inverted_index.get("key")
    //             .expect("Problem accessing token in index").keys() {

    //                 let f = final_index.get_mut(token)
    //                             .expect("Problem getting token of final index")
    //                             .get_mut(post)
    //                             .expect("Problem getting post of final index");
                    
    //                 let tfidf = f.get("tf").unwrap() * idf_scores.get(token).unwrap();

    //                 f.insert(String::from("tfidf"), tfidf);

    //             }
    //     }

    //     // multipl the tf and idf for each token to derive the tfidf score

        

    // }
    
    // write_to_file(&output_file,serde_json::to_string_pretty(&inverted_index)
    //     .expect("Problem converting to json"));
    
    return Ok(())
}
