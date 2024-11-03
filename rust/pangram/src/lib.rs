/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut characters: Vec<char> = sentence
        .chars()
        .filter_map(|char| {
            if char.is_ascii_alphabetic() {
                Some(char.to_ascii_lowercase())
            } else {
                None
            }
        })
        .collect();
    characters.sort();
    let mut string = String::with_capacity(characters.len());
    string.extend(characters);
    let mut latest_character = 96;
    for byte in string.as_bytes() {
        if byte.saturating_sub(latest_character) == 1 {
            latest_character = *byte;
        }
    }
    latest_character == 122
}

// https://exercism.org/tracks/rust/exercises/pangram/solutions/joohan
pub fn objectively_better_is_pangram(sentence: &str) -> bool {
    let s = sentence.to_lowercase();
    ('a'..='z').all(|b| s.contains(b))
}
