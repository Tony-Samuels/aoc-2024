use std::{
    cmp::Ordering,
    hint::unreachable_unchecked,
    mem::MaybeUninit,
    simd::{num::SimdInt as _, LaneCount, Simd, SupportedLaneCount},
};

use aoc_runner_derive::aoc;

use crate::Assume as _;

/// Number of datapoints expected
const DATA_COUNT: usize = 1_000;
/// Number of digits for each pair of numbers in each datapoint
const NUM_DIGITS: usize = 5;
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

fn input_handling_inner<'a, const N: usize>(
    left: &mut [MaybeUninit<i32>; DATA_COUNT],
    right: &mut [MaybeUninit<i32>; DATA_COUNT],
    lines: impl Iterator<Item = &'a [u8]>,
    offset: usize,
) where
    LaneCount<N>: SupportedLaneCount,
{
    let mut left_arr = MaybeUninit::uninit_array();
    let mut right_arr = MaybeUninit::uninit_array();

    for (index, line) in lines.take(N).enumerate() {
        // Strip new line character
        let line = &line[..LINE_LENGTH];
        left_arr[index].write(u64_from_slice(&line[..NUM_DIGITS]));
        right_arr[index].write(u64_from_slice(&line[NUM2_START..]));
    }

    let left_arr = unsafe { MaybeUninit::array_assume_init(left_arr) };
    let right_arr = unsafe { MaybeUninit::array_assume_init(right_arr) };

    for (j, val) in parse_simd::<N>(Simd::from(left_arr))
        .as_array()
        .iter()
        .enumerate()
    {
        left[offset + j].write(*val as i32);
    }

    for (j, val) in parse_simd::<N>(Simd::from(right_arr))
        .as_array()
        .iter()
        .enumerate()
    {
        right[offset + j].write(*val as i32);
    }
}

fn input_handling(input: &str) -> ([i32; DATA_COUNT], [i32; DATA_COUNT]) {
    let input = input.as_bytes();

    let mut left = MaybeUninit::uninit_array();
    let mut right = MaybeUninit::uninit_array();

    let mut lines = input.chunks(LINE_LENGTH + 1);
    let mut offset = 0;
    for _ in 0..(DATA_COUNT / 64) {
        input_handling_inner::<64>(&mut left, &mut right, &mut lines, offset);
        offset += 64;
    }

    input_handling_inner::<32>(&mut left, &mut right, &mut lines, offset);
    offset += 32;

    input_handling_inner::<8>(&mut left, &mut right, &mut lines, offset);
    offset += 8;

    debug_assert!(
        offset == 1_000,
        "Expected to have written 1000 lines, written {offset}"
    );

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

fn parse_simd<const N: usize>(mut val: Simd<u64, N>) -> Simd<u64, N>
where
    LaneCount<N>: SupportedLaneCount,
{
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

    const INPUT: &str = include_str!("../input/2024/day1.txt");

    #[test]
    fn example_p1() {
        assert_eq!(part1(INPUT), 1938424);
    }

    #[test]
    fn example_p2() {
        assert_eq!(part2(INPUT), 22014209)
    }

    #[test]
    fn simd_swar() {
        let input = Simd::from_array(std::array::from_fn(|i| {
            u64_from_slice(format!("{i:05}").as_bytes())
        }));
        for (i, val) in parse_simd::<64>(input).as_array().iter().enumerate() {
            assert_eq!(*val, i as u64);
        }

        let input = Simd::from_array(std::array::from_fn(|i| {
            u64_from_slice(format!("{:05}", i * 1_000).as_bytes())
        }));
        for (i, val) in parse_simd::<64>(input).as_array().iter().enumerate() {
            assert_eq!(*val, (i * 1_000) as u64);
        }
    }
}
