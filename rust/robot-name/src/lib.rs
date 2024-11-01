use std::sync::{
    atomic::{AtomicU64, Ordering},
    LazyLock,
};

use rand::{thread_rng, RngCore};

static RNG: LazyLock<AtomicU64> =
    LazyLock::new(|| AtomicU64::new(thread_rng().next_u64() % 676_000));

/// Generates a random number in range 0..676_000 using a Linear Congruential Generator.
fn next_random_number() -> u64 {
    let mut rng = RNG.load(Ordering::Acquire);
    rng = rng.wrapping_mul(16_811).wrapping_add(7) % 676_000;
    RNG.store(rng, Ordering::Release);
    rng
}

fn generate_name() -> String {
    let mut prng = next_random_number();
    let mut bytes = Vec::with_capacity(5);
    bytes.push(LETTERS[(prng as usize) % LETTERS.len()]);
    prng /= LETTERS.len() as u64;
    bytes.push(LETTERS[(prng as usize) % LETTERS.len()]);
    prng /= LETTERS.len() as u64;
    bytes.push(DIGITS[(prng as usize) % DIGITS.len()]);
    prng /= DIGITS.len() as u64;
    bytes.push(DIGITS[(prng as usize) % DIGITS.len()]);
    prng /= DIGITS.len() as u64;
    bytes.push(DIGITS[(prng as usize) % DIGITS.len()]);
    String::from_utf8(bytes).unwrap()
}

static LETTERS: &[u8; 26] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static DIGITS: &[u8; 10] = b"0123456789";

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: generate_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        // Total number of names:
        // 26^2 + 10^3 = 676_000
        let mut prng = next_random_number();
        let mut bytes = Vec::with_capacity(5);
        bytes.push(LETTERS[(prng as usize) % LETTERS.len()]);
        prng /= LETTERS.len() as u64;
        bytes.push(LETTERS[(prng as usize) % LETTERS.len()]);
        prng /= LETTERS.len() as u64;
        bytes.push(DIGITS[(prng as usize) % DIGITS.len()]);
        prng /= DIGITS.len() as u64;
        bytes.push(DIGITS[(prng as usize) % DIGITS.len()]);
        prng /= DIGITS.len() as u64;
        bytes.push(DIGITS[(prng as usize) % DIGITS.len()]);
        self.name = generate_name();
    }
}
