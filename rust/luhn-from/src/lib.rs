use std::{
    fmt::Debug,
    ops::{DivAssign, Rem},
};

pub struct Luhn {
    digits: Option<Vec<u8>>,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let digits = match &self.digits {
            None => return false,
            Some(digits) => digits,
        };
        if digits.len() <= 1 {
            return false;
        }
        let total = digits.as_slice().chunks(2).fold(0u8, |mut acc, digits| {
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

impl<'a> From<&'a str> for Luhn {
    fn from(input: &'a str) -> Self {
        let digits: Option<Vec<_>> = input
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_digit(10))
            .map(|d| d.map(|d2| d2 as u8))
            .rev()
            .collect();
        Luhn { digits }
    }
}

impl From<String> for Luhn {
    fn from(input: String) -> Self {
        Luhn::from(input.as_ref())
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

impl<T> From<T> for Luhn
where
    T: DigitsNumber,
{
    fn from(value: T) -> Self {
        Luhn {
            digits: Some(DigitsIter { source: value }.collect()),
        }
    }
}

impl DigitsNumber for u8 {}
impl DigitsNumber for u16 {}
impl DigitsNumber for u32 {}
impl DigitsNumber for u64 {}
impl DigitsNumber for usize {}
