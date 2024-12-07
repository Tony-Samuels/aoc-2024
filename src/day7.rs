use aoc_runner_derive::aoc;

use crate::{assume, debug, Assume, Unreachable};

const EOL: u8 = b'\n';

#[derive(Clone, Copy)]
struct LineCharIter<'a> {
    inner: &'a [u8],
    line_just_ended: bool,
}

#[cfg(any(test, feature = "debug"))]
impl std::fmt::Debug for LineCharIter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LineNumIter")
            .field(
                "inner",
                &std::str::from_utf8(self.inner)
                    .unwrap()
                    .lines()
                    .next()
                    .unwrap_or_default(),
            )
            .field("line_just_ended", &self.line_just_ended)
            .finish()
    }
}

impl<'a> LineCharIter<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            inner: s.as_bytes(),
            line_just_ended: true,
        }
    }

    fn jump_to_next_line(&mut self) {
        if !self.line_just_ended {
            debug!("Jumping to end of line: {self:?}");
            self.take_while(|&c| c != EOL).for_each(drop);
            debug!("Jumped to end of line: {self:?}");
        }
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl Iterator for LineCharIter<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
        unsafe fn inner(iter: &mut LineCharIter<'_>) -> Option<u8> {
            if iter.inner.len() == 0 {
                return None;
            }

            let c = *iter.inner.get_unchecked(0);
            iter.inner = iter.inner.get_unchecked(1..);
            iter.line_just_ended = c == EOL;
            Some(c)
        }

        unsafe { inner(self) }
    }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn get_num(input: &mut LineCharIter<'_>) -> (u64, u8) {
    debug!("Fetching num from {input:?}");
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
        let mut input = LineCharIter::new(input);
        let mut count = 0;

        while input.len() > 3 {
            input.jump_to_next_line();
            let (target, term) = get_num(&mut input);
            assume!(
                term == b':',
                "Unexpected start of line terminator after {target}: {term} ({})",
                term as char
            );
            let space = input.next().assume();
            assume!(
                space == b' ',
                "Unexpected char after start: {space} ({})",
                space as char
            );

            let (first_num, space) = get_num(&mut input);
            assume!(
                space == b' ',
                "Unexpected char after first num: {space} ({})",
                space as char
            );

            let math_checks_out = recurse_p1(&mut input, target, first_num);
            debug!("Math checks out for {target}: {math_checks_out}");
            count += target * math_checks_out as u64;
        }

        debug!("Remaining input: {}", input.len());

        count
    }
    unsafe { inner(input) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn recurse_p1(input: &mut LineCharIter<'_>, target: u64, acc: u64) -> bool {
    let (num, term) = get_num(&mut *input);
    let mul = num * acc;
    let add = num + acc;
    debug!("Checking {num} for {target} with {acc}: * {mul}, + {add}");

    if term == EOL {
        (target == mul) || (target == add)
    } else {
        (mul <= target && recurse_p1(&mut input.clone(), target, mul))
            || (add <= target && recurse_p1(input, target, add))
    }
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn inner(input: &str) -> u64 {
        let mut input = LineCharIter::new(input);
        let mut count = 0;

        while {
            input.jump_to_next_line();
            input.len() > 3
        } {
            let (target, term) = get_num(&mut input);
            assume!(
                term == b':',
                "Unexpected start of line terminator after {target}: {term} ({})",
                term as char
            );
            let space = input.next().assume();
            assume!(
                space == b' ',
                "Unexpected char after start: {space} ({})",
                space as char
            );

            let (first_num, space) = get_num(&mut input);
            assume!(
                space == b' ',
                "Unexpected char after first num: {space} ({})",
                space as char
            );

            let math_checks_out = recurse_p2(&mut input, target, first_num);
            debug!("Math checks out for {target}: {math_checks_out}");
            count += target * math_checks_out as u64;
        }

        debug!("Remaining input: {}", input.len());

        count
    }
    unsafe { inner(input) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn recurse_p2(input: &mut LineCharIter<'_>, target: u64, acc: u64) -> bool {
    let (num, term) = get_num(&mut *input);
    assume!(num != 0);
    assume!(term == EOL || term == b' ', "Unexpected terminator");
    let mul = num * acc;
    let add = num + acc;
    let cat = acc * 10u64.pow(num.ilog10() + 1) + num;
    debug!("Checking {num} for {target} with {acc}: * {mul}, + {add}, || {cat}");

    if term == EOL {
        (target == mul) || (target == add) || (target == cat)
    } else {
        (mul <= target && recurse_p2(&mut input.clone(), target, mul))
            || (add <= target && recurse_p2(&mut input.clone(), target, add))
            || (cat <= target && recurse_p2(input, target, cat))
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
        assert_eq!(part2(input), 7);
    }
}
