use std::cmp::min;

use aoc_runner_derive::aoc;
use memchr::{
    arch::all::packedpair::HeuristicFrequencyRank,
    memmem::{find_iter, FindIter, FinderBuilder},
};

use crate::debug;

macro_rules! p {
    ($num:ident) => {
        ($num - b'0') as u32
    };
    ($tens:ident, $units:ident) => {
        (($tens - b'0') * 10 + $units - b'0') as u32
    };
    ($hundreds:ident, $tens:ident, $units:ident) => {
        (($hundreds - b'0') as u32 * 100 + ($tens - b'0') as u32 * 10 + ($units - b'0') as u32)
    };
}

struct Aoc3;
impl HeuristicFrequencyRank for Aoc3 {
    fn rank(&self, byte: u8) -> u8 {
        const TABLE: [u8; 256] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 15, 16, 0, 16, 15, 16, 17, 20, 255, 246, 15, 17, 120, 16, 0, 15, 42, 61, 65,
            64, 65, 63, 66, 63, 64, 60, 18, 15, 15, 0, 16, 15, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 16, 15, 0, 0, 16, 0, 14, 9, 79, 17,
            0, 101, 0, 0, 0, 112, 115, 20, 62, 0, 0, 34, 15, 36, 97, 0, 101, 0, 16, 0, 17, 0, 16,
            15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        TABLE[byte as usize]
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn inner(input: &str) -> u32 {
        let input = input.as_bytes();

        let mut sum = 0;

        let finder = FinderBuilder::new().build_forward_with_ranker(Aoc3, "mul(".as_bytes());
        let iter = finder.find_iter(input);

        debug!("Match counts: {}", iter.clone().count());
        for partial_match_pos in iter {
            match &input[(partial_match_pos + 4)..] {
                [num1 @ b'0'..=b'9', b',', num2 @ b'0'..=b'9', b')', ..] => {
                    sum += p!(num1) * p!(num2);
                }
                [num1_1 @ b'0'..=b'9', num1_2 @ b'0'..=b'9', b',', num2 @ b'0'..=b'9', b')', ..] => {
                    sum += p!(num1_1, num1_2) * p!(num2)
                }
                [num1_1 @ b'0'..=b'9', num1_2 @ b'0'..=b'9', num1_3 @ b'0'..=b'9', b',', num2 @ b'0'..=b'9', b')', ..] => {
                    sum += p!(num1_1, num1_2, num1_3) * p!(num2)
                }
                [num1 @ b'0'..=b'9', b',', num2_1 @ b'0'..=b'9', num2_2 @ b'0'..=b'9', b')', ..] => {
                    sum += p!(num1) * p!(num2_1, num2_2);
                }
                [num1_1 @ b'0'..=b'9', num1_2 @ b'0'..=b'9', b',', num2_1 @ b'0'..=b'9', num2_2 @ b'0'..=b'9', b')', ..] =>
                {
                    sum += p!(num1_1, num1_2) * p!(num2_1, num2_2);
                }
                [num1_1 @ b'0'..=b'9', num1_2 @ b'0'..=b'9', num1_3 @ b'0'..=b'9', b',', num2_1 @ b'0'..=b'9', num2_2 @ b'0'..=b'9', b')', ..] =>
                {
                    sum += p!(num1_1, num1_2, num1_3) * p!(num2_1, num2_2);
                }
                [num1 @ b'0'..=b'9', b',', num2_1 @ b'0'..=b'9', num2_2 @ b'0'..=b'9', num2_3 @ b'0'..=b'9', b')', ..] =>
                {
                    sum += p!(num1) * p!(num2_1, num2_2, num2_3);
                }
                [num1_1 @ b'0'..=b'9', num1_2 @ b'0'..=b'9', b',', num2_1 @ b'0'..=b'9', num2_2 @ b'0'..=b'9', num2_3 @ b'0'..=b'9', b')', ..] =>
                {
                    sum += p!(num1_1, num1_2) * p!(num2_1, num2_2, num2_3);
                }
                [num1_1 @ b'0'..=b'9', num1_2 @ b'0'..=b'9', num1_3 @ b'0'..=b'9', b',', num2_1 @ b'0'..=b'9', num2_2 @ b'0'..=b'9', num2_3 @ b'0'..=b'9', b')', ..] =>
                {
                    let num1 = p!(num1_1, num1_2, num1_3);
                    let num2 = p!(num2_1, num2_2, num2_3);
                    debug!("{num1} * {num2}");
                    sum += num1 * num2;
                }
                arr => {
                    debug!(
                        "Unexpected values: {:?}",
                        std::str::from_utf8(&arr[..(min(arr.len(), 10))]).unwrap()
                    )
                }
            }
        }

        sum
    }

    unsafe { inner(input) }
}

#[derive(Clone)]
struct MultiIter<'a> {
    curr1: Option<usize>,
    curr2: Option<usize>,

    iter1: FindIter<'a, 'a>,
    iter2: FindIter<'a, 'a>,
}

impl<'a> MultiIter<'a> {
    fn new(mut iter1: FindIter<'a, 'a>, mut iter2: FindIter<'a, 'a>) -> Self {
        Self {
            curr1: iter1.next(),
            curr2: iter2.next(),
            iter1,
            iter2,
        }
    }
}

#[cfg(any(test, feature = "debug"))]
impl std::fmt::Debug for MultiIter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MultiIter")
            .field("curr1", &self.curr1)
            .field("curr2", &self.curr2)
            .finish_non_exhaustive()
    }
}

