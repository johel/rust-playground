const APOSTROPHE: &'static str = "'";

fn word_acronym(word: &str) -> String {
    let parts = word
        .split(|c: char| c.is_lowercase())
        .filter(|c| !c.is_empty() && c.to_string() != APOSTROPHE)
        .fold(String::new(), |acc, curr| format!("{}{}", acc, curr));

    let first_letter = &word[0..1];
    let all_letters_upper = parts.len() == word.len();
    if all_letters_upper {
        first_letter.to_string()
    } else if first_letter.contains(|a: char| a.is_lowercase()) {
        format!("{}{}", first_letter.to_uppercase(), parts)
    } else {
        parts
    }
}

pub fn abbreviate(phrase: &str) -> String {
    let words = phrase.split(|c: char| !c.is_alphanumeric() && c.to_string() != APOSTROPHE);
    let result = words
        .filter(|word| word.len() > 0)
        .map(|word| word_acronym(word))
        .fold(String::new(), |acc, curr| format!("{}{}", acc, curr));

    return result;
}
