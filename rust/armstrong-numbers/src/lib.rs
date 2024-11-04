pub fn is_armstrong_number(num: u32) -> bool {
    let string = num.to_string();
    string.chars().fold(0u32, |acc, x| {
        acc + x.to_digit(10).unwrap().pow(string.len() as u32)
    }) == num
}
