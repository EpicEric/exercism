use std::{collections::HashMap, sync::mpsc, thread};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let (tx, rx) = mpsc::channel();
    if !input.is_empty() {
        thread::scope(|scope| {
            input
                .chunks(1.max(input.len() / worker_count))
                .for_each(|chunk| {
                    let tx = tx.clone();
                    scope.spawn(move || {
                        let mut map: HashMap<char, usize> = HashMap::new();
                        for line in chunk {
                            line.to_lowercase()
                                .chars()
                                .filter(|c| c.clone().is_alphabetic())
                                .for_each(|c| {
                                    let e = map.entry(c).or_default();
                                    *e += 1;
                                });
                        }
                        tx.send(map).unwrap();
                    });
                });
        });
    }
    drop(tx);
    match rx.iter().reduce(|mut acc, rx_map| {
        rx_map.keys().for_each(|key| {
            let e = acc.entry(*key).or_default();
            *e += rx_map.get(key).unwrap();
        });
        acc
    }) {
        None => HashMap::with_capacity(0),
        Some(map) => map,
    }
}
