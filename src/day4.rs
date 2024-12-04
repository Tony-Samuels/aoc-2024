use aoc_runner_derive::aoc;
use memchr::Memchr;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
    unsafe { part1_inner::<141>(input.as_bytes()) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part1_inner<const LINE_LEN: usize>(input: &[u8]) -> u32 {
    let mut count = 0;
    for x_pos in input
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == b'X')
        .map(|(n, _)| n)
    {
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
    for a_pos in input
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == b'A')
        .map(|(n, _)| n)
    {
        let first_valid = (input.get(a_pos.wrapping_sub(LINE_LEN + 1)).unwrap_or(&0)
            ^ input.get(a_pos + LINE_LEN + 1).unwrap_or(&0))
            == 30;

        let both_valid = first_valid
            && (input.get(a_pos.wrapping_sub(LINE_LEN - 1)).unwrap_or(&0)
                ^ input.get(a_pos + LINE_LEN - 1).unwrap_or(&0))
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
}
