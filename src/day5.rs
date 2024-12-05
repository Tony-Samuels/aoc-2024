use std::cmp::Ordering;

use aoc_runner_derive::aoc;

use crate::{assume, debug, Assume, Unreachable};

const ZERO: u8 = b'0';

static mut RULES: [u128; 100] = [0; 100];

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
#[inline]
unsafe fn p(n1: u8, n2: u8) -> u8 {
    assume!(
        n1.wrapping_sub(ZERO) < 10,
        "Not a digit: {n1} {n1:x} {}",
        n1 as char
    );
    assume!(
        n2.wrapping_sub(ZERO) < 10,
        "Not a digit: {n2} {n2:x} {}",
        n2 as char
    );

    (n1 - ZERO) * 10 + n2 - ZERO
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn parse_rules<const RULE_LINES: usize>(input: &[u8]) {
    for line in 0..RULE_LINES {
        let [n1_1, n1_2] = input
            .as_ptr()
            .add(line * 6)
            .cast::<[u8; 2]>()
            .read_unaligned();
        let n1 = p(n1_1, n1_2);

        let [n2_1, n2_2] = input
            .as_ptr()
            .add(line * 6 + 3)
            .cast::<[u8; 2]>()
            .read_unaligned();
        let n2 = p(n2_1, n2_2);

        assume!(n1 != n2, "{n1} must be before {n2}?!?");
        RULES[n1 as usize] |= 1 << n2;
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> i32 {
    unsafe { inner_p1::<1_176>(input) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_p1<const RULE_LINES: usize>(input: &str) -> i32 {
    let input = input.as_bytes();
    parse_rules::<RULE_LINES>(input);
    let mut offset = RULE_LINES * 6 + 1;

    let mut result = 0;

    while input.len() > offset + 3 {
        let mut seen = 0u128;
        let line_start = offset;

        let valid = loop {
            debug!(
                "Curr: {}",
                std::str::from_utf8(&input[offset..offset + 3]).unwrap()
            );
            let [n1, n2, term] = input
                .as_ptr()
                .add(offset)
                .cast::<[u8; 3]>()
                .read_unaligned();
            let num = p(n1, n2);
            seen |= 1 << num;
            offset += 3;

            if RULES[num as usize] & seen != 0 {
                debug!("Rule breakage");
                offset += input[offset..].iter().position(|&c| c == b'\n').assume() + 1;
                break false;
            }

            if term == b'\n' {
                break true;
            }
        };

        if valid {
            let line_end = offset;
            let [n1, n2] = input
                .as_ptr()
                .add((line_start + line_end) / 2 - 1)
                .cast::<[u8; 2]>()
                .read_unaligned();
            let midpoint = p(n1, n2) as i32;
            debug!("Midpoint: {midpoint}");
            result += midpoint;
        }
    }

    result
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> i32 {
    unsafe { inner_p2::<1_176>(input) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_p2<const RULE_LINES: usize>(input: &str) -> i32 {
    let input = input.as_bytes();
    parse_rules::<RULE_LINES>(input);

    let mut offset = RULE_LINES * 6 + 1;

    let mut result = 0;

    while input.len() > offset + 3 {
        let mut seen = 0u128;
        let line_start = offset;

        let mut nums = [0; 23];

        let mut unsorted = false;

        loop {
            debug!(
                "Curr: {}",
                std::str::from_utf8(&input[offset..offset + 3]).unwrap()
            );
            let [n1, n2, term] = input
                .as_ptr()
                .add(offset)
                .cast::<[u8; 3]>()
                .read_unaligned();
            let num = p(n1, n2);
            nums[(offset - line_start) / 3] = num;

            seen |= 1 << num;
            offset += 3;

            if RULES[num as usize] & seen != 0 {
                unsorted = true;
            }

            if term == b'\n' {
                break;
            }
        }

        if unsorted {
            let line_end = offset;
            let num_count = (line_end - line_start) / 3;
            let nums = &mut nums[..num_count];
            debug!("Found numbers: {nums:?}");

            nums.select_nth_unstable_by(num_count / 2, |&n1, &n2| {
                if RULES[n1 as usize] & 1 << n2 != 0 {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });

            let midpoint = nums[num_count / 2];
            debug!("Midpoint: {midpoint}");
            result += midpoint as i32;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn real_p1() {
        let input = include_str!("../input/2024/day5.txt");
        assert_eq!(part1(input), 5_391);
    }

    #[test]
    fn real_p2() {
        let input = include_str!("../input/2024/day5.txt");
        assert_eq!(part2(input), 6_142);
    }
}
