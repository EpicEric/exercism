#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    // If first list is bigger, swap and compare, then return the inverse
    if first_list.len() > second_list.len() {
        match sublist(second_list, first_list) {
            Comparison::Sublist => return Comparison::Superlist,
            Comparison::Superlist => unreachable!(),
            comparison => return comparison,
        }
    }
    if first_list.is_empty() {
        return if second_list.is_empty() {
            Comparison::Equal
        } else {
            Comparison::Sublist
        };
    }
    for window in second_list.windows(first_list.len()) {
        if window.eq(first_list) {
            return if second_list.len() == first_list.len() {
                Comparison::Equal
            } else {
                Comparison::Sublist
            };
        }
    }
    Comparison::Unequal
}
