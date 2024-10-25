use std::collections::VecDeque;

pub struct CircularBuffer<T> {
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    data: VecDeque<T>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(capacity),
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.data.len() == self.data.capacity() {
            return Err(Error::FullBuffer);
        }
        self.data.push_back(element);
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        match self.data.pop_front() {
            Some(value) => Ok(value),
            None => Err(Error::EmptyBuffer),
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn overwrite(&mut self, element: T) {
        if self.data.len() == self.data.capacity() {
            self.data.pop_front();
        }
        self.data.push_back(element);
    }
}
