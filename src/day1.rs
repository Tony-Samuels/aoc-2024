use std::cmp::Ordering;

use aoc_runner_derive::aoc;

const NUM_DIGIT_COUNT: usize = if cfg!(test) { 1 } else { 5 };
const SEP_CHAR_COUNT: usize = 3;

// Perf notes:
// - Using `u32` and `abs_diff` is ~20-30% slower
// - Using `&[u8]` instead of `&str` and `str` functions is ~20-30% faster
// - Using indexing instead of iterating makes no difference
#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let (left, right) = input_handling(input);

    left.into_iter()
        .zip(right.into_iter())
        .map(|(left, right)| (left - right).abs())
        .sum::<i32>()
}

// Perf notes:
// - Using the current pointer solution is ~10-15% faster than a `HashMap` based
#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let (left, right) = input_handling(input);
    let left = &mut left.into_iter();
    let right = &mut right.into_iter();

    let mut similarity = 0;
    let mut curr_left_similarity = 0;

    let mut curr_left = left.next().unwrap();
    let mut curr_right = right.next().unwrap();

    loop {
        match curr_left.cmp(&curr_right) {
            Ordering::Less => {
                similarity += curr_left_similarity;

                if let Some(new_left) = left.next() {
                    if curr_left != new_left {
                        curr_left_similarity = 0;
                    }
                    curr_left = new_left;
                } else {
                    curr_left_similarity = 0;
                    break;
                }
            }
            Ordering::Greater => {
                if let Some(new_right) = right.next() {
                    curr_right = new_right;
                } else {
                    break;
                }
            }
            Ordering::Equal => {
                curr_left_similarity += curr_left;
                if let Some(new_right) = right.next() {
                    curr_right = new_right;
                } else {
                    break;
                }
            }
        }
    }

    similarity += curr_left * curr_left_similarity;

    similarity
}

// Perf notes:
// - Using `unwrap` instead of `unreachable_unchecked` is faster!
//     - 3-10% depending on where it's used
// - Using `unwrap` and `unreachable` has the same perf (as expected)
// - Using an array instead of a `Vec` is ~10% slower
// - Using foreknowledge of how long the numbers are makes a ~20-25% improvement versus searching
fn input_handling(input: &str) -> (Vec<i32>, Vec<i32>) {
    let input = input.as_bytes();

    let mut left = Vec::with_capacity(1_000);
    let mut right = Vec::with_capacity(1_000);

    const LINE_LENGTH: usize = NUM_DIGIT_COUNT + SEP_CHAR_COUNT + NUM_DIGIT_COUNT;
    const NUM2_START: usize = NUM_DIGIT_COUNT + SEP_CHAR_COUNT;

    for line in input.chunks_exact(LINE_LENGTH + 1) {
        // Strip new line character
        let line = &line[..LINE_LENGTH];
        let num1: i32 = atoi_simd::parse_pos(&line[..NUM_DIGIT_COUNT]).unwrap();
        let num2: i32 = atoi_simd::parse_pos(&line[NUM2_START..]).unwrap();

        left.push(num1);
        right.push(num2);
    }

    left.sort_unstable();
    right.sort_unstable();

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn example_p1() {
        assert_eq!(part1(INPUT), 11);
    }

    #[test]
    fn example_p2() {
        assert_eq!(part2(INPUT), 31)
    }
}
