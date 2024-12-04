use std::{cmp::min, mem::transmute, num::NonZero};

use aoc_runner_derive::aoc;
use memchr::Memchr;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
    unsafe { part1_inner::<141>(input.as_bytes()) }
}

const X: NonZero<u8> = NonZero::new(b'X').unwrap();
const M: NonZero<u8> = NonZero::new(b'M').unwrap();
const A: NonZero<u8> = NonZero::new(b'A').unwrap();
const S: NonZero<u8> = NonZero::new(b'S').unwrap();

const XMAS: u32 = unsafe { transmute::<[NonZero<u8>; 4], u32>([X, M, A, S]) };
const SAMX: u32 = unsafe { transmute::<[NonZero<u8>; 4], u32>([S, A, M, X]) };

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part1_inner<const LINE_LEN: usize>(input: &[u8]) -> u32 {
    let mut count = 0;

    let input = transmute::<&[u8], &[NonZero<u8>]>(input);

    for x_pos in input
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == X)
        .map(|(n, _)| n)
    {
        // Right/left
        if x_pos + 3 < input.len() {
            let val = (input.as_ptr().add(x_pos) as *const u32).read_unaligned();
            count += (val == XMAS) as u32;
        }

        if x_pos >= 3 {
            let val = (input.as_ptr().add(x_pos).sub(3) as *const u32).read_unaligned();
            count += (val == SAMX) as u32;
        }

        for diff in [
            // Up/down
            LINE_LEN,
            // \
            LINE_LEN + 1,
            // /
            LINE_LEN - 1,
        ] {
            let valid = some_or_0((input.get(x_pos + diff))) == M.into();
            let valid = valid && some_or_0((input.get(x_pos + 2 * diff))) == A.into();
            let valid = valid && some_or_0((input.get(x_pos + 3 * diff))) == S.into();
            count += valid as u32;

            let valid = some_or_0(input.get(x_pos.wrapping_sub(diff))) == M.into();
            let valid = valid && some_or_0((input.get(x_pos.wrapping_sub(2 * diff)))) == A.into();
            let valid = valid && some_or_0((input.get(x_pos.wrapping_sub(3 * diff)))) == S.into();
            count += valid as u32;
        }
    }

    count
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u32 {
    unsafe { part2_inner::<141>(input.as_bytes()) }
}

#[inline]
fn some_or_0(v: Option<&NonZero<u8>>) -> u8 {
    unsafe { transmute::<Option<NonZero<u8>>, u8>(v.copied()) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part2_inner<const LINE_LEN: usize>(input: &[u8]) -> u32 {
    let mut count = 0;

    let input = transmute::<&[u8], &[NonZero<u8>]>(input);

    for a_pos in input
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == A)
        .map(|(n, _)| n)
    {
        let first_valid = (some_or_0(input.get(a_pos.wrapping_sub(LINE_LEN + 1)))
            ^ some_or_0(input.get(a_pos + LINE_LEN + 1)))
            == 30;

        let both_valid = first_valid
            && (some_or_0(input.get(a_pos.wrapping_sub(LINE_LEN - 1)))
                ^ some_or_0(input.get(a_pos + LINE_LEN - 1)))
                == 30;

        count += both_valid as u32;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn p1_example() {
        assert_eq!(unsafe { part1_inner::<11>(INPUT.as_bytes()) }, 18);
    }

    #[test]
    fn p2_example() {
        assert_eq!(unsafe { part2_inner::<11>(INPUT.as_bytes()) }, 9);
    }

    const REAL_INPUT: &str = include_str!("../input/2024/day4.txt");

    #[test]
    fn p1_real() {
        assert_eq!(unsafe { part1(REAL_INPUT) }, 2_593);
    }

    #[test]
    fn p2_real() {
        assert_eq!(unsafe { part2(REAL_INPUT) }, 1_950);
    }

    #[test]
    fn p1_simplest() {
        assert_eq!(unsafe { part1_inner::<4>("XMAS".as_bytes()) }, 1);
    }

    #[test]
    fn p1_simplest_backward() {
        assert_eq!(unsafe { part1_inner::<4>("SAMX".as_bytes()) }, 1);
    }
}
