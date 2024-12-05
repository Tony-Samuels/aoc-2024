use std::cmp::Ordering;

use aoc_runner_derive::aoc;

use crate::{assume, debug, Assume, Unreachable};

const ZERO: u8 = b'0';

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
unsafe fn parse_rules<const RULE_LINES: usize>(input: &[u8]) -> [u128; 100] {
    let mut rules = [0; 100];

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
        rules[n1 as usize] |= 1 << n2;
    }

    rules
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> i32 {
    unsafe { inner_p1::<1_176>(input) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_p1<const RULE_LINES: usize>(input: &str) -> i32 {
    let input = input.as_bytes();
    let rules = parse_rules::<RULE_LINES>(input);
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

            if rules[num as usize] & seen != 0 {
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
    let rules = parse_rules::<RULE_LINES>(input);

    let mut offset = RULE_LINES * 6 + 1;

    let mut result = 0;

    while input.len() > offset + 3 {
        let mut seen = 0u128;
        let line_start = offset;

        let mut nums = [0; 33];

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

            if rules[num as usize] & seen != 0 {
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

            nums.sort_by(|&n1, &n2| {
                if rules[n1 as usize] & 1 << n2 != 0 {
                    Ordering::Greater
                } else if rules[n2 as usize] & 1 << n1 != 0 {
                    Ordering::Less
                } else {
                    Ordering::Equal
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

    const INPUT: &str = indoc!(
        "
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
        "
    );

    #[test]
    fn rules_example() {
        let mut expected_rules = [0u128; 100];
        expected_rules[29] |= 1 << 13;
        expected_rules[47] |= 1 << 13;
        expected_rules[47] |= 1 << 29;
        expected_rules[47] |= 1 << 53;
        expected_rules[47] |= 1 << 61;
        expected_rules[53] |= 1 << 13;
        expected_rules[53] |= 1 << 29;
        expected_rules[61] |= 1 << 13;
        expected_rules[61] |= 1 << 29;
        expected_rules[61] |= 1 << 53;
        expected_rules[75] |= 1 << 13;
        expected_rules[75] |= 1 << 29;
        expected_rules[75] |= 1 << 47;
        expected_rules[75] |= 1 << 53;
        expected_rules[75] |= 1 << 61;
        expected_rules[97] |= 1 << 13;
        expected_rules[97] |= 1 << 29;
        expected_rules[97] |= 1 << 47;
        expected_rules[97] |= 1 << 53;
        expected_rules[97] |= 1 << 61;
        expected_rules[97] |= 1 << 75;

        let rules = unsafe { parse_rules::<21>(INPUT.as_bytes()) };

        for (index, (expected, calculated)) in expected_rules.into_iter().zip(rules).enumerate() {
            assert!(
                expected == calculated,
                "Failed for {index}:\n{expected:0100b}\n{calculated:0100b}\n{}",
                std::iter::repeat_n(0..10, 10)
                    .flatten()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            );
        }
    }

    #[test]
    fn p1_example() {
        assert_eq!(unsafe { inner_p1::<21>(INPUT) }, 143);
    }

    #[test]
    fn p2_example() {
        assert_eq!(unsafe { inner_p2::<21>(INPUT) }, 123);
    }

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
