use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !matches!(nucleotide, 'G' | 'A' | 'T' | 'C') {
        return Err(nucleotide);
    }
    let mut count = 0;
    for char in dna.chars() {
        match char {
            char if char == nucleotide => count += 1,
            'G' | 'A' | 'T' | 'C' => (),
            _ => return Err(char),
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map: HashMap<char, usize> = [('G', 0), ('A', 0), ('T', 0), ('C', 0)]
        .into_iter()
        .collect();
    for char in dna.chars() {
        let entry = map.get_mut(&char).ok_or(char)?;
        *entry += 1;
    }
    Ok(map)
}
