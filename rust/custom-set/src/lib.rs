#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T>(Vec<T>);

impl<T> CustomSet<T>
where
    T: Ord + Clone,
{
    pub fn new(input: &[T]) -> Self {
        let mut set = CustomSet(input.iter().cloned().collect());
        // We keep our inner Vec sorted and deduped throughout the whole code.
        set.0.sort();
        set.0.dedup();
        set
    }

    pub fn contains(&self, element: &T) -> bool {
        // Vec is sorted, we can perform binary search.
        self.0.binary_search(element).is_ok()
    }

    pub fn add(&mut self, element: T) {
        // Add element while preserving sortedness.
        match self.0.binary_search(&element) {
            Ok(_) => (),
            Err(pos) => self.0.insert(pos, element),
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.0.iter().all(|element| other.contains(element))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        !self.0.iter().any(|element| other.contains(element))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        // Vec is still sorted if we drop elements.
        CustomSet(
            self.0
                .iter()
                .filter(|element| other.contains(element))
                .cloned()
                .collect::<Vec<_>>(),
        )
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        // Vec is still sorted if we drop elements.
        CustomSet(
            self.0
                .iter()
                .filter(|element| !other.contains(element))
                .cloned()
                .collect::<Vec<_>>(),
        )
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut new_set = CustomSet(self.0.clone());
        // Using CustomSet::add guarantees that the inner Vec stays sorted and dedup'd.
        for element in other.0.iter().cloned() {
            new_set.add(element);
        }
        new_set
    }
}
