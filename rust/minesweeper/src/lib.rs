fn count_neighboring_mines(minefield: &[&str], i: usize, j: usize) -> usize {
    [
        if i == 0 || j == 0 {
            None
        } else {
            minefield
                .get(i - 1)
                .and_then(|line| line.as_bytes().get(j - 1))
        },
        if i == 0 {
            None
        } else {
            minefield.get(i - 1).and_then(|line| line.as_bytes().get(j))
        },
        if i == 0 {
            None
        } else {
            minefield
                .get(i - 1)
                .and_then(|line| line.as_bytes().get(j + 1))
        },
        if j == 0 {
            None
        } else {
            minefield.get(i).and_then(|line| line.as_bytes().get(j - 1))
        },
        minefield.get(i).and_then(|line| line.as_bytes().get(j + 1)),
        if j == 0 {
            None
        } else {
            minefield
                .get(i + 1)
                .and_then(|line| line.as_bytes().get(j - 1))
        },
        minefield.get(i + 1).and_then(|line| line.as_bytes().get(j)),
        minefield
            .get(i + 1)
            .and_then(|line| line.as_bytes().get(j + 1)),
    ]
    .into_iter()
    .filter(|neighbor| neighbor.map_or(false, |&n| n == b'*'))
    .count()
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(i, &line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(j, &cell)| {
                    if cell == b' ' {
                        match count_neighboring_mines(minefield, i, j) {
                            0 => ' ',
                            num => char::from_digit(num as u32, 10).unwrap(),
                        }
                    } else {
                        '*'
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}
