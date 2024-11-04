/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut iter_isbn = isbn.chars().filter(|c| c.is_ascii_alphanumeric());
    let mut sum = 0u32;
    for factor in (1..=10).rev() {
        match iter_isbn.next() {
            Some('X') if factor == 1 => sum += 10,
            Some(digit @ '0'..='9') => {
                sum += digit.to_digit(10).unwrap() * factor;
            }
            _ => return false,
        }
    }
    sum % 11 == 0 && iter_isbn.next().is_none()
}
