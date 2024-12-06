use aoc_runner_derive::aoc;

use crate::{debug, Assume, BitIter};

const ZERO: u8 = b'0';
const ZERO_ZERO: u16 = ZERO as u16 * 0x0101;
const ZERO_64: u64 = ZERO as u64 * 0x0101010101010101;

static mut RULES: [u128; 100] = [0; 100];

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
#[inline]
unsafe fn p(n: u16) -> u8 {
    let [n1, n2] = (n - ZERO_ZERO).to_ne_bytes();

    n1 * 10 + n2
}

const RULE_LINES: usize = 1_176;

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn parse_rules(input: &[u8]) {
    const { assert!(RULE_LINES % 2 == 0) }
    for line in (0..RULE_LINES).step_by(1) {
        let [n1_1, n1_2, _, n2_1, n2_2, ..] =
            (input.as_ptr().add(line * 6).cast::<u64>().read_unaligned() - ZERO_64).to_ne_bytes();
        let n1 = n1_1 * 10 + n1_2;
        let n2 = n2_1 * 10 + n2_2;

        *RULES.get_unchecked_mut(n1 as usize) |= 1 << n2;
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> i32 {
    unsafe { inner_p1(input) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_p1(input: &str) -> i32 {
    let input = input.as_bytes();
    parse_rules(input);
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
            let (n, term) = input
                .as_ptr()
                .add(offset)
                .cast::<(u16, u8)>()
                .read_unaligned();
            let num = p(n);
            seen |= 1 << num;
            offset += 3;

            if RULES.get_unchecked(num as usize) & seen != 0 {
                debug!("Rule breakage");
                offset += input
                    .get_unchecked(offset..)
                    .iter()
                    .position(|&c| c == b'\n')
                    .assume()
                    + 1;
                break false;
            }

            if term == b'\n' {
                break true;
            }
        };

        if valid {
            let line_end = offset;
            let n = input
                .as_ptr()
                .add((line_start + line_end) / 2 - 1)
                .cast::<u16>()
                .read_unaligned();
            let midpoint = p(n) as i32;
            debug!("Midpoint: {midpoint}");
            result += midpoint;
        }
    }

    result
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> i32 {
    unsafe { inner_p2(input) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_p2(input: &str) -> i32 {
    let input = input.as_bytes();
    parse_rules(input);

    let mut offset = RULE_LINES * 6 + 1;
    let mut result = 0;

    while input.len() > offset + 3 {
        let mut seen = 0u128;
        let line_start = offset;

        let mut unsorted = false;

        loop {
            debug!(
                "Curr: {}",
                std::str::from_utf8(&input[offset..offset + 3]).unwrap()
            );
            let (n, term) = input
                .as_ptr()
                .add(offset)
                .cast::<(u16, u8)>()
                .read_unaligned();
            let num = p(n);
            seen |= 1 << num;
            offset += 3;

            if RULES.get_unchecked(num as usize) & seen != 0 {
                unsorted = true;
            }

            if term == b'\n' {
                break;
            }
        }

        if unsorted {
            let line_end = offset;
            let num_count = (line_end - line_start) / 3;
            let midpoint = num_count / 2;

            for n in BitIter(seen) {
                if (RULES.get_unchecked(n) & seen).count_ones() == midpoint as u32 {
                    debug!("Midpoint: {midpoint}");
                    result += n as i32;
                    break;
                }
            }
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
