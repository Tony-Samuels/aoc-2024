use std::intrinsics::{unchecked_div, unchecked_mul, unchecked_rem, unchecked_sub};

use aoc_runner_derive::aoc;
use atoi_simd::parse_any_pos;

use crate::{ArrayVec, Assume};

const EOL: u8 = b'\n';

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn inner(input: &str) -> u64 {
        let input = input.as_bytes();
        let mut count = 0;
        let mut vec = ArrayVec::<12, u64>::new();
        let mut pos = 0;

        while input.len() > pos + 3 {
            vec.clear();

            let (target, bytes) = parse_any_pos(input.get_unchecked(pos..)).assume();
            pos += bytes + 2;
            loop {
                let (next, bytes) = parse_any_pos(input.get_unchecked(pos..)).assume();
                vec.push_unchecked(next);
                let term = *input.get_unchecked(pos + bytes);
                pos += bytes + 1;
                if term == EOL {
                    break;
                }
            }

            let math_checks_out = recurse_p1(target, vec);
            count += target * math_checks_out as u64;
        }

        count
    }
    unsafe { inner(input) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn recurse_p1<const N: usize>(target: u64, mut nums: ArrayVec<N, u64>) -> bool {
    let num = nums.pop_unchecked();
    if nums.len == 0 {
        num == target
    } else {
        (unchecked_rem(target, num) == 0 && recurse_p1(unchecked_div(target, num), nums))
            || (target >= num && recurse_p1(unchecked_sub(target, num), nums))
    }
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn inner(input: &str) -> u64 {
        let input = input.as_bytes();
        let mut count = 0;
        let mut vec = ArrayVec::<12, u64>::new();
        let mut pos = 0;

        while input.len() > pos + 3 {
            vec.clear();

            let (target, bytes) = parse_any_pos(input.get_unchecked(pos..)).assume();
            pos += bytes + 2;
            loop {
                let (next, bytes) = parse_any_pos(input.get_unchecked(pos..)).assume();
                vec.push_unchecked(next);
                let term = *input.get_unchecked(pos + bytes);
                pos += bytes + 1;
                if term == EOL {
                    break;
                }
            }

            let math_checks_out = recurse_p2(target, vec);
            count += target * math_checks_out as u64;
        }

        count
    }
    unsafe { inner(input) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn recurse_p2<const N: usize>(target: u64, mut nums: ArrayVec<N, u64>) -> bool {
    let num = nums.pop_unchecked();
    if nums.len == 0 {
        num == target
    } else {
        (unchecked_rem(target, num) == 0 && recurse_p2(unchecked_div(target, num), nums))
            || (target >= num && recurse_p2(unchecked_sub(target, num), nums)
                || ({
                    let mut temp = num;
                    let mut tens = 1;
                    while temp > 0 {
                        tens = unchecked_mul(tens, 10);
                        temp = unchecked_div(temp, 10);
                    }
                    unchecked_rem(target, tens) == num
                        && recurse_p2(unchecked_div(target, tens), nums)
                }))
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    "};

    #[test]
    fn p1_example() {
        assert_eq!(part1(INPUT), 3_749);
    }

    #[test]
    fn p2_example() {
        assert_eq!(part2(INPUT), 11_387);
    }

    #[test]
    fn real_p1() {
        let input = include_str!("../input/2024/day7.txt");
        assert_eq!(part1(input), 10_741_443_549_536);
    }

    #[test]
    fn real_p2() {
        let input = include_str!("../input/2024/day7.txt");
        assert_eq!(part2(input), 500_335_179_214_836);
    }
}
