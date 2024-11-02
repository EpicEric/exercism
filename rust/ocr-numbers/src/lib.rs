// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use std::iter::zip;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

fn recognize_number(input: (&[u8], &[u8], &[u8], &[u8])) -> char {
    match input {
        (b" _ ", b"| |", b"|_|", b"   ") => '0',
        (b"   ", b"  |", b"  |", b"   ") => '1',
        (b" _ ", b" _|", b"|_ ", b"   ") => '2',
        (b" _ ", b" _|", b" _|", b"   ") => '3',
        (b"   ", b"|_|", b"  |", b"   ") => '4',
        (b" _ ", b"|_ ", b" _|", b"   ") => '5',
        (b" _ ", b"|_ ", b"|_|", b"   ") => '6',
        (b" _ ", b"  |", b"  |", b"   ") => '7',
        (b" _ ", b"|_|", b"|_|", b"   ") => '8',
        (b" _ ", b"|_|", b" _|", b"   ") => '9',
        _ => '?',
    }
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() || lines.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(lines.len()));
    }
    if lines[0].is_empty() || lines[0].len() % 3 != 0 {
        return Err(Error::InvalidColumnCount(lines[0].len()));
    }
    let digit_lines: Vec<String> = lines
        .chunks(4)
        .map(|l| {
            let &[l1, l2, l3, l4] = l else { unreachable!() };
            zip(
                zip(l1.as_bytes().chunks(3), l2.as_bytes().chunks(3)),
                zip(l3.as_bytes().chunks(3), l4.as_bytes().chunks(3)),
            )
            .map(|((s1, s2), (s3, s4))| recognize_number((s1, s2, s3, s4)))
            .collect::<String>()
        })
        .collect();
    let mut output = String::new();
    let mut is_first = true;
    for digit_line in digit_lines {
        if !is_first {
            output.push(',');
        }
        output.push_str(&digit_line);
        is_first = false;
    }
    Ok(output)
}
