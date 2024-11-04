use std::{
    collections::{BTreeSet, HashSet},
    hash::Hash,
};

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
struct BookGroup {
    books: BTreeSet<u32>,
}

impl From<&BookGroup> for u32 {
    fn from(value: &BookGroup) -> Self {
        match value.books.len() {
            1 => 800,
            2 => 1520,
            3 => 2160,
            4 => 2560,
            5 => 3000,
            _ => panic!("BookGroup must have between 1 and 5 books"),
        }
    }
}

impl Hash for BookGroup {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut vec: Vec<u32> = self.books.iter().copied().collect();
        vec.sort();
        for element in vec {
            state.write_u32(element);
        }
    }
}

fn create_possibilities_tree(
    mut possibility: Vec<BookGroup>,
    new_book: u32,
) -> Vec<Vec<BookGroup>> {
    dbg!(&possibility);
    let mut new_possibilities = HashSet::with_capacity(possibility.len() + 1);
    // Create possibilities of adding this book to existing groups
    for i in 0..possibility.len() {
        let group = &possibility[i];
        let head = &possibility[..i];
        let tail = &possibility[i + 1..];
        if !group.books.contains(&new_book) {
            let mut new_group = group.clone();
            new_group.books.insert(new_book);
            let mut leaf = Vec::new();
            leaf.extend_from_slice(head);
            leaf.push(new_group);
            leaf.extend_from_slice(tail);
            leaf.sort();
            new_possibilities.insert(leaf);
        }
    }
    // Create possibility of adding this book to a new group
    possibility.push(BookGroup {
        books: [new_book].into_iter().collect(),
    });
    possibility.sort();
    new_possibilities.insert(possibility);
    new_possibilities.into_iter().collect()
}

pub fn lowest_price(books: &[u32]) -> u32 {
    books
        .iter()
        .fold(<Vec<Vec<BookGroup>>>::new(), |acc, &book| {
            if acc.is_empty() {
                vec![vec![BookGroup {
                    books: [book].into_iter().collect(),
                }]]
            } else {
                let set: HashSet<Vec<BookGroup>> = acc
                    .into_iter()
                    .flat_map(|possibility| create_possibilities_tree(possibility, book))
                    .collect();
                set.into_iter().collect()
            }
        })
        .into_iter()
        .map(|possibility| possibility.iter().map(<u32>::from).sum())
        .min()
        .unwrap_or(0)
}
