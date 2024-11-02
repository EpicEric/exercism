use std::{cmp::Ordering, collections::HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Corner {
    x: usize,
    y: usize,
}

impl PartialOrd for Corner {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Corner {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Equal => (),
            ord => return ord,
        }
        self.x.cmp(&other.x)
    }
}

fn is_corner(char: char) -> bool {
    char == '+'
}

fn is_horizontal_line_segment(char: char) -> bool {
    matches!(char, '+' | '-')
}

fn is_vertical_line_segment(char: char) -> bool {
    matches!(char, '+' | '|')
}

fn is_rectangle(lines: &[&str], top_left_corner: &Corner, bottom_right_corner: &Corner) -> bool {
    // Top line
    lines[top_left_corner.y][top_left_corner.x..bottom_right_corner.x + 1]
        .chars()
        .all(is_horizontal_line_segment)
        // Bottom line
        && lines[bottom_right_corner.y][top_left_corner.x..bottom_right_corner.x + 1]
            .chars()
            .all(is_horizontal_line_segment)
        // Left line
        && lines[top_left_corner.y..bottom_right_corner.y + 1]
            .iter()
            .map(|line| line.chars().nth(top_left_corner.x).unwrap())
            .all(is_vertical_line_segment)
        // Right line
        && lines[top_left_corner.y..bottom_right_corner.y + 1]
            .iter()
            .map(|line| line.chars().nth(bottom_right_corner.x).unwrap())
            .all(is_vertical_line_segment)
}

pub fn count(lines: &[&str]) -> u32 {
    if lines.is_empty() {
        return 0;
    }
    let mut set: HashSet<Corner> = HashSet::new();
    let mut corners: Vec<Corner> = (0..lines.len())
        .flat_map(|j| {
            lines[j].char_indices().filter_map(move |(i, char)| {
                if is_corner(char) {
                    Some(Corner { x: i, y: j })
                } else {
                    None
                }
            })
        })
        .inspect(|corner| {
            set.insert(*corner);
        })
        .collect();
    corners.sort();
    if corners.len() <= 3 {
        return 0;
    }
    let mut sum = 0;
    for (index, top_left_corner) in corners[..corners.len() - 1].iter().enumerate() {
        sum += corners[index..]
            .iter()
            .filter(|corner| corner.x > top_left_corner.x && corner.y > top_left_corner.y)
            .filter(|bottom_right_corner| is_rectangle(lines, top_left_corner, bottom_right_corner))
            .count()
    }
    sum as u32
}
