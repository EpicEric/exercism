use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut set = HashSet::new();
    'c: for c in (sum / 3)..(sum / 2) {
        for b in (((sum - c) / 2)..c).rev() {
            let a = sum - c - b;
            if c * c == a * a + b * b {
                set.insert([a, b, c]);
                continue 'c;
            }
        }
    }
    set
}
