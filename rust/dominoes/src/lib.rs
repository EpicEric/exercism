use std::collections::BTreeSet;

#[derive(Clone)]
struct Chain {
    matching_pip: u8,
    current: Vec<ChainNode>,
    remaining: BTreeSet<usize>,
}

#[derive(Copy, Clone)]
enum ChainNode {
    RightWay(usize),
    Flipped(usize),
}

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.is_empty() {
        return Some(Vec::new());
    }
    let (last_pip, matching_pip) = input[0];
    // Create a starting chain that uses the first domino in its original orientation
    let mut possibilities: Vec<Chain> = vec![Chain {
        matching_pip,
        current: vec![ChainNode::RightWay(0)],
        remaining: (1..input.len()).collect(),
    }];
    for _ in 1..input.len() {
        possibilities = possibilities
            .into_iter()
            .flat_map(|possibility| {
                possibility
                    .remaining
                    .iter()
                    // Return Some((usize, ChainNode, u8)) if i is a valid next domino
                    // Some((index in original input, orientation, value of the leftover pips))
                    .filter_map(|&i| {
                        // Check both orientations of the possible next piece
                        if input[i].0 == possibility.matching_pip {
                            Some((i, ChainNode::RightWay(i), input[i].1))
                        } else if input[i].1 == possibility.matching_pip {
                            Some((i, ChainNode::Flipped(i), input[i].0))
                        } else {
                            None
                        }
                    })
                    // Map each possibility to a new Chain
                    .map(|(i, node, matching_pip)| {
                        let mut current = possibility.current.clone();
                        let mut remaining = possibility.remaining.clone();
                        current.push(node);
                        remaining.remove(&i);
                        Chain {
                            matching_pip,
                            current,
                            remaining,
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
    possibilities
        .iter()
        // For any full chains, make sure that the last pip matches the remaining one, otherwise invalid
        .find(|Chain { matching_pip, .. }| *matching_pip == last_pip)
        .map(|Chain { current, .. }| {
            current
                .iter()
                .map(|node| match node {
                    ChainNode::RightWay(i) => input[*i],
                    ChainNode::Flipped(i) => {
                        let (a, b) = input[*i];
                        (b, a)
                    }
                })
                .collect()
        })
}
