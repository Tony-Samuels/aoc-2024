use std::{cmp::min, mem::transmute};

use aoc_runner_derive::aoc;

use crate::{assume, debug};

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
    unsafe { part1_inner::<141>(input.as_bytes()) }
}

const X: u8 = b'X';
const M: u8 = b'M';
const A: u8 = b'A';
const S: u8 = b'S';

const XMAS: u32 = unsafe { transmute::<[u8; 4], u32>([X, M, A, S]) };
const SAMX: u32 = unsafe { transmute::<[u8; 4], u32>([S, A, M, X]) };

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
#[inline]
unsafe fn iter_offset<const MATCH: u8>(
    input: &[u8],
    start: usize,
    end: usize,
) -> impl Iterator<Item = usize> + '_ {
    (start..end).filter(|&n| *input.get_unchecked(n) == MATCH)
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part1_inner<const LINE_LEN: usize>(input: &[u8]) -> u32
where
    [(); { LINE_LEN + 1 }]:,
    [(); { LINE_LEN - 1 }]:,
{
    // Assume trailing new line
    let len = const { LINE_LEN * (LINE_LEN - 1) - 1 };
    assume!(input[len] == b'\n', "Expected trailing new line");

    let iter_offset = iter_offset::<X>;

    let down = line::<LINE_LEN>;
    let up = line_neg::<LINE_LEN>;

    let down_right = line::<{ LINE_LEN + 1 }>;
    let up_left = line_neg::<{ LINE_LEN + 1 }>;

    let down_left = line::<{ LINE_LEN - 1 }>;
    let up_right = line_neg::<{ LINE_LEN - 1 }>;

    let mut count = 0;

    // Top few can't have up, left
    for pos in iter_offset(input, 0, 3) {
        count += right(input, pos);
        debug!("Count: {count}");
        count += down(input, pos);
        debug!("Count: {count}");
        count += down_right(input, pos);
        debug!("Count: {count}");
    }

    // Top few lines can't have up
    for x_pos in iter_offset(input, 3, LINE_LEN * 3) {
        count += left(input, x_pos);
        debug!("Count: {count}");
        count += right(input, x_pos);
        debug!("Count: {count}");
        count += down_left(input, x_pos);
        debug!("Count: {count}");
        count += down(input, x_pos);
        debug!("Count: {count}");
        count += down_right(input, x_pos);
        debug!("Count: {count}");
    }

    // First few on line 4 can't have left
    for pos in iter_offset(input, LINE_LEN * 3, LINE_LEN * 3 + 3) {
        count += up(input, pos);
        debug!("Count: {count}");
        count += up_right(input, pos);
        debug!("Count: {count}");
        count += right(input, pos);
        debug!("Count: {count}");
        count += down(input, pos);
        debug!("Count: {count}");
        count += down_right(input, pos);
        debug!("Count: {count}");
    }

    let main_end = len - (LINE_LEN * 3 + 3);
    for x_pos in iter_offset(input, LINE_LEN * 3 + 3, main_end) {
        count += up_left(input, x_pos);
        debug!("Count: {count}");
        count += up(input, x_pos);
        debug!("Count: {count}");
        count += up_right(input, x_pos);
        debug!("Count: {count}");
        count += left(input, x_pos);
        debug!("Count: {count}");
        count += right(input, x_pos);
        debug!("Count: {count}");
        count += down_left(input, x_pos);
        debug!("Count: {count}");
        count += down(input, x_pos);
        debug!("Count: {count}");
        count += down_right(input, x_pos);
        debug!("Count: {count}");
    }

    // Last few on 4th last line can't have right
    for pos in iter_offset(input, main_end, main_end + 3) {
        count += up_left(input, pos);
        debug!("Count: {count}");
        count += up(input, pos);
        debug!("Count: {count}");
        count += left(input, pos);
        debug!("Count: {count}");
        count += down_left(input, pos);
        debug!("Count: {count}");
        count += down(input, pos);
        debug!("Count: {count}");
    }

    // Bottom few lines can't have down
    for x_pos in iter_offset(input, len - LINE_LEN * 3, len - 3) {
        count += up_left(input, x_pos);
        debug!("Count: {count}");
        count += up(input, x_pos);
        debug!("Count: {count}");
        count += up_right(input, x_pos);
        debug!("Count: {count}");
        count += left(input, x_pos);
        debug!("Count: {count}");
        count += right(input, x_pos);
        debug!("Count: {count}");
    }

    // Last few can't have right, down
    for pos in iter_offset(input, len - 3, len) {
        count += up_left(input, pos);
        debug!("Count: {count}");
        count += up(input, pos);
        debug!("Count: {count}");
        count += left(input, pos);
        debug!("Count: {count}");
    }

    count
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
#[inline]
unsafe fn right(input: &[u8], x_pos: usize) -> u32 {
    ((input.as_ptr().add(x_pos) as *const u32).read_unaligned() == XMAS) as u32
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
#[inline]
unsafe fn left(input: &[u8], x_pos: usize) -> u32 {
    ((input.as_ptr().add(x_pos - 3) as *const u32).read_unaligned() == SAMX) as u32
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
#[inline]
unsafe fn line<const DIFF: usize>(input: &[u8], x_pos: usize) -> u32 {
    (*input.get_unchecked(x_pos + DIFF) == M
        && *input.get_unchecked(x_pos + 2 * DIFF) == A
        && *input.get_unchecked(x_pos + 3 * DIFF) == S) as u32
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
#[inline]
unsafe fn line_neg<const DIFF: usize>(input: &[u8], x_pos: usize) -> u32 {
    (*input.get_unchecked(x_pos - DIFF) == M
        && *input.get_unchecked(x_pos - 2 * DIFF) == A
        && *input.get_unchecked(x_pos - 3 * DIFF) == S) as u32
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u32 {
    unsafe { part2_inner::<141>(input.as_bytes()) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part2_inner<const LINE_LEN: usize>(input: &[u8]) -> u32 {
    // Assume trailing new line
    let len = const { LINE_LEN * (LINE_LEN - 1) - 1 };
    assume!(input[len] == b'\n', "Expected trailing new line");

    let mut count = 0;

    for a_pos in iter_offset::<A>(input, LINE_LEN + 1, len - LINE_LEN - 1) {
        let first_valid = (input.get_unchecked(a_pos - (LINE_LEN + 1))
            ^ input.get_unchecked(a_pos + LINE_LEN + 1))
            == 30;

        let both_valid = first_valid
            && (input.get_unchecked(a_pos - (LINE_LEN - 1))
                ^ input.get_unchecked(a_pos + LINE_LEN - 1))
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
MXMXAXMASX
";

    #[test]
    fn p1_example() {
        assert_eq!(unsafe { part1_inner::<11>(INPUT.as_bytes()) }, 18);
    }

    #[test]
    fn p2_example() {
        assert_eq!(unsafe { part2_inner::<11>(INPUT.as_bytes()) }, 9);
    }

    #[test]
    fn p1_edge_checks() {
        // format!("........\n........\n........\n........\n........\n........\n........\n........"),
        for input in [
            // Right
            format!(
                "XMAS....\n........\n........\n........\n........\n........\n........\n........\n"
            ),
            format!(
                "........\n........\n........\n........\n........\n........\n........\n....XMAS\n"
            ),
            // Left
            format!(
                "SAMX....\n........\n........\n........\n........\n........\n........\n........\n"
            ),
            format!(
                "........\n........\n........\n........\n........\n........\n........\n....SAMX\n"
            ),
            // Down
            format!(
                "X.......\nM.......\nA.......\nS.......\n........\n........\n........\n........\n"
            ),
            format!(
                "........\n........\n........\n........\n.......X\n.......M\n.......A\n.......S\n"
            ),
            // Up
            format!(
                "S.......\nA.......\nM.......\nX.......\n........\n........\n........\n........\n"
            ),
            format!(
                "........\n........\n........\n........\n.......S\n.......A\n.......M\n.......X\n"
            ),
            // Down right
            format!(
                "X.......\n.M......\n..A.....\n...S....\n........\n........\n........\n........\n"
            ),
            format!(
                "........\n........\n........\n........\n....X...\n.....M..\n......A.\n.......S\n"
            ),
            // Down left
            format!(
                "...X....\n..M.....\n.A......\nS.......\n........\n........\n........\n........\n"
            ),
            format!(
                "........\n........\n........\n........\n.......X\n......M.\n.....A..\n....S...\n"
            ),
            // Up right
            format!(
                "...S....\n..A.....\n.M......\nX.......\n........\n........\n........\n........\n"
            ),
            format!(
                "........\n........\n........\n........\n.......S\n......A.\n.....M..\n....X...\n"
            ),
            // Up left
            format!(
                "S.......\n.A......\n..M.....\n...X....\n........\n........\n........\n........\n"
            ),
            format!(
                "........\n........\n........\n........\n....S...\n.....A..\n......M.\n.......X\n"
            ),
        ] {
            debug!("New input");
            assert!(
                input.lines().map(str::len).all(|n| n == 8),
                "Incorrect line length"
            );
            assert_eq!(
                unsafe { part1_inner::<9>(input.as_bytes()) },
                1,
                "\n{input}"
            );
        }
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
    fn p2_reduced_range() {
        let input = "M.S
.A.
M.S
"
        .as_bytes();
        assert_eq!(unsafe { part2_inner::<4>(input) }, 1);
    }
}
