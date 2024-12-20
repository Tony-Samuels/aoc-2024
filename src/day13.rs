use std::intrinsics::{unchecked_add, unchecked_div, unchecked_rem, unchecked_shr};

use aoc_runner_derive::aoc;

use crate::{assume, debug, p, ZERO};

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

    if unchecked_rem(num1, denom) != 0 || unchecked_rem(num2, denom) != 0 {
        return 0;
    }

    let i = unchecked_div(num1, denom);
    assume!(i >= 0);
    let j = unchecked_div(num2, denom);
    assume!(j >= 0);

    i.unchecked_add(j.unchecked_mul(3))
}

unsafe fn swar(s: [u8; 8]) -> i64 {
    const ALL_0: u64 = 0x3030303030303030;
    const MASK: u64 = 0x000000FF000000FF;
    const MUL1: u64 = 0x000F424000000064;
    const MUL2: u64 = 0x0000271000000001;

    debug!(
        "{:?}",
        s.iter().copied().map(char::from).collect::<Vec<_>>()
    );
    let mut val = u64::from_ne_bytes(s);
    val = val.unchecked_sub(ALL_0);
    debug!("{:?}", val.to_ne_bytes());
    val = val.wrapping_mul(10).unchecked_add(val.unchecked_shr(8));
    val = unchecked_shr(
        unchecked_add(
            (val & MASK).wrapping_mul(MUL1),
            (val.unchecked_shr(16) & MASK).wrapping_mul(MUL2),
        ),
        32,
    );

    val as i64
}

unsafe fn read_target(input: &[u8], pos: usize) -> (i64, usize) {
    let input = input.get_unchecked(pos..);
    debug!(
        "Reading line: {}",
        std::str::from_utf8(&input).unwrap().lines().next().unwrap()
    );
    if input.get_unchecked(3).is_ascii_digit() {
        if input.get_unchecked(4).is_ascii_digit() {
            let [n1, n2, n3, n4, n5] = input.as_ptr().cast::<[u8; _]>().read_unaligned();
            (swar([ZERO, ZERO, ZERO, n1, n2, n3, n4, n5]), 5)
        } else {
            let [n1, n2, n3, n4] = input.as_ptr().cast::<[u8; _]>().read_unaligned();
            (swar([ZERO, ZERO, ZERO, ZERO, n1, n2, n3, n4]), 4)
        }
    } else {
        let [n1, n2, n3] = input.as_ptr().cast::<[u8; _]>().read_unaligned();
        (swar([ZERO, ZERO, ZERO, ZERO, ZERO, n1, n2, n3]), 3)
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
