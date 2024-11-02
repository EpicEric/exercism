use std::{fmt::Debug, ops::Add};

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: Debug + Copy + Add<T, Output = T> + PartialOrd + PartialEq + From<u8>,
{
    pub fn build(mut sides: [T; 3]) -> Option<Triangle<T>> {
        if sides[0] > <T>::from(0) && sides[1] > <T>::from(0) && sides[0] > <T>::from(0) {
            sides.sort_by(|a, b| a.partial_cmp(b).unwrap());
            if sides[0] + sides[1] >= sides[2] {
                Some(Triangle { sides })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1] && self.sides[1] != self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1] || self.sides[1] == self.sides[2]
    }
}
