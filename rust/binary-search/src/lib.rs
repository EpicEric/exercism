use std::cmp::Ordering;

pub fn find<T, A>(array: A, key: T) -> Option<usize>
where
    T: Ord,
    A: AsRef<[T]>,
{
    let array = array.as_ref();
    if array.is_empty() {
        return None;
    }
    let (mut start, mut end) = (0, array.len() - 1);
    while start < end {
        let mid = (end + start) / 2;
        dbg!(&start, &end, &mid);
        match array[mid].cmp(&key) {
            Ordering::Equal => return Some(mid),
            Ordering::Greater => end = mid.saturating_sub(1),
            Ordering::Less => start = mid.saturating_add(1),
        }
    }
    if array[start] == key {
        Some(start)
    } else {
        None
    }
}
