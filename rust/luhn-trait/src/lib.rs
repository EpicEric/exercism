use std::{
    fmt::Debug,
    ops::{DivAssign, Rem},
};

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

struct LuhnVec(Vec<u8>);

impl FromIterator<u8> for LuhnVec {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Luhn for LuhnVec {
    fn valid_luhn(&self) -> bool {
        if self.0.len() <= 1 {
            return false;
        }
        let total = self.0.as_slice().chunks(2).fold(0u8, |mut acc, digits| {
            acc += digits[0];
            if let Some(digit) = digits.get(1) {
                let mut digit = *digit;
                digit *= 2;
                if digit > 9 {
                    digit -= 9;
                }
                acc += digit;
            };
            // We only care about the last digit; strip the others
            acc % 10
        });
        total % 10 == 0
    }
}

impl Luhn for Option<LuhnVec> {
    fn valid_luhn(&self) -> bool {
        match self {
            None => false,
            Some(vec) => vec.valid_luhn(),
        }
    }
}

impl<'a> Luhn for &'a str {
    fn valid_luhn(&self) -> bool {
        let digits: Option<LuhnVec> = self
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_digit(10))
            .map(|d| d.map(|d2| d2 as u8))
            .rev()
            .collect();
        digits.valid_luhn()
    }
}

impl Luhn for String {
    fn valid_luhn(&self) -> bool {
        self.as_str().valid_luhn()
    }
}

trait DigitsNumber:
    Rem<Self, Output = Self>
    + PartialEq<Self>
    + Copy
    + DivAssign<Self>
    + TryInto<u8, Error: Debug>
    + From<u8>
{
}

struct DigitsIter<T: DigitsNumber> {
    source: T,
}

impl<T> Iterator for DigitsIter<T>
where
    T: DigitsNumber,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.source == 0.into() {
            return None;
        }
        let digit: u8 = (self.source % 10u8.into()).try_into().unwrap();
        self.source /= 10u8.into();
        Some(digit)
    }
}

impl DigitsNumber for u8 {}
impl DigitsNumber for u16 {}
impl DigitsNumber for u32 {}
impl DigitsNumber for u64 {}
impl DigitsNumber for usize {}

impl<T> Luhn for T
where
    T: DigitsNumber,
{
    fn valid_luhn(&self) -> bool {
        LuhnVec(DigitsIter { source: *self }.collect::<Vec<u8>>()).valid_luhn()
    }
}
