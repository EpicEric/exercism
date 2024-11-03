use std::fmt::{Display, Formatter, Result, Write};

pub struct Roman {
    thousands: u8,
    hundreds: u8,
    tens: u8,
    ones: u8,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for _ in 0..self.thousands {
            f.write_char('M')?;
        }
        for (value, units_char, five_units_char, ten_units_char) in [
            (self.hundreds, 'C', 'D', 'M'),
            (self.tens, 'X', 'L', 'C'),
            (self.ones, 'I', 'V', 'X'),
        ] {
            match value {
                9 => {
                    f.write_char(units_char)?;
                    f.write_char(ten_units_char)?;
                }
                6..=8 => {
                    f.write_char(five_units_char)?;
                    for _ in 6..=value {
                        f.write_char(units_char)?;
                    }
                }
                5 => f.write_char(five_units_char)?,
                4 => {
                    f.write_char(units_char)?;
                    f.write_char(five_units_char)?;
                }
                0..=3 => {
                    for _ in 0..value {
                        f.write_char(units_char)?;
                    }
                }
                _ => unreachable!(),
            }
        }
        Ok(())
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman {
            thousands: (num / 1000) as u8,
            hundreds: ((num % 1000) / 100) as u8,
            tens: ((num % 100) / 10) as u8,
            ones: (num % 10) as u8,
        }
    }
}
