use std::{
    cmp::min,
    simd::{
        cmp::{SimdPartialEq as _, SimdPartialOrd as _},
        Mask, Simd,
    },
};

use aoc_runner_derive::aoc;
use memchr::{
    arch::all::packedpair::HeuristicFrequencyRank,
    memmem::{find_iter, FindIter, FinderBuilder},
    Memchr,
};

use crate::{debug, Assume, Unreachable};

macro_rules! p {
    ($num:expr) => {
        ($num as u32)
    };
    ($tens:expr, $units:expr) => {
        ($tens as u32 * 10 + $units as u32)
    };
    ($hundreds:expr, $tens:expr, $units:expr) => {
        ($hundreds as u32 * 100 + $tens as u32 * 10 + $units as u32)
    };
}

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
unsafe fn inner_part1(mut input: &[u8]) -> u32 {
    let mut sum = 0;

    while !input.is_empty() {
        debug!(
            "Line: {}",
            std::str::from_utf8(&input[..min(64, input.len())]).unwrap()
        );
        let line = Simd::<u8, 64>::load_or(input, Simd::splat(0));
        let u_pos = line
            .simd_eq(Simd::splat(b'u'))
            .to_bitmask()
            .trailing_zeros() as usize;
        if u_pos != 0 {
            debug!("u at {u_pos}");
            input = &input[min(u_pos, input.len())..];
            continue;
        }

        // ul(000,000)
        if line[2] != b'(' {
            debug!("Missing (");
            input = &input[min(3, input.len())..];
            continue;
        }

        let line = line.rotate_elements_left::<3>().resize::<16>(0);
        let digits = line - Simd::splat(b'0');
        let digit_mask = digits.simd_lt(Simd::splat(10));

        let comma_pos = digit_mask.to_bitmask().trailing_ones() as usize;
        if line[comma_pos] != b',' {
            debug!("Missing ,");
            input = &input[min(comma_pos, input.len())..];
            continue;
        }

        let shifted_digit_mask = digit_mask.to_bitmask() >> (comma_pos + 1);
        let end_bracket_pos = comma_pos + 1 + shifted_digit_mask.trailing_ones() as usize;
        input = &input[min(end_bracket_pos, input.len())..];
        if line[end_bracket_pos] != b')' {
            debug!("{}", std::str::from_utf8(line.as_array()).unwrap());
            debug!("Missing ) at {end_bracket_pos}");
            debug!("{shifted_digit_mask:b}, {comma_pos}");
            continue;
        }

        // let tens_mask = Mask::from_bitmask(digit_mask.to_bitmask() >> 1) & digit_mask;
        // let hundreds_mask = Mask::from_bitmask(tens_mask.to_bitmask() >> 1) & digit_mask;
        // let units_mask = digit_mask & !tens_mask & !hundreds_mask;
        // let fake_tens_mask = units_mask.to_bitmask() >> 1;

        let digit_bitmask = digit_mask.to_bitmask() & 0b1111111;
        debug!(
            "\nline: {}\nmask: {digit_bitmask:b}",
            std::str::from_utf8(line.as_array()).unwrap()
        );
        sum += match digit_bitmask {
            0b101 => p!(digits[0]) * p!(digits[2]),
            0b1011 => p!(digits[0], digits[1]) * p!(digits[3]),
            0b10111 => p!(digits[0], digits[1], digits[2]) * p!(digits[4]),
            0b1101 => p!(digits[0]) * p!(digits[2], digits[3]),
            0b11011 => p!(digits[0], digits[1]) * p!(digits[3], digits[4]),
            0b110111 => p!(digits[0], digits[1], digits[2]) * p!(digits[4], digits[5]),
            0b11101 => p!(digits[0]) * p!(digits[2], digits[3], digits[4]),
            0b111011 => p!(digits[0], digits[1]) * p!(digits[3], digits[4], digits[5]),
            0b1110111 => p!(digits[0], digits[1], digits[2]) * p!(digits[4], digits[5], digits[6]),
            mask => {
                debug!("Unreachable mask: {mask:b}");
                Unreachable.assume();
            }
        };
        debug!("Sum: {sum}");

        input = &input[min(end_bracket_pos, input.len())..];
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
