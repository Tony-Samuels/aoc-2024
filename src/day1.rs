use std::cmp::Ordering;

use aoc_runner_derive::aoc;
use atoi_simd::parse;

// Perf notes:
// - Using `u32` and abs_diff is ~20-30% slower
// - Using `&str`, `parse` and `str::split` is ~20-30% slower
#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let (left, right) = input_handling(input);

    left.into_iter()
        .zip(right.into_iter())
        .map(|(left, right)| (left - right).abs())
        .sum::<i32>()
}

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
                similarity += curr_left * curr_left_similarity;

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
                curr_left_similarity += 1;
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

fn input_handling(input: &str) -> (Vec<i32>, Vec<i32>) {
    let input = input.as_bytes();

    let mut left = Vec::with_capacity(1_000);
    let mut right = Vec::with_capacity(1_000);

    for line in input.split(|&c| c == b'\n') {
        let iter = &mut line.split(|&c| c == b' ');
        let num1: i32 = parse(iter.next().unwrap()).unwrap();
        let num2: i32 = parse(iter.skip_while(|s| s.is_empty()).next().unwrap()).unwrap();

        debug_assert!(
            iter.next().is_none(),
            "Expected line to have only two numbers: {}",
            std::str::from_utf8(line).unwrap()
        );

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
