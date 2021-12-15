use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
  let sorted_word = slice_to_sorted_string(word);
  let mut result = HashSet::new();

  for el in possible_anagrams {
      let is_different_word = el.to_lowercase() != word.to_lowercase();
      let is_anagram = slice_to_sorted_string(el) == sorted_word;
      if is_different_word && is_anagram {
          result.insert(*el);
      }
  }

  return result;
}


fn slice_to_sorted_string(word: &str) -> String {
  let mut word_chars: Vec<char> = word.to_lowercase().chars().collect();
  word_chars.sort_unstable();
  let sorted_word = String::from_iter(word_chars);
  return sorted_word;
}