use std::collections::{HashMap, HashSet};

fn count_letters(word: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    word.to_lowercase()
        .chars()
        .for_each(|c| *map.entry(c).or_insert(0) += 1);
    map
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word_letters = count_letters(word);
    possible_anagrams
        .iter()
        .filter(|&anagram| anagram.to_lowercase() != word.to_lowercase())
        .filter(|&anagram| {
            let anagram_letters = count_letters(anagram);
            if word_letters.len() == anagram_letters.len() {
                word_letters
                    .keys()
                    .all(|k| word_letters.get(k) == anagram_letters.get(k))
            } else {
                false
            }
        })
        .copied()
        .collect()
}
