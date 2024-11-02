use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    let mut start: Option<usize> = None;
    let mut iter = words.char_indices().peekable();
    while let Some((i, char)) = iter.next() {
        if start.is_none() && char.is_alphanumeric() {
            start = Some(i);
        } else if start.is_some() && !char.is_alphanumeric() {
            match (char, iter.peek()) {
                ('\'', Some((_, next_char))) if next_char.is_alphanumeric() => (),
                _ => {
                    *map.entry(words[start.take().unwrap()..i].to_lowercase())
                        .or_default() += 1;
                }
            }
        }
    }
    if let Some(start) = start {
        *map.entry(words[start..].to_lowercase()).or_default() += 1;
    }
    dbg!(&map);
    map
}
