use core::str;

fn translate_word(input: &str) -> String {
    let mut prefix: Vec<char> = Vec::new();
    let mut suffix: Vec<char> = Vec::new();
    if matches!(
        input.as_bytes(),
        &[b'a' | b'e' | b'i' | b'o' | b'u', ..] | &[b'x', b'r', ..] | &[b'y', b't', ..]
    ) {
        suffix = input.chars().collect();
    } else {
        let mut finished_prefix = false;
        let mut iter = input.chars().peekable();
        while let Some(char) = iter.next() {
            if finished_prefix {
                suffix.push(char);
            } else {
                prefix.push(char);
                match (char, iter.peek()) {
                    ('q', Some('u')) => {
                        prefix.push(iter.next().unwrap());
                        finished_prefix = true;
                    }
                    (_, Some('y') | Some('a') | Some('e') | Some('i') | Some('o') | Some('u')) => {
                        finished_prefix = true;
                    }
                    (_, _) => (),
                }
            }
        }
    }
    let mut output = String::new();
    output.extend(suffix.into_iter());
    output.extend(prefix.into_iter());
    output.extend("ay".chars());
    output
}

pub fn translate(input: &str) -> String {
    let mut output = String::new();
    let mut iter = input.chars();
    let mut current_word: Option<String> = None;
    while let Some(char) = iter.next() {
        if char.is_alphabetic() {
            current_word.get_or_insert_with(String::new).push(char);
        } else {
            current_word
                .take()
                .inspect(|word| output.push_str(&translate_word(&word)));
            output.push(char);
        }
    }
    current_word.inspect(|word| output.push_str(&translate_word(&word)));
    output
}
