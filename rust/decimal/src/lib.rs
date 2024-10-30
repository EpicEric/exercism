use std::ops::{Add, Mul, Sub};

use num_bigint::{BigInt, Sign};

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Clone, Debug, PartialEq)]
pub struct Decimal {
    /// Absolute value of the mantissa with sign.
    mantissa: BigInt,
    /// Negative exponent in base 10 for the decimal number. This value is 0 if there is no fractional part.
    exponent: usize,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let mut chars = input.chars().peekable();
        let sign = match chars.peek() {
            Some('-') => {
                chars.next();
                Sign::Minus
            }
            Some('+') => {
                chars.next();
                Sign::Plus
            }
            Some(_) => Sign::Plus,
            None => return None,
        };
        let mut digits = vec![];
        let mut exponent = 0;
        let mut has_fractional_part = false;
        for char in chars {
            if char == '.' {
                has_fractional_part = true;
                continue;
            }
            digits.push(char.to_digit(10)? as u8);
            if has_fractional_part {
                exponent += 1;
            }
        }
        Some(
            Decimal {
                mantissa: BigInt::from_radix_be(sign, digits.as_slice(), 10)?,
                exponent,
            }
            .normalize(),
        )
    }

    fn normalize(mut self) -> Self {
        while self.mantissa.clone() % 10 == BigInt::ZERO && self.exponent > 0 {
            self.mantissa /= 10;
            self.exponent -= 1;
        }
        self
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Bigger sign == Bigger number
        match self.mantissa.sign().partial_cmp(&other.mantissa.sign()) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        // Match exponents to compare mantissas
        let mut this = self.clone();
        let mut other = other.clone();
        while this.exponent < other.exponent {
            this.mantissa *= 10;
            this.exponent += 1;
        }
        while other.exponent < this.exponent {
            other.mantissa *= 10;
            other.exponent += 1;
        }
        // Bigger mantissa == Bigger number
        this.mantissa.partial_cmp(&other.mantissa)
    }
}

impl Add for Decimal {
    type Output = Decimal;

    fn add(self, rhs: Self) -> Self::Output {
        // Match exponents to add mantissas
        let mut this = self.clone();
        let mut other = rhs.clone();
        while this.exponent < other.exponent {
            this.mantissa *= 10;
            this.exponent += 1;
        }
        while other.exponent < this.exponent {
            other.mantissa *= 10;
            other.exponent += 1;
        }
        Decimal {
            mantissa: this.mantissa + other.mantissa,
            exponent: this.exponent,
        }
        .normalize()
    }
}

impl Sub for Decimal {
    type Output = Decimal;

    fn sub(self, rhs: Self) -> Self::Output {
        // Match exponents to subtract mantissas
        let mut this = self.clone();
        let mut other = rhs.clone();
        while this.exponent < other.exponent {
            this.mantissa *= 10;
            this.exponent += 1;
        }
        while other.exponent < this.exponent {
            other.mantissa *= 10;
            other.exponent += 1;
        }
        Decimal {
            mantissa: this.mantissa - other.mantissa,
            exponent: this.exponent,
        }
        .normalize()
    }
}

impl Mul for Decimal {
    type Output = Decimal;

    fn mul(self, rhs: Self) -> Self::Output {
        // Clear exponents to multiply mantissas
        let mut this = self.clone();
        let mut other = rhs.clone();
        let mut exponent = 0usize;
        while this.exponent > 0 {
            this.exponent -= 1;
            exponent += 1;
        }
        while other.exponent > 0 {
            other.exponent -= 1;
            exponent += 1;
        }
        Decimal {
            mantissa: this.mantissa * other.mantissa,
            exponent,
        }
        .normalize()
    }
}
