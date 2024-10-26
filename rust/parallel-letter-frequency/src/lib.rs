use std::{collections::HashMap, thread};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if !input.is_empty() {
        match thread::scope(|scope| {
            let join_handles: Vec<_> = input
                .chunks(1.max(input.len() / worker_count))
                .map(|chunk| {
                    let join_handle = scope.spawn(move || {
                        let mut map: HashMap<char, usize> = HashMap::new();
                        for line in chunk {
                            line.to_lowercase()
                                .chars()
                                .filter(|c| c.is_alphabetic())
                                .for_each(|c| {
                                    let e = map.entry(c).or_default();
                                    *e += 1;
                                });
                        }
                        map
                    });
                    join_handle
                })
                .collect();
            join_handles
                .into_iter()
                .map(|jh| jh.join().unwrap())
                .reduce(|mut acc, rx_map| {
                    rx_map.keys().for_each(|key| {
                        let e = acc.entry(*key).or_default();
                        *e += rx_map.get(key).unwrap();
                    });
                    acc
                })
        }) {
            Some(map) => map,
            None => HashMap::with_capacity(0),
        }
    } else {
        HashMap::with_capacity(0)
    }
}
