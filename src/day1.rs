use std::{
    cmp::Ordering,
    hint::unreachable_unchecked,
    mem::MaybeUninit,
    simd::{num::SimdInt as _, Simd},
};

use aoc_runner_derive::aoc;

use crate::Assume as _;

/// Number of datapoints expected
const DATA_COUNT: usize = if cfg!(test) { 6 } else { 1_000 };
/// Number of digits for each pair of numbers in each datapoint
const NUM_DIGITS: usize = if cfg!(test) { 1 } else { 5 };
/// Number of characters separating the pair of numbers in each datapoint
const SEP_CHAR_COUNT: usize = 3;

/// The length of each line in the data, not including the newline character
const LINE_LENGTH: usize = NUM_DIGITS + SEP_CHAR_COUNT + NUM_DIGITS;
/// The position in each line where the second of the numbers starts
const NUM2_START: usize = NUM_DIGITS + SEP_CHAR_COUNT;

// SWAR
const ALL_0: u64 = 0x3030303030303030;
const MASK: u64 = 0x000000FF000000FF;
const MUL1: u64 = 0x000F424000000064;
const MUL2: u64 = 0x0000271000000001;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let (mut left, mut right) = input_handling(input);
    left.sort_unstable();
    right.sort_unstable();

    simd_count(&left, &right) + iter_count(&left, &right)
}

// For profiling
fn simd_count(left: &[i32; DATA_COUNT], right: &[i32; DATA_COUNT]) -> i32 {
    let mut sum = 0;

    for count in 0..(DATA_COUNT / 64) {
        let min = count * 64;
        let left = Simd::<_, 64>::from_slice(&left[min..]);
        let right = Simd::<_, 64>::from_slice(&right[min..]);

        sum += (left - right).abs().reduce_sum()
    }

    sum
}

// For profiling
fn iter_count(left: &[i32; DATA_COUNT], right: &[i32; DATA_COUNT]) -> i32 {
    left.iter()
        .zip(right)
        .skip((DATA_COUNT / 64) * 64)
        .map(|(left, right)| (left - right).abs())
        .sum()
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

    let mut left = MaybeUninit::uninit_array();
    let mut right = MaybeUninit::uninit_array();

    let chunks = &mut input.chunks_exact((LINE_LENGTH + 1) * 64);
    for (i, data) in chunks.enumerate() {
        let mut left_arr = MaybeUninit::uninit_array();
        let mut right_arr = MaybeUninit::uninit_array();

        let lines = &mut data.chunks_exact(LINE_LENGTH + 1);
        for (index, line) in lines.enumerate() {
            // Strip new line character
            let line = &line[..LINE_LENGTH];
            left_arr[index].write(u64_from_slice(&line[..NUM_DIGITS]));
            right_arr[index].write(u64_from_slice(&line[NUM2_START..]));
        }
        debug_assert!(
            lines.remainder().is_empty(),
            "Unexpected remainder: {}",
            std::str::from_utf8(lines.remainder()).unwrap()
        );

        let left_arr = unsafe { MaybeUninit::array_assume_init(left_arr) };
        let right_arr = unsafe { MaybeUninit::array_assume_init(right_arr) };

        for (j, val) in parse_simd(Simd::from(left_arr))
            .as_array()
            .iter()
            .enumerate()
        {
            left[i * 64 + j].write(*val as i32);
        }

        for (j, val) in parse_simd(Simd::from(right_arr))
            .as_array()
            .iter()
            .enumerate()
        {
            right[i * 64 + j].write(*val as i32);
        }
        debug_assert!(
            lines.remainder().is_empty(),
            "Unexpected remainder: {}",
            std::str::from_utf8(lines.remainder()).unwrap()
        );
    }

    let elements_parsed = (DATA_COUNT / 64) * 64;
    for (index, line) in chunks.remainder().chunks(LINE_LENGTH + 1).enumerate() {
        // Strip new line character, if present
        let line = &line[..LINE_LENGTH];
        let num1 = parse_single(u64_from_slice(&line[..NUM_DIGITS]));
        let num2 = parse_single(u64_from_slice(&line[NUM2_START..]));

        left[elements_parsed + index].write(num1);
        right[elements_parsed + index].write(num2);
    }

    unsafe {
        (
            MaybeUninit::array_assume_init(left),
            MaybeUninit::array_assume_init(right),
        )
    }
}

fn u64_from_slice(s: &[u8]) -> u64 {
    let arr = match s {
        #[cfg(test)]
        [num] => [b'0', b'0', b'0', b'0', b'0', b'0', b'0', *num],
        [n1, n2, n3, n4, n5] => [b'0', b'0', b'0', *n1, *n2, *n3, *n4, *n5],
        _ => unsafe { unreachable_unchecked() },
    };

    u64::from_le_bytes(arr)
}

fn parse_single(mut val: u64) -> i32 {
    val -= ALL_0;
    val = (val * 10) + (val >> 8);
    val = ((val & MASK).wrapping_mul(MUL1) + ((val >> 16) & MASK).wrapping_mul(MUL2)) >> 32;

    val as i32
}

fn parse_simd(mut val: Simd<u64, 64>) -> Simd<u64, 64> {
    val -= Simd::splat(ALL_0);
    val = (val * Simd::splat(10)) + (val >> 8);
    val = ((val & Simd::splat(MASK)) * Simd::splat(MUL1)
        + ((val >> 16) & Simd::splat(MASK)) * Simd::splat(MUL2))
        >> 32;

    val
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn single_swar() {
        for input in ["12345", "1"] {
            let val = parse_single(u64_from_slice(input.as_bytes()));
            assert_eq!(val, input.parse::<i32>().unwrap());
        }
    }

    #[test]
    fn simd_swar() {
        let input = Simd::from_array(std::array::from_fn(|i| {
            u64_from_slice(format!("{i:05}").as_bytes())
        }));
        for (i, val) in parse_simd(input).as_array().iter().enumerate() {
            assert_eq!(*val, i as u64);
        }

        let input = Simd::from_array(std::array::from_fn(|i| {
            u64_from_slice(format!("{:05}", i * 1_000).as_bytes())
        }));
        for (i, val) in parse_simd(input).as_array().iter().enumerate() {
            assert_eq!(*val, (i * 1_000) as u64);
        }
    }
}
