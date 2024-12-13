use aoc_runner_derive::aoc;
use memchr::{arch::all::packedpair::HeuristicFrequencyRank, memmem::FinderBuilder, Memchr};

use crate::{debug, p};

struct Aoc3;
impl HeuristicFrequencyRank for Aoc3 {
    fn rank(&self, byte: u8) -> u8 {
        [
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
        ][byte as usize]
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    unsafe { inner_part1(input.as_bytes()) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_part1(input: &[u8]) -> u32 {
    let mut sum = 0;

    let iter = Memchr::new(b'u', input);

    debug!("Match counts: {}", iter.clone().count());
    for partial_match_pos in iter {
        match input[partial_match_pos + 1..] {
            [b'l', b'(', num1 @ b'0'..=b'9', b',', num2 @ b'0'..=b'9', b')', ..] => {
                sum += p!(u32, num1) * p!(u32, num2);
            }
            [b'l', b'(', num1_1 @ b'0'..=b'9', num1_2 @ b'0'..=b'9', b',', num2 @ b'0'..=b'9', b')', ..] => {
                sum += p!(u32, num1_1, num1_2) * p!(u32, num2)
            }
            [b'l', b'(', num1_1 @ b'0'..=b'9', num1_2 @ b'0'..=b'9', num1_3 @ b'0'..=b'9', b',', num2 @ b'0'..=b'9', b')', ..] => {
                sum += p!(u32, num1_1, num1_2, num1_3) * p!(u32, num2)
            }
            [b'l', b'(', num1 @ b'0'..=b'9', b',', num2_1 @ b'0'..=b'9', num2_2 @ b'0'..=b'9', b')', ..] =>
            {
                sum += p!(u32, num1) * p!(u32, num2_1, num2_2);
            }
            [b'l', b'(', num1_1 @ b'0'..=b'9', num1_2 @ b'0'..=b'9', b',', num2_1 @ b'0'..=b'9', num2_2 @ b'0'..=b'9', b')', ..] =>
            {
                sum += p!(u32, num1_1, num1_2) * p!(u32, num2_1, num2_2);
            }
            [b'l', b'(', num1_1 @ b'0'..=b'9', num1_2 @ b'0'..=b'9', num1_3 @ b'0'..=b'9', b',', num2_1 @ b'0'..=b'9', num2_2 @ b'0'..=b'9', b')', ..] =>
            {
                sum += p!(u32, num1_1, num1_2, num1_3) * p!(u32, num2_1, num2_2);
            }
            [b'l', b'(', num1 @ b'0'..=b'9', b',', num2_1 @ b'0'..=b'9', num2_2 @ b'0'..=b'9', num2_3 @ b'0'..=b'9', b')', ..] =>
            {
                sum += p!(u32, num1) * p!(u32, num2_1, num2_2, num2_3);
            }
            [b'l', b'(', num1_1 @ b'0'..=b'9', num1_2 @ b'0'..=b'9', b',', num2_1 @ b'0'..=b'9', num2_2 @ b'0'..=b'9', num2_3 @ b'0'..=b'9', b')', ..] =>
            {
                sum += p!(u32, num1_1, num1_2) * p!(u32, num2_1, num2_2, num2_3);
            }
            [b'l', b'(', num1_1 @ b'0'..=b'9', num1_2 @ b'0'..=b'9', num1_3 @ b'0'..=b'9', b',', num2_1 @ b'0'..=b'9', num2_2 @ b'0'..=b'9', num2_3 @ b'0'..=b'9', b')', ..] =>
            {
                let num1 = p!(u32, num1_1, num1_2, num1_3);
                let num2 = p!(u32, num2_1, num2_2, num2_3);
                debug!("{num1} * {num2}");
                sum += num1 * num2;
            }
            _ => (),
        }
    }

    sum
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn inner(input: &str) -> u32 {
        let mut input = input.as_bytes();

        let finder_dont = FinderBuilder::new().build_forward_with_ranker(Aoc3, "'t".as_bytes());
        let finder_do = FinderBuilder::new().build_forward_with_ranker(Aoc3, "do(".as_bytes());

        let mut sum = 0;
        while let Some(pos) = finder_dont.find(input) {
            debug!("Don't position {pos}");
            sum += inner_part1(&input[..pos - 3]);

            let pos = pos + finder_do.find(&input[pos + 4..]).unwrap_or(input.len() - 4);
            debug!("Do position {pos}");
            input = &input[pos + 4..];
        }

        sum + inner_part1(input)
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
