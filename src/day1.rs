use std::{
    cmp::Ordering,
    mem::MaybeUninit,
    simd::{num::SimdInt as _, Simd},
};

use aoc_runner_derive::aoc;

use crate::Assume as _;

/// Number of datapoints expected
const DATA_COUNT: usize = if cfg!(test) { 6 } else { 1_000 };
/// Number of digits for each pair of numbers in each datapoint
const NUM_DIGIT_COUNT: usize = if cfg!(test) { 1 } else { 5 };
/// Number of characters separating the pair of numbers in each datapoint
const SEP_CHAR_COUNT: usize = 3;

/// The length of each line in the data, not including the newline character
const LINE_LENGTH: usize = NUM_DIGIT_COUNT + SEP_CHAR_COUNT + NUM_DIGIT_COUNT;
/// The position in each line where the second of the numbers starts
const NUM2_START: usize = NUM_DIGIT_COUNT + SEP_CHAR_COUNT;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let (mut left, mut right) = input_handling(input);
    left.sort_unstable();
    right.sort_unstable();

    let div_64 = left.len() / 64;
    let mut sum = 0;

    for count in 0..div_64 {
        let min = count * 64;
        let left = Simd::<_, 64>::from_slice(&left[min..]);
        let right = Simd::<_, 64>::from_slice(&right[min..]);

        sum += (left - right).abs().reduce_sum()
    }

    sum + left
        .into_iter()
        .zip(right)
        .skip(div_64 * 64)
        .map(|(left, right)| (left - right).abs())
        .sum::<i32>()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let (mut left, mut right) = input_handling(input);
    left.sort_unstable();
    right.sort_unstable();
    let left = &mut left.into_iter();
    let right = &mut right.into_iter();

    let mut similarity = 0;
    let mut curr_left_similarity = 0;
    let mut curr_left = left.next().assume();
    let mut curr_right = right.next().assume();

    loop {
        match curr_left.cmp(&curr_right) {
            Ordering::Less => {
                let mut new_left = Some(curr_left);
                while new_left == Some(curr_left) {
                    similarity += curr_left_similarity;
                    new_left = left.next();
                }

                curr_left_similarity = 0;

                if let Some(new_left) = new_left {
                    curr_left = new_left;
                } else {
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

    similarity + curr_left_similarity
}

fn input_handling(input: &str) -> ([i32; DATA_COUNT], [i32; DATA_COUNT]) {
    let input = input.as_bytes();

    let mut left = [MaybeUninit::uninit(); DATA_COUNT];
    let mut right = [MaybeUninit::uninit(); DATA_COUNT];

    let chunks = &mut input.chunks_exact(LINE_LENGTH + 1);

    for (index, line) in chunks.enumerate() {
        // Strip new line character
        let line = &line[..LINE_LENGTH];
        let num1 = parse_pos(&line[..NUM_DIGIT_COUNT]);
        let num2 = parse_pos(&line[NUM2_START..]);

        left[index].write(num1);
        right[index].write(num2);
    }

    // End '\n' might be stripped
    let remainder: Option<[_; LINE_LENGTH]> = chunks.remainder().try_into().ok();
    if let Some(line) = remainder {
        let num1 = parse_pos(&line[..NUM_DIGIT_COUNT]);
        let num2 = parse_pos(&line[NUM2_START..]);

        left.last_mut().assume().write(num1);
        right.last_mut().assume().write(num2);
    } else {
        debug_assert!(
            chunks.remainder().is_empty(),
            "Remainder: {}",
            std::str::from_utf8(chunks.remainder()).unwrap()
        );
    }

    unsafe {
        (
            MaybeUninit::array_assume_init(left),
            MaybeUninit::array_assume_init(right),
        )
    }
}

// For profiling
fn parse_pos(s: &[u8]) -> i32 {
    atoi_simd::parse_pos(s).assume()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn example_p1() {
        assert_eq!(part1(INPUT), 11);
    }

    #[test]
    fn example_p2() {
        assert_eq!(part2(INPUT), 31)
    }
}
