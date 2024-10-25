use std::{borrow::Borrow, ops::BitXor};

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    pointer: usize,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key: ?Sized + AsRef<[u8]>>(key: &'a Key) -> Xorcism<'a> {
        Xorcism {
            key: key.as_ref(),
            pointer: 0,
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        if self.key.is_empty() {
            return;
        }
        data.iter_mut().for_each(|d| {
            *d ^= self.key[self.pointer];
            self.pointer += 1;
            if self.pointer == self.key.len() {
                self.pointer = 0;
            }
        });
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    // NOTE: I struggled to figure out the solution below on my own
    pub fn munge<'b, Data>(&'b mut self, data: Data) -> impl Iterator<Item = u8> + 'b + Captures<'a>
    where
        Data: IntoIterator,
        <Data as IntoIterator>::Item: Borrow<u8>,
        <Data as IntoIterator>::IntoIter: 'b,
        <Data as IntoIterator>::Item: BitXor<u8, Output = u8>,
    {
        data.into_iter().map(|d| {
            let d = d ^ self.key[self.pointer];
            self.pointer += 1;
            if self.pointer == self.key.len() {
                self.pointer = 0;
            }
            d
        })
    }
}

// https://github.com/rust-lang/rust/issues/34511#issuecomment-373423999
pub trait Captures<'a> {}
impl<'a, T: ?Sized> Captures<'a> for T {}
