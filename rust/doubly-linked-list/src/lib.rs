use std::ptr::NonNull;

// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

struct Node<T> {
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
    element: T,
}

pub struct Cursor<'a, T> {
    current: Option<NonNull<Node<T>>>,
    list: &'a mut LinkedList<T>,
}

pub struct Iter<'a, T> {
    current: Option<&'a NonNull<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            current: self.head,
            list: self,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            current: self.tail,
            list: self,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.head.as_ref(),
        }
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        let current = unsafe { self.current?.as_mut() };
        Some(&mut current.element)
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        let current = unsafe { self.current?.as_mut() };
        let mut next = current.next?;
        self.current = Some(next);
        Some(&mut unsafe { next.as_mut() }.element)
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        let current = unsafe { self.current?.as_mut() };
        let mut prev = current.prev?;
        self.current = Some(prev);
        Some(&mut unsafe { prev.as_mut() }.element)
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        let current_ptr = self.current?.as_ptr();
        // SAFETY: We construct the Node ourselves in insert().
        // We cast back to Box to drop the reference at the end of scope, preventing any leaks.
        let current = unsafe { Box::from_raw(current_ptr) };
        // Adjust head and tail accordingly
        if self.list.head?.as_ptr() == current_ptr {
            self.list.head = current.next;
        }
        if self.list.tail?.as_ptr() == current_ptr {
            self.list.tail = current.prev;
        }
        // Update prev and next to forget this node
        if let Some(mut prev) = current.prev {
            let prev = unsafe { prev.as_mut() };
            prev.next = current.next;
        }
        if let Some(mut next) = current.next {
            let next = unsafe { next.as_mut() };
            next.prev = current.prev;
        }
        self.current = current.next.or(current.prev);
        self.list.len -= 1;
        // Partial move required to drop the Box.
        let element = current.element;
        Some(element)
    }

    fn insert(
        &mut self,
        element: T,
        prev: Option<NonNull<Node<T>>>,
        next: Option<NonNull<Node<T>>>,
    ) {
        let node = Box::new(Node {
            prev,
            next,
            element,
        });
        let node = NonNull::new(Box::into_raw(node));
        match prev {
            // Start of list
            None => self.list.head = node,
            Some(mut prev) => {
                let prev = unsafe { prev.as_mut() };
                prev.next = node;
            }
        }
        match next {
            // End of list
            None => self.list.tail = node,
            Some(mut next) => {
                let next = unsafe { next.as_mut() };
                next.prev = node;
            }
        }
        self.list.len += 1;
    }

    pub fn insert_after(&mut self, element: T) {
        let (prev, next) = match self.current {
            // Empty list
            None => (None, None),
            Some(mut current) => {
                let current = unsafe { current.as_mut() };
                (self.current, current.next)
            }
        };
        self.insert(element, prev, next);
    }

    pub fn insert_before(&mut self, element: T) {
        let (prev, next) = match self.current {
            // Empty list
            None => (None, None),
            Some(mut current) => {
                let current = unsafe { current.as_mut() };
                (current.prev, self.current)
            }
        };
        self.insert(element, prev, next);
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let current = unsafe { self.current?.as_ref() };
        let element = &current.element;
        self.current = current.next.as_ref();
        Some(element)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cursor = self.cursor_front();
        while cursor.take().is_some() {}
    }
}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
