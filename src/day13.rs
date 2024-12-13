use std::intrinsics::{unchecked_div, unchecked_rem};

use aoc_runner_derive::aoc;

use crate::{assume, debug, p, Assume, Unreachable};

const BUTTON_FIRST_NUM_OFFSET: usize = 12;
const SECOND_NUM_OFFSET: usize = 4;

const PRIZE_FIRST_NUM_OFFSET: usize = 9;

unsafe fn calc_cost(a_x: i64, a_y: i64, b_x: i64, b_y: i64, target_x: i64, target_y: i64) -> i64 {
    let denom = a_x.unchecked_mul(b_y).unchecked_sub(a_y.unchecked_mul(b_x));
    assume!(denom != 0);
    let num1 = a_x
        .unchecked_mul(target_y)
        .unchecked_sub(a_y.unchecked_mul(target_x));
    let num2 = b_y
        .unchecked_mul(target_x)
        .unchecked_sub(b_x.unchecked_mul(target_y));

    let a_presses = unchecked_div(num1, denom);
    let a_div_cleanly = unchecked_rem(num1, denom) == 0;
    assume!(a_presses >= 0 || !a_div_cleanly);
    let b_presses = unchecked_div(num2, denom);
    let b_div_cleanly = unchecked_rem(num2, denom) == 0;
    assume!(b_presses >= 0 || !b_div_cleanly);

    a_presses
        .unchecked_add(b_presses.unchecked_mul(3))
        .unchecked_mul((a_div_cleanly && b_div_cleanly) as _)
}

unsafe fn read_target(input: &[u8], pos: usize) -> (i64, usize) {
    debug!(
        "Reading line: {}",
        std::str::from_utf8(&input[pos..])
            .unwrap()
            .lines()
            .next()
            .unwrap()
    );
    match *input.get_unchecked(pos..) {
        [n10_000 @ b'0'..=b'9', n1_000 @ b'0'..=b'9', n100 @ b'0'..=b'9', n10 @ b'0'..=b'9', n1 @ b'0'..=b'9', ..] => {
            (p!(i64, n10_000, n1_000, n100, n10, n1), 5)
        }
        [n1_000 @ b'0'..=b'9', n100 @ b'0'..=b'9', n10 @ b'0'..=b'9', n1 @ b'0'..=b'9', ..] => {
            (p!(i64, n1_000, n100, n10, n1), 4)
        }
        [n100 @ b'0'..=b'9', n10 @ b'0'..=b'9', n1 @ b'0'..=b'9', ..] => {
            (p!(i64, n100, n10, n1), 3)
        }
        _ => Unreachable.assume(),
    }
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> i64 {
    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn inner(input: &str) -> i64 {
        let input = input.as_bytes();
        let mut sum = 0;
        let mut pos = 0;

        while input.len() > pos + 2 {
            pos += BUTTON_FIRST_NUM_OFFSET;
            let a_x = p!(
                i64,
                *input.get_unchecked(pos),
                *input.get_unchecked(pos + 1)
            );
            pos += SECOND_NUM_OFFSET + 2;
            let a_y = p!(
                i64,
                *input.get_unchecked(pos),
                *input.get_unchecked(pos + 1)
            );

            pos += 3 + BUTTON_FIRST_NUM_OFFSET;
            let b_x = p!(
                i64,
                *input.get_unchecked(pos),
                *input.get_unchecked(pos + 1)
            );
            pos += SECOND_NUM_OFFSET + 2;
            let b_y = p!(
                i64,
                *input.get_unchecked(pos),
                *input.get_unchecked(pos + 1)
            );

            pos += 3 + PRIZE_FIRST_NUM_OFFSET;
            let (target_x, offset) = read_target(input, pos);
            pos += SECOND_NUM_OFFSET + offset;
            let (target_y, offset) = read_target(input, pos);
            pos += 2 + offset;

            sum += calc_cost(a_x, a_y, b_x, b_y, target_x, target_y);
        }

        sum
    }
    unsafe { inner(input) }
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn inner(input: &str) -> i64 {
        let input = input.as_bytes();
        let mut sum = 0;
        let mut pos = 0;

        while input.len() > pos + 2 {
            pos += BUTTON_FIRST_NUM_OFFSET;
            let a_x = p!(
                i64,
                *input.get_unchecked(pos),
                *input.get_unchecked(pos + 1)
            );
            pos += SECOND_NUM_OFFSET + 2;
            let a_y = p!(
                i64,
                *input.get_unchecked(pos),
                *input.get_unchecked(pos + 1)
            );

            pos += 3 + BUTTON_FIRST_NUM_OFFSET;
            let b_x = p!(
                i64,
                *input.get_unchecked(pos),
                *input.get_unchecked(pos + 1)
            );
            pos += SECOND_NUM_OFFSET + 2;
            let b_y = p!(
                i64,
                *input.get_unchecked(pos),
                *input.get_unchecked(pos + 1)
            );

            pos += 3 + PRIZE_FIRST_NUM_OFFSET;
            let (target_x, offset) = read_target(input, pos);
            pos += SECOND_NUM_OFFSET + offset;
            let (target_y, offset) = read_target(input, pos);
            pos += 2 + offset;

            let target_x = target_x.unchecked_add(10_000_000_000_000);
            let target_y = target_y.unchecked_add(10_000_000_000_000);

            sum += calc_cost(a_x, a_y, b_x, b_y, target_x, target_y);
        }

        sum
    }
    unsafe { inner(input) }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
    "};

    #[test]
    fn p1_example() {
        assert_eq!(part1(INPUT), 480);
    }

    #[test]
    fn p2_example() {
        assert_eq!(part2(INPUT), 875_318_608_908);
    }

    #[test]
    fn real_p1() {
        let input = include_str!("../input/2024/day13.txt");
        assert_eq!(part1(input), 29_877);
    }

    #[test]
    fn real_p2() {
        let input = include_str!("../input/2024/day13.txt");
        assert_eq!(part2(input), 99_423_413_811_305);
    }
}
