// Using the following solutions as a reference:
// https://exercism.org/tracks/rust/exercises/book-store/solutions/maniac200
// https://exercism.org/tracks/rust/exercises/book-store/solutions/dan-seol
// Plus my own annotations to go through the problem.
pub fn lowest_price(books: &[u32]) -> u32 {
    let mut histogram = books.iter().fold([0u32; 5], |mut acc, book| {
        acc[(book - 1) as usize] += 1;
        acc
    });
    histogram.sort();
    // Now we have the counts of each kind of book, from smallest to biggest
    // (eg. &[1, 1, 2, 3, 4, 4, 5, 5] => [1, 1, 2, 2, 2])
    let mut cur = 0;
    let mut sets = (0..5).fold([0u32; 5], |mut acc, idx| {
        acc[4 - idx] = histogram[idx] - cur;
        cur = histogram[idx];
        acc
    });
    // This gets us the number of possible sets
    // (eg.      [0, 0, 1, 0, 1])
    //  ^ sets of 1, 2, 3, 4, 5 books
    // But since [4, 4] is cheaper than [3, 5], we'll have to count those as pairs of 4 instead
    let groups_of_three_and_five = sets[2].min(sets[4]);
    sets[2] -= groups_of_three_and_five;
    sets[3] += 2 * groups_of_three_and_five;
    sets[4] -= groups_of_three_and_five;
    dbg!(sets);
    // (eg. [0, 0, 0, 2, 0])
    // Now we compute the total
    sets[0] * 1 * 8 * 100
        + sets[1] * 2 * 8 * 95
        + sets[2] * 3 * 8 * 90
        + sets[3] * 4 * 8 * 80
        + sets[4] * 5 * 8 * 75
}
