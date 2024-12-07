use aoc_runner_derive::aoc;

use crate::{assume, debug, ArrayVec, Assume, Unreachable};

const EOL: u8 = b'\n';

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn get_num(input: impl Iterator<Item = u8>) -> (u64, u8) {
    let mut num = 0;
    for c in input {
        if c.wrapping_sub(b'0') < 10 {
            num *= 10;
            num += (c - b'0') as u64;
        } else {
            debug!("Found num {num}");
            return (num, c);
        }
    }

    Unreachable.assume()
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn inner(input: &str) -> u64 {
        let mut input = input.bytes();
        let mut count = 0;
        let mut vec = ArrayVec::<20, u64>::new();

        while input.len() > 3 {
            vec.clear();

            let (target, term) = get_num(&mut input);
            assume!(
                term == b':',
                "Unexpected start of line terminator after {target}: {term} ({})",
                term as char
            );
            loop {
                let (next, term) = get_num(&mut input);
                vec.push_unchecked(next);
                if term == EOL {
                    break;
                }
            }

            debug!("Running with vec: {vec:?}");
            let math_checks_out = recurse_p1(target, vec);
            debug!("Math checks out for {target}: {math_checks_out}");
            count += target * math_checks_out as u64;
        }

        debug!("Remaining input: {}", input.len());

        count
    }
    unsafe { inner(input) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn recurse_p1<const N: usize>(target: u64, mut nums: ArrayVec<N, u64>) -> bool {
    if nums.len == 1 {
        let num = nums.pop_unchecked();

        num == target
    } else {
        let num = nums.pop_unchecked();

        (target % num == 0 && recurse_p1(target / num, nums))
            || (target >= num && recurse_p1(target - num, nums))
    }
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn inner(input: &str) -> u64 {
        let mut input = input.bytes();
        let mut count = 0;
        let mut vec = ArrayVec::<20, u64>::new();

        while input.len() > 3 {
            vec.clear();

            let (target, term) = get_num(&mut input);
            assume!(
                term == b':',
                "Unexpected start of line terminator after {target}: {term} ({})",
                term as char
            );
            loop {
                let (next, term) = get_num(&mut input);
                vec.push_unchecked(next);
                if term == EOL {
                    break;
                }
            }

            debug!("Running with vec: {vec:?}");
            let math_checks_out = recurse_p2(target, vec);
            debug!("Math checks out for {target}: {math_checks_out}");
            count += target * math_checks_out as u64;
        }

        debug!("Remaining input: {}", input.len());

        count
    }
    unsafe { inner(input) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn recurse_p2<const N: usize>(target: u64, mut nums: ArrayVec<N, u64>) -> bool {
    if nums.len == 1 {
        let num = nums.pop_unchecked();

        num == target
    } else {
        let num = nums.pop_unchecked();
        let tens = 10u64.pow(num.ilog10() + 1);

        (target % num == 0 && recurse_p2(target / num, nums))
            || (target >= num && recurse_p2(target - num, nums)
                || (target % tens == num && recurse_p2(target / tens, nums)))
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
        assert_eq!(part2(input), 500335179214836);
    }
}
