use std::cmp::{min, Ordering};

use aoc_runner_derive::aoc;

use crate::{debug, Assume as _, Unreachable};

#[derive(Clone, Copy)]
struct LineNumIter<'a> {
    inner: &'a [u8],
    last_ended_line: bool,
    line_just_ended: bool,
}

#[cfg(any(test, feature = "debug"))]
impl std::fmt::Debug for LineNumIter<'_> {
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
            .field("last_ended_line", &self.last_ended_line)
            .field("line_just_ended", &self.line_just_ended)
            .finish()
    }
}

impl<'a> LineNumIter<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            inner: s.as_bytes(),
            last_ended_line: false,
            line_just_ended: true,
        }
    }

    fn jump_to_next_line(&mut self) {
        if !self.line_just_ended {
            debug!("Jumping to end of line: {self:?}");
            while self.next().is_some() {}
            debug!("Jumped to end of line: {self:?}");
        }
    }
}

impl Iterator for LineNumIter<'_> {
    type Item = i8;

    fn next(&mut self) -> Option<Self::Item> {
        #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
        unsafe fn inner(iter: &mut LineNumIter<'_>) -> Option<i8> {
            if iter.last_ended_line {
                debug!("Ending line");
                iter.last_ended_line = false;
                iter.line_just_ended = true;
                return None;
            }

            iter.line_just_ended = false;

            let len = iter.inner.len();

            match &iter.inner[..min(3, len)] {
                [n @ b'0'..=b'9', b' ' | b'\n', ..] | [n @ b'0'..=b'9'] => {
                    if iter.inner.get(1).is_some_and(|&c| c == b'\n') {
                        debug!("Line end reached");
                        iter.last_ended_line = true;
                    }

                    iter.inner = &iter.inner[min(2, len)..];
                    Some((n - b'0') as i8)
                }
                [n1 @ b'0'..=b'9', n2 @ b'0'..=b'9', b' ' | b'\n']
                | [n1 @ b'0'..=b'9', n2 @ b'0'..=b'9'] => {
                    if iter.inner.get(2).is_some_and(|&c| c == b'\n') {
                        debug!("Line end reached");
                        iter.last_ended_line = true;
                    }

                    iter.inner = &iter.inner[min(3, len)..];
                    Some(((n1 - b'0') * 10 + n2 - b'0') as i8)
                }
                [] => None,
                _ => Unreachable.assume(),
            }
        }

        unsafe { inner(self) }
    }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn check_diff(first: i8, second: i8) -> bool {
    let diff = (first - second).abs();
    diff == 1 || diff == 2 || diff == 3
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn inner(input: &str) -> i32 {
        let mut count = 1_000;

        let iter = &mut LineNumIter::new(input);
        while let Some(first) = {
            // Ensure the iterator has reached the end of the line (may not have happened due to copying)
            iter.jump_to_next_line();
            iter.next()
        } {
            let second = iter.next().assume();

            if !check_diff(first, second) {
                count -= 1;
                continue;
            }

            let dir = first.cmp(&second);

            let sub = iter
                .try_fold(second, |last, curr| {
                    if last.cmp(&curr) == dir && check_diff(last, curr) {
                        Some(curr)
                    } else {
                        None
                    }
                })
                .is_none();
            count -= sub as i32;

            if !sub {
                break;
            }
        }

        count
    }
    unsafe { inner(input) }
}

#[derive(Debug)]
struct Recurse {
    dir: Ordering,
    penultimate: i8,
    last: i8,
    failure_hit: bool,
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn recurse(iter: &mut LineNumIter, hit_failure: &mut bool, data: Recurse) -> bool {
    let Recurse {
        dir,
        penultimate,
        last,
        failure_hit,
    } = data;

    let Some(curr) = iter.next() else {
        return true;
    };
    if last.cmp(&curr) == dir && check_diff(last, curr) {
        recurse(
            iter,
            hit_failure,
            Recurse {
                dir,
                penultimate: last,
                last: curr,
                failure_hit,
            },
        )
    } else if failure_hit {
        false
    } else {
        *hit_failure = true;
        let skip_current = recurse(
            &mut iter.clone(),
            &mut false,
            Recurse {
                dir,
                penultimate,
                last,
                failure_hit: true,
            },
        );

        let skip_last = penultimate.cmp(&curr) == dir
            && check_diff(penultimate, curr)
            && recurse(
                iter,
                &mut false,
                Recurse {
                    dir,
                    penultimate,
                    last: curr,
                    failure_hit: true,
                },
            );

        skip_current || skip_last
    }
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn inner(input: &str) -> i32 {
        let mut count = 1_000;

        let iter = &mut LineNumIter::new(input);
        while let Some(first) = {
            // Ensure the iterator has reached the end of the line (may not have happened due to copying)
            iter.jump_to_next_line();
            iter.next()
        } {
            let second = iter.next().assume();

            let mut dir_check_iter = *iter;
            let third = dir_check_iter.next().assume();
            let fourth = dir_check_iter.next().assume();

            let mut inc_count = 0;
            let mut dec_count = 0;

            for pair in [(first, second), (second, third), (third, fourth)] {
                match pair.0.cmp(&pair.1) {
                    Ordering::Less => dec_count += 1,
                    Ordering::Equal => (),
                    Ordering::Greater => inc_count += 1,
                }
            }

            let dir = match inc_count.cmp(&dec_count) {
                Ordering::Equal => {
                    count -= 1;
                    continue;
                }
                order => order,
            };

            let valid = if first.cmp(&second) == dir && check_diff(first, second) {
                let mut hit_failure = false;
                let valid = recurse(
                    iter,
                    &mut hit_failure,
                    Recurse {
                        dir,
                        penultimate: first,
                        last: second,
                        failure_hit: false,
                    },
                );
                if valid && !hit_failure {
                    break;
                }

                valid
            } else {
                let third = iter.next().assume();

                let skip_first = || {
                    second.cmp(&third) == dir
                        && check_diff(second, third)
                        && recurse(
                            &mut iter.clone(),
                            &mut false,
                            Recurse {
                                dir,
                                penultimate: second,
                                last: third,
                                failure_hit: true,
                            },
                        )
                };

                let skip_second = || {
                    first.cmp(&third) == dir
                        && check_diff(first, third)
                        && recurse(
                            &mut iter.clone(),
                            &mut false,
                            Recurse {
                                dir,
                                penultimate: first,
                                last: third,
                                failure_hit: true,
                            },
                        )
                };

                skip_first() || skip_second()
            };

            count -= !valid as i32;

            debug!("Line finished, count is {count}");
        }

        count
    }
    unsafe { inner(input) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn data() {
        let mut data = LineNumIter::new(INPUT);
        let mut index = 0;

        for line in INPUT.lines() {
            for num in line.split_whitespace().map(|n| n.parse::<i8>().unwrap()) {
                assert_eq!(data.next(), Some(num), "Invalid output at {index}");
                index += 1;
            }

            assert_eq!(data.next(), None, "Invalid output at {index}");
        }

        assert_eq!(data.next(), None);
        assert_eq!(data.next(), None);
        assert_eq!(data.next(), None);
    }

    #[test]
    fn real_p1() {
        let input = include_str!("../input/2024/day2.txt");
        assert_eq!(part1(input), 287);
    }

    #[test]
    fn real_p2() {
        let input = include_str!("../input/2024/day2.txt");
        assert_eq!(part2(input), 354);
    }
}
