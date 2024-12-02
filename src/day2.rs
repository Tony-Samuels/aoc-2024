use std::{cmp::Ordering, hint::unreachable_unchecked};

use aoc_runner_derive::aoc;

use crate::Assume as _;

fn parse_2_digits_or_fewer(s: &str) -> i8 {
    (match s.as_bytes() {
        [n] => n - b'0',
        [a, b] => (a - b'0') * 10 + (b - b'0'),
        _ => unsafe { unreachable_unchecked() },
    }) as i8
}

fn check_diff(first: i8, second: i8) -> bool {
    let diff = (first - second).abs();
    (1..=3).contains(&diff)
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let mut count = 0;

    for line in input.lines() {
        let mut iter = line.split_ascii_whitespace().map(parse_2_digits_or_fewer);
        let first = iter.next().assume();
        let second = iter.next().assume();

        if !check_diff(first, second) {
            continue;
        }

        let dir = first.cmp(&second);

        count += iter
            .fold(Some(second), |last, curr| match last {
                Some(last) if last.cmp(&curr) == dir && check_diff(last, curr) => Some(curr),
                _ => None,
            })
            .is_some() as i32;
    }

    count
}

struct Recurse<I> {
    iter: I,
    dir: Ordering,
    penultimate: i8,
    last: i8,
    failure_hit: bool,
}

impl<I> std::fmt::Debug for Recurse<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            iter: _,
            dir,
            penultimate,
            last,
            failure_hit,
        } = self;
        f.debug_struct("Recurse")
            .field("dir", dir)
            .field("penultimate", penultimate)
            .field("last", last)
            .field("failure_hit", failure_hit)
            .finish_non_exhaustive()
    }
}

fn recurse<I>(data: Recurse<I>) -> bool
where
    I: Clone + Iterator<Item = i8>,
{
    let Recurse {
        mut iter,
        dir,
        penultimate,
        last,
        failure_hit,
    } = data;

    let Some(curr) = iter.next() else {
        return true;
    };
    if last.cmp(&curr) == dir && check_diff(last, curr) {
        recurse(Recurse {
            iter,
            dir,
            penultimate: last,
            last: curr,
            failure_hit,
        })
    } else if failure_hit {
        false
    } else {
        let skip_current = recurse(Recurse {
            iter: iter.clone(),
            dir,
            penultimate,
            last,
            failure_hit: true,
        });

        let skip_last = penultimate.cmp(&curr) == dir
            && check_diff(penultimate, curr)
            && recurse(Recurse {
                iter,
                dir,
                penultimate,
                last: curr,
                failure_hit: true,
            });

        skip_current || skip_last
    }
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    let mut count = 0;
    for line in input.lines() {
        let mut iter = line.split_ascii_whitespace().map(parse_2_digits_or_fewer);
        let first = iter.next().assume();
        let second = iter.next().assume();

        let mut dir_check_iter = iter.clone();
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
            Ordering::Equal => continue,
            order => order,
        };

        count += if first.cmp(&second) == dir && check_diff(first, second) {
            recurse(Recurse {
                iter,
                dir,
                penultimate: first,
                last: second,
                failure_hit: false,
            })
        } else {
            let third = iter.next().assume();

            let skip_first = second.cmp(&third) == dir
                && check_diff(second, third)
                && recurse(Recurse {
                    iter: iter.clone(),
                    dir,
                    penultimate: second,
                    last: third,
                    failure_hit: true,
                });

            let skip_second = first.cmp(&third) == dir
                && check_diff(first, third)
                && recurse(Recurse {
                    iter,
                    dir,
                    penultimate: first,
                    last: third,
                    failure_hit: true,
                });

            skip_first || skip_second
        } as i32;
    }

    count
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
    fn example_p1() {
        assert_eq!(part1(INPUT), 2);
    }

    #[test]
    fn example_p2() {
        assert_eq!(part2(INPUT), 4);
    }

    #[test]
    fn example_p2_broken_down() {
        let input = "7 6 4 2 1";
        assert_eq!(part2(input), 1, "{input}");
        let input = "1 2 7 8 9";
        assert_eq!(part2(input), 0, "{input}");
        let input = "9 7 6 2 1";
        assert_eq!(part2(input), 0, "{input}");
        let input = "1 3 2 4 5";
        assert_eq!(part2(input), 1, "{input}");
        let input = "8 6 4 4 1";
        assert_eq!(part2(input), 1, "{input}");
        let input = "1 3 6 7 9";
        assert_eq!(part2(input), 1, "{input}");
    }

    #[test]
    fn p2_remove_first() {
        let input = "10 0 1 2";
        assert_eq!(part2(input), 1, "{input}");
    }

    #[test]
    fn p2_remove_second() {
        let input = "0 10 1 2";
        assert_eq!(part2(input), 1, "{input}");
    }

    #[test]
    fn p2_remove_third() {
        let input = "0 1 10 2";
        assert_eq!(part2(input), 1, "{input}");
    }

    #[test]
    fn p2_remove_fourth() {
        let input = "0 1 2 10";
        assert_eq!(part2(input), 1, "{input}");
    }
}
