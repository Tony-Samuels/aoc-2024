use std::simd::{cmp::SimdPartialEq, Simd};

use aoc_runner_derive::aoc;
use memchr::Memchr;

use crate::debug;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
    unsafe { part1_inner::<141>(input.as_bytes()) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part1_inner<const LINE_LEN: usize>(input: &[u8]) -> u32 {
    let mut count = 0;
    for x_pos in Finder::new(b'X', input) {
        for diff in [
            // Right/left
            1,
            // Up/down
            LINE_LEN,
            // \
            LINE_LEN + 1,
            // /
            LINE_LEN - 1,
        ] {
            let valid = input.get(x_pos + diff) == Some(&b'M');
            let valid = valid && input.get(x_pos + 2 * diff) == Some(&b'A');
            let valid = valid && input.get(x_pos + 3 * diff) == Some(&b'S');
            count += valid as u32;

            let valid = input.get(x_pos.wrapping_sub(diff)) == Some(&b'M');
            let valid = valid && input.get(x_pos.wrapping_sub(2 * diff)) == Some(&b'A');
            let valid = valid && input.get(x_pos.wrapping_sub(3 * diff)) == Some(&b'S');
            count += valid as u32;
        }
    }

    count
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u32 {
    unsafe { part2_inner::<141>(input.as_bytes()) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part2_inner<const LINE_LEN: usize>(input: &[u8]) -> u32 {
    let mut count = 0;
    for a_pos in Finder::new(b'A', input) {
        let first = [
            input.get(a_pos.wrapping_sub(LINE_LEN + 1)),
            input.get(a_pos + LINE_LEN + 1),
        ];
        let first_valid =
            first == [Some(&b'M'), Some(&b'S')] || first == [Some(&b'S'), Some(&b'M')];

        let both_valid = first_valid && {
            let second = [
                input.get(a_pos.wrapping_sub(LINE_LEN - 1)),
                input.get(a_pos + LINE_LEN - 1),
            ];
            second == [Some(&b'M'), Some(&b'S')] || second == [Some(&b'S'), Some(&b'M')]
        };

        count += both_valid as u32;
    }

    count
}

struct Finder<'a> {
    c: u8,
    input: &'a [u8],
    pos: usize,
}

impl<'a> Finder<'a> {
    fn new(c: u8, input: &'a [u8]) -> Self {
        Self { c, input, pos: 0 }
    }
}

impl Iterator for Finder<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
        unsafe fn inner(iter: &mut Finder<'_>) -> Option<usize> {
            if iter.pos < iter.input.len() {
                let line = Simd::<_, 64>::load_or(&iter.input[iter.pos..], Simd::splat(0));
                let offset = line
                    .simd_eq(Simd::splat(iter.c))
                    .to_bitmask()
                    .trailing_zeros() as usize;
                debug!(
                    "{}: {offset}",
                    std::str::from_utf8(line.as_array()).unwrap()
                );
                iter.pos += offset + 1;
                if offset == 64 {
                    let _ = iter.next();
                }

                Some(iter.pos - 1)
            } else {
                None
            }
        }

        unsafe { inner(self) }
    }
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
    fn finder() {
        let finder = &mut Finder::new(b'X', INPUT.as_bytes());
        let memchr = &mut Memchr::new(b'X', INPUT.as_bytes());

        for (count, (finder, memchr)) in finder.zip(&mut *memchr).enumerate() {
            assert_eq!(finder, memchr, "Failed on {count}th time");
        }

        assert_eq!(finder.next(), None);
        assert_eq!(memchr.next(), None);
    }
}
