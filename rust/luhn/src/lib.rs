/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.is_empty() {
        return false;
    }
    let numbers: Option<Vec<_>> = code
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10))
        .rev()
        .collect();
    let Some(numbers) = numbers else {
        return false;
    };
    if numbers.len() <= 1 {
        return false;
    }
    let total = numbers.as_slice().chunks(2).fold(0u32, |mut acc, digits| {
        acc += digits[0];
        match digits.get(1) {
            Some(digit) => {
                let mut digit = *digit;
                digit *= 2;
                if digit > 9 {
                    digit -= 9;
                }
                acc += digit;
            }
            None => (),
        };
        acc
    });
    total % 10 == 0
}
