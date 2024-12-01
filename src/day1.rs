use std::cmp::Ordering;

use aoc_runner_derive::aoc;

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
pub fn part1(input: &str) -> u32 {
    let (mut left, mut right) = input_handling(input);
    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .map(|(left, right)| (left as i32 - right as i32).abs() as u32)
        .sum::<u32>()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
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

fn input_handling(input: &str) -> ([u32; DATA_COUNT], [u32; DATA_COUNT]) {
    let input = input.as_bytes();

    let mut left = [0; DATA_COUNT];
    let mut right = [0; DATA_COUNT];

    for (index, line) in input.chunks_exact(LINE_LENGTH + 1).enumerate() {
        // Strip new line character
        let line = &line[..LINE_LENGTH];
        let num1: u32 = atoi_simd::parse_pos(&line[..NUM_DIGIT_COUNT]).unwrap();
        let num2: u32 = atoi_simd::parse_pos(&line[NUM2_START..]).unwrap();

        left[index] = num1;
        right[index] = num2;
    }

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
