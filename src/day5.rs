use std::{
    arch::x86_64::{__m128i, _mm_maddubs_epi16},
    mem::transmute,
    simd::Simd,
};

use aoc_runner_derive::aoc;

use crate::{debug, Assume, BitIter};

const ZERO: u8 = b'0';
const ZERO_ZERO: u16 = ZERO as u16 * 0x0101;

static mut RULES: [u128; 100] = [0; 100];

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
#[inline]
unsafe fn p(n: u16) -> u8 {
    let [n1, n2] = (n - ZERO_ZERO).to_ne_bytes();

    n1 * 10 + n2
}

const RULE_LINES: usize = 1_176;

const RULES_GATHERER: [usize; 32] = {
    let mut arr = [0; 32];
    let mut index = 0;
    loop {
        arr[2 * index] = index * 3;
        arr[2 * index + 1] = index * 3 + 1;
        index += 1;
        if 2 * index == 32 {
            break;
        }
    }
    arr
};

const RULES_MULTIPLIER: __m128i = {
    let mut arr = [0u8; 16];
    let mut index = 0;
    loop {
        arr[index] = 10;
        arr[index + 1] = 1;
        index += 2;
        if index == 16 {
            break;
        }
    }
    unsafe { transmute::<[u8; 16], __m128i>(arr) }
};

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn parse_rules(input: &[u8]) {
    let rules_gatherer = Simd::from_array(RULES_GATHERER);

    const {
        assert!(RULE_LINES % 8 == 0);
    }
    for line in (0..RULE_LINES - 7).step_by(8) {
        let line_offset = line * 6;
        let simd_nums =
            Simd::<u8, 32>::gather_or_default(input.get_unchecked(line_offset..), rules_gatherer)
                - Simd::splat(ZERO);
        let nums = transmute::<__m128i, [u16; 8]>(_mm_maddubs_epi16(
            simd_nums.resize::<16>(0).into(),
            RULES_MULTIPLIER,
        ));

        *RULES.get_unchecked_mut(*nums.get_unchecked(0) as usize) |= 1 << nums.get_unchecked(1);
        *RULES.get_unchecked_mut(*nums.get_unchecked(2) as usize) |= 1 << nums.get_unchecked(3);
        *RULES.get_unchecked_mut(*nums.get_unchecked(4) as usize) |= 1 << nums.get_unchecked(5);
        *RULES.get_unchecked_mut(*nums.get_unchecked(6) as usize) |= 1 << nums.get_unchecked(7);

        let nums = transmute::<__m128i, [u16; 8]>(_mm_maddubs_epi16(
            simd_nums
                .rotate_elements_right::<16>()
                .resize::<16>(0)
                .into(),
            RULES_MULTIPLIER,
        ));

        *RULES.get_unchecked_mut(*nums.get_unchecked(0) as usize) |= 1 << nums.get_unchecked(1);
        *RULES.get_unchecked_mut(*nums.get_unchecked(2) as usize) |= 1 << nums.get_unchecked(3);
        *RULES.get_unchecked_mut(*nums.get_unchecked(4) as usize) |= 1 << nums.get_unchecked(5);
        *RULES.get_unchecked_mut(*nums.get_unchecked(6) as usize) |= 1 << nums.get_unchecked(7);
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

    const INPUT: &str = include_str!("../input/2024/day5.txt");

    unsafe fn old_parse_rules(input: &[u8]) -> [u128; 100] {
        let mut rules = [0; 100];

        for line in 0..RULE_LINES {
            let n1 = input.as_ptr().add(line * 6).cast::<u16>().read_unaligned();
            let n1 = p(n1);

            let n2 = input
                .as_ptr()
                .add(line * 6 + 3)
                .cast::<u16>()
                .read_unaligned();
            let n2 = p(n2);

            *rules.get_unchecked_mut(n1 as usize) |= 1 << n2;
        }

        rules
    }

    #[test]
    fn simd_parse() {
        unsafe { parse_rules(INPUT.as_bytes()) };
        let parsed = unsafe { RULES };
        let expected = unsafe { old_parse_rules(INPUT.as_bytes()) };

        for (index, (parsed, expected)) in parsed.into_iter().zip(expected).enumerate() {
            assert!(
                parsed == expected,
                "Mismatch at index {index}:\n{expected:0100b}\n{parsed:0100b}"
            )
        }
    }

    #[test]
    fn real_p1() {
        assert_eq!(part1(INPUT), 5_391);
    }

    #[test]
    fn real_p2() {
        assert_eq!(part2(INPUT), 6_142);
    }
}
