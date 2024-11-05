#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

impl Bucket {
    fn other(&self) -> Self {
        match self {
            Bucket::One => Bucket::Two,
            Bucket::Two => Bucket::One,
        }
    }
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

fn gcd(mut a: u8, mut b: u8) -> u8 {
    while b != 0 {
        (a, b) = (b, a % b)
    }
    a
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    if goal > capacity_1 && goal > capacity_2 {
        return None;
    }
    if goal % gcd(capacity_1, capacity_2) != 0 {
        return None;
    }
    let mut step = 1;
    let (left_capacity, right_capacity, mut left_bucket, mut right_bucket) =
        if start_bucket == &Bucket::One {
            (capacity_1, capacity_2, capacity_1, 0)
        } else {
            (capacity_2, capacity_1, capacity_2, 0)
        };
    if right_capacity == goal {
        step = 2;
        right_bucket = goal;
    } else {
        while left_bucket != goal && right_bucket != goal {
            step += 1;
            // Reference: https://swdevnotes.com/algorithms/2021/water-pouring-puzzle/
            if right_bucket == right_capacity {
                right_bucket = 0;
            } else if left_bucket == 0 {
                left_bucket = left_capacity;
            } else {
                let amount_to_pass = left_bucket.min(right_capacity - right_bucket);
                left_bucket -= amount_to_pass;
                right_bucket += amount_to_pass;
            }
        }
    }
    Some(BucketStats {
        moves: step,
        goal_bucket: if left_bucket == goal {
            *start_bucket
        } else {
            start_bucket.other()
        },
        other_bucket: if left_bucket == goal {
            right_bucket
        } else {
            left_bucket
        },
    })
}
