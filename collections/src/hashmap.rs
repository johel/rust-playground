use std::collections::HashMap;

pub fn words_mapping(text: &str) -> HashMap< &str, i32> {

    // let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    return map;
}


pub fn get_from_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);  // returns and optional

    match score {
        Some(hey) => println!("hey {}", hey),
        None => ()
    }
}