impl Iterator for MultiIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
        unsafe fn inner(iter: &mut MultiIter<'_>) -> Option<usize> {
            debug!("MultiIter: {iter:?}");
            let curr1 = iter.curr1.unwrap_or(usize::MAX);
            let curr2 = iter.curr2.unwrap_or(usize::MAX);

            let rc;

            if curr1 < curr2 {
                rc = iter.curr1;
                iter.curr1 = iter.iter1.next();
            } else {
                rc = iter.curr2;
                iter.curr2 = iter.iter2.next();
            }
            rc
        }
        unsafe { inner(self) }
    }
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn inner(input: &str) -> u32 {
        let mut input = input.as_bytes();

        let finder_mul = FinderBuilder::new().build_forward_with_ranker(Aoc3, "mul(".as_bytes());
        let iter_mul = finder_mul.find_iter(input);

        let finder_dont =
            FinderBuilder::new().build_forward_with_ranker(Aoc3, "don't()".as_bytes());
        let iter_dont = finder_dont.find_iter(input);

        let mut iter = MultiIter::new(iter_mul, iter_dont);
        let finder_do = FinderBuilder::new().build_forward_with_ranker(Aoc3, "do()".as_bytes());

        let mut sum = 0;
        let mut enabled = true;
        while let Some(partial_match_pos) = iter.next() {
            match &input[partial_match_pos..] {
                [b'd', b'o', b'n', b'\'', b't', b'(', b')', ..] => {
                    let Some(pos) = finder_do.find_iter(&input[partial_match_pos..]).next() else {
                        break;
                    };

                    input = &input[partial_match_pos + pos + 4..];
                    let iter_mul = finder_mul.find_iter(input);
                    let iter_dont = finder_dont.find_iter(input);

                    iter = MultiIter::new(iter_mul, iter_dont);
                }
                [b'm', b'u', b'l', b'(', num1 @ b'0'..=b'9', b',', num2 @ b'0'..=b'9', b')', ..] => {
                    sum += p!(num1) * p!(num2);
                }
                [b'm', b'u', b'l', b'(', num1_1 @ b'0'..=b'9', num1_2 @ b'0'..=b'9', b',', num2 @ b'0'..=b'9', b')', ..] => {
                    sum += p!(num1_1, num1_2) * p!(num2)
                }
                [b'm', b'u', b'l', b'(', num1_1 @ b'0'..=b'9', num1_2 @ b'0'..=b'9', num1_3 @ b'0'..=b'9', b',', num2 @ b'0'..=b'9', b')', ..] => {
                    sum += p!(num1_1, num1_2, num1_3) * p!(num2)
                }
                [b'm', b'u', b'l', b'(', num1 @ b'0'..=b'9', b',', num2_1 @ b'0'..=b'9', num2_2 @ b'0'..=b'9', b')', ..] =>
                {
                    sum += p!(num1) * p!(num2_1, num2_2);
                }
                [b'm', b'u', b'l', b'(', num1_1 @ b'0'..=b'9', num1_2 @ b'0'..=b'9', b',', num2_1 @ b'0'..=b'9', num2_2 @ b'0'..=b'9', b')', ..] =>
                {
                    sum += p!(num1_1, num1_2) * p!(num2_1, num2_2);
                }
                [b'm', b'u', b'l', b'(', num1_1 @ b'0'..=b'9', num1_2 @ b'0'..=b'9', num1_3 @ b'0'..=b'9', b',', num2_1 @ b'0'..=b'9', num2_2 @ b'0'..=b'9', b')', ..] =>
                {
                    sum += p!(num1_1, num1_2, num1_3) * p!(num2_1, num2_2);
                }
                [b'm', b'u', b'l', b'(', num1 @ b'0'..=b'9', b',', num2_1 @ b'0'..=b'9', num2_2 @ b'0'..=b'9', num2_3 @ b'0'..=b'9', b')', ..] =>
                {
                    sum += p!(num1) * p!(num2_1, num2_2, num2_3);
                }
                [b'm', b'u', b'l', b'(', num1_1 @ b'0'..=b'9', num1_2 @ b'0'..=b'9', b',', num2_1 @ b'0'..=b'9', num2_2 @ b'0'..=b'9', num2_3 @ b'0'..=b'9', b')', ..] =>
                {
                    sum += p!(num1_1, num1_2) * p!(num2_1, num2_2, num2_3);
                }
                [b'm', b'u', b'l', b'(', num1_1 @ b'0'..=b'9', num1_2 @ b'0'..=b'9', num1_3 @ b'0'..=b'9', b',', num2_1 @ b'0'..=b'9', num2_2 @ b'0'..=b'9', num2_3 @ b'0'..=b'9', b')', ..] =>
                {
                    let num1 = p!(num1_1, num1_2, num1_3);
                    let num2 = p!(num2_1, num2_2, num2_3);
                    sum += num1 * num2;
                }
                arr => {
                    debug!(
                        "Unexpected values: {:?}",
                        std::str::from_utf8(&arr[..(min(arr.len(), 10))]).unwrap()
                    )
                }
            };
        }

        sum
    }

    unsafe { inner(input) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(input), 161);
    }

    #[test]
    fn p2_example() {
        let input = "
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(input), 48);
    }

    const REAL_INPUT: &str = include_str!("../input/2024/day3.txt");

    #[test]
    fn p1_real() {
        assert_eq!(part1(REAL_INPUT), 182_619_815);
    }

    #[test]
    fn p2_real() {
        assert_eq!(part2(REAL_INPUT), 80_747_545);
    }
}
