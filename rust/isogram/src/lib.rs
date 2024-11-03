use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut set = HashSet::new();
    for char in candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
    {
        if set.contains(&char) {
            return false;
        }
        set.insert(char);
    }
    true
}
