use std::env;
use std::fs;

fn main() {
    let list:Vec<String> = env::args().collect();
    let word_to_search = &list[1];
    let filename = &list[2];
    let contents = fs::read_to_string(filename)
        .expect("error reading the file");

    println!("searching for: {:?}", word_to_search);
    println!("in file: {:?}", filename);
    println!("contents: \n {}", contents);
}
