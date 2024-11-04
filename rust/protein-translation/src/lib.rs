pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut vec = Vec::new();
    for chunk in rna.as_bytes().chunks(3) {
        vec.push(match chunk {
            b"AUG" => "Methionine",
            b"UUU" | b"UUC" => "Phenylalanine",
            b"UUA" | b"UUG" => "Leucine",
            b"UCU" | b"UCC" | b"UCA" | b"UCG" => "Serine",
            b"UAU" | b"UAC" => "Tyrosine",
            b"UGU" | b"UGC" => "Cysteine",
            b"UGG" => "Tryptophan",
            b"UAA" | b"UAG" | b"UGA" => return Some(vec),
            _ => return None,
        })
    }
    Some(vec)
}
