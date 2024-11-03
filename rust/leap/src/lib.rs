pub fn is_leap_year(year: u64) -> bool {
    match (year % 400, year % 100, year % 4) {
        (0, _, _) => true,
        (_, 0, _) => false,
        (_, _, x) => x == 0,
    }
}
