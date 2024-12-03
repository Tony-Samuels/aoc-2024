use std::cmp::min;

use aoc_runner_derive::aoc;
use memchr::memmem::{find_iter, FindIter};

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

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn inner(input: &str) -> u32 {
        let input = input.as_bytes();

        let mut sum = 0;

        let iter = find_iter(input, "mul(".as_bytes());

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
    curr3: Option<usize>,

    iter1: FindIter<'a, 'a>,
    iter2: FindIter<'a, 'a>,
    iter3: FindIter<'a, 'a>,
}

#[cfg(any(test, feature = "debug"))]
impl std::fmt::Debug for MultiIter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MultiIter")
            .field("curr1", &self.curr1)
            .field("curr2", &self.curr2)
            .field("curr3", &self.curr3)
            .finish_non_exhaustive()
    }
}

impl<'a> MultiIter<'a> {
    fn new(input: &'a [u8]) -> Self {
        let mut iter1 = find_iter(input, b"mul(");
        let curr1 = iter1.next();
        let mut iter2 = find_iter(input, b"do()");
        let curr2 = iter2.next();
        let mut iter3 = find_iter(input, b"don't()");
        let curr3 = iter3.next();

        Self {
            curr1,
            curr2,
            curr3,
            iter1,
            iter2,
            iter3,
        }
    }
}

impl Iterator for MultiIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        debug!("MultiIter: {self:?}");
        let curr1 = self.curr1.unwrap_or(usize::MAX);
        let curr2 = self.curr2.unwrap_or(usize::MAX);
        let curr3 = self.curr3.unwrap_or(usize::MAX);

        let rc;

        if curr1 <= curr2 && curr1 <= curr3 {
            rc = self.curr1;
            self.curr1 = self.iter1.next();
        } else if curr2 <= curr1 && curr2 <= curr3 {
            rc = self.curr2;
            self.curr2 = self.iter2.next();
        } else {
            rc = self.curr3;
            self.curr3 = self.iter3.next();
        }
        rc
    }
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn inner(input: &str) -> u32 {
        let input = input.as_bytes();

        let mut sum = 0;
        let mut enabled = true;
        for partial_match_pos in MultiIter::new(input) {
            if enabled {
                match &input[partial_match_pos..] {
                    [b'd', b'o', b'n', b'\'', b't', b'(', b')', ..] => {
                        enabled = false;
                    }
                    [b'm', b'u', b'l', b'(', num1 @ b'0'..=b'9', b',', num2 @ b'0'..=b'9', b')', ..] =>
                    {
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
            } else {
                enabled = &input[partial_match_pos..][..4] == "do()".as_bytes();
            }
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
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
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
