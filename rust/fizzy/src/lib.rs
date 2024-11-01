use std::{cell::RefCell, fmt::Display, ops::Rem};

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    function: Box<dyn Fn(T) -> bool>,
    substitution: String,
}

impl<T> Matcher<T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: Into<String>,
    {
        Matcher {
            function: Box::new(matcher),
            substitution: subs.into(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T> {
    matchers: RefCell<Vec<Matcher<T>>>,
}

impl<T> Fizzy<T> {
    pub fn new() -> Self {
        Fizzy {
            matchers: RefCell::new(Vec::new()),
        }
    }

    #[must_use]
    pub fn add_matcher(self, matcher: Matcher<T>) -> Self {
        self.matchers.borrow_mut().push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: IntoIterator<Item = T>,
        T: Display + Copy,
    {
        iter.into_iter().map(move |value| {
            let string: String = self
                .matchers
                .borrow()
                .iter()
                .map(|matcher| {
                    if (matcher.function)(value) {
                        matcher.substitution.clone()
                    } else {
                        String::new()
                    }
                })
                .collect();
            if string.is_empty() {
                value.to_string()
            } else {
                string
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: PartialEq<T> + Rem<T, Output = T> + From<u8>,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n| n % 3u8.into() == 0u8.into(), "fizz"))
        .add_matcher(Matcher::new(|n| n % 5u8.into() == 0u8.into(), "buzz"))
}
