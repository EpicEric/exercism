pub struct SimpleLinkedList<T> {
    // Delete this field
    // dummy is needed to avoid unused parameter error during compilation
    contents: Option<(T, Box<SimpleLinkedList<T>>)>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { contents: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.contents.is_none()
    }

    pub fn len(&self) -> usize {
        match self.contents {
            Some((_, ref tail)) => 1 + tail.len(),
            None => 0,
        }
    }

    pub fn push(&mut self, element: T) {
        let contents = Box::new(Self {
            contents: self.contents.take(),
        });
        self.contents = Some((element, contents));
    }

    pub fn pop(&mut self) -> Option<T> {
        let (element, contents) = self.contents.take()?;
        self.contents = contents.contents;
        Some(element)
    }

    pub fn peek(&self) -> Option<&T> {
        self.contents.as_ref().map(|(element, _)| element)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut this = self;
        let mut list = Self::new();
        while let Some(element) = this.pop() {
            list.push(element);
        }
        list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for element in iter {
            list.push(element);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let len = linked_list.len();
        let vec = Vec::with_capacity(len);
        let mut vec = std::mem::ManuallyDrop::new(vec);
        let cap = vec.capacity();
        let vec_ptr: *mut T = vec.as_mut_ptr();
        for i in (0..len).rev() {
            let element = linked_list.pop().unwrap();
            // SAFETY: We only access the allocated capacity of the underlying Vec (i.e. in range 0..len).
            unsafe {
                *vec_ptr.add(i) = element;
            }
        }
        // SAFETY: We are reusing values from the previous Vec allocation.
        // Conversely, ManuallyDrop guarantees that there won't be a double-free.
        unsafe { Vec::from_raw_parts(vec_ptr, len, cap) }
    }
}
