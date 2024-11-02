use std::{fs, str};

use anyhow::Error;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
#[derive(Debug, Default)]
pub struct Flags {
    prepend: bool,
    output_names: bool,
    case_insensitive: bool,
    invert: bool,
    entire_line: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut config = Flags {
            ..Default::default()
        };
        for &flag in flags {
            match flag {
                "-n" => config.prepend = true,
                "-l" => config.output_names = true,
                "-i" => config.case_insensitive = true,
                "-v" => config.invert = true,
                "-x" => config.entire_line = true,
                _ => (),
            }
        }
        config
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut data = Vec::new();
    for filename in files {
        let file = fs::read(filename)?;
        let file_data: &str = str::from_utf8(file.as_slice())?;
        for (num, line) in file_data.lines().enumerate() {
            let condition = if flags.entire_line {
                if flags.case_insensitive {
                    line.to_lowercase() == pattern.to_lowercase()
                } else {
                    line == pattern
                }
            } else if flags.case_insensitive {
                line.to_lowercase().contains(&pattern.to_lowercase())
            } else {
                line.contains(pattern)
            };
            let condition = if flags.invert { !condition } else { condition };
            if condition {
                let result = if flags.output_names {
                    data.push(filename.to_string());
                    break;
                } else if flags.prepend {
                    format!("{}:{}", num + 1, line)
                } else {
                    line.to_owned()
                };
                if files.len() > 1 {
                    data.push(format!("{}:{}", filename, result));
                } else {
                    data.push(result);
                }
            }
        }
    }
    Ok(data)
}
