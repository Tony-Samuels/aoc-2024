use std::{
    arch::x86_64::{
        _mm_madd_epi16, _mm_maddubs_epi16, _mm_packus_epi32, _mm_set_epi16, _mm_set_epi8,
    },
    intrinsics::{unchecked_div, unchecked_mul, unchecked_rem, unchecked_sub},
    mem::transmute,
    simd::{cmp::SimdPartialEq, Simd},
};

use aoc_runner_derive::aoc;

use crate::{debug, ArrayVec, Assume, Unreachable};

const ZERO: u8 = b'0';
const ZEROES_128: u128 = {
    let mut num = ZERO as u128;
    let mut last = 0;
    while num != last {
        last = num;
        num = (num << 8) + ZERO as u128;
    }
    num
};
const EOL: u8 = b'\n';

const GATHER_INDICES: [[usize; 16]; 17] = [
    [usize::MAX; 16],
    [
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        0,
    ],
    [
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        0,
        1,
    ],
    [
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        0,
        1,
        2,
    ],
    [
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        0,
        1,
        2,
        3,
    ],
    [
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        0,
        1,
        2,
        3,
        4,
    ],
    [
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        0,
        1,
        2,
        3,
        4,
        5,
    ],
    [
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        0,
        1,
        2,
        3,
        4,
        5,
        6,
    ],
    [
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
    ],
    [
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
    ],
    [
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
    ],
    [
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
    ],
    [
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
    ],
    [
        usize::MAX,
        usize::MAX,
        usize::MAX,
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
    ],
    [
        usize::MAX,
        usize::MAX,
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
    ],
    [usize::MAX, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
];

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn simd_parse(input: &[u8], index: [usize; 16]) -> u64 {
    let nums = Simd::gather_or_default(input, Simd::from_array(index));
    let mult = _mm_set_epi8(1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10);
    let nums = _mm_maddubs_epi16(nums.into(), mult);
    let mult = _mm_set_epi16(1, 100, 1, 100, 1, 100, 1, 100);
    let nums = _mm_madd_epi16(nums, mult);
    let nums = _mm_packus_epi32(nums, nums);
    let mult = _mm_set_epi16(0, 0, 0, 0, 1, 10_000, 1, 10_000);
    let nums = _mm_madd_epi16(nums, mult);
    let nums: Simd<u64, 2> = nums.into();
    let num = nums[0];

    (num & 0xffffffff) * 100_000_000 + (num >> 32)
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn parse_num(input: &[u8]) -> (u64, usize) {
    let input_simd = Simd::<u8, 16>::load_or_default(input);
    let len = (input_simd.simd_eq(Simd::splat(EOL))
        | input_simd.simd_eq(Simd::splat(b' '))
        | input_simd.simd_eq(Simd::splat(b':')))
    .to_bitmask()
    .trailing_zeros() as usize;
    let input: [u8; 16] = unchecked_sub(transmute::<_, u128>(input_simd), ZEROES_128).to_ne_bytes();
    let num = match len {
        1 => input[0] as _,
        2 => input[0] as u64 * 10 + input[1] as u64,
        3 => input[0] as u64 * 100 + input[1] as u64 * 10 + input[2] as u64,
        4..=16 => simd_parse(&input[..len], *GATHER_INDICES.get_unchecked(len)),
        _ => {
            debug!(
                "Unexpected number length {len} in {}",
                std::str::from_utf8(&input[..len]).unwrap()
            );
            Unreachable.assume();
        }
    };

    (num, len)
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn inner(input: &str) -> u64 {
        let input = input.as_bytes();
        let mut count = 0;
        let mut vec = ArrayVec::<12, u64>::new();
        let mut pos = 0;

        while input.len() > pos + 3 {
            vec.clear();

            let (target, bytes) = parse_num(input.get_unchecked(pos..));
            pos += bytes + 2;
            loop {
                let (next, bytes) = parse_num(input.get_unchecked(pos..));
                vec.push_unchecked(next);
                let term = *input.get_unchecked(pos + bytes);
                pos += bytes + 1;
                if term == EOL {
                    break;
                }
            }

            let math_checks_out = recurse_p1(target, vec);
            count += target * math_checks_out as u64;
        }

        count
    }
    unsafe { inner(input) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn recurse_p1<const N: usize>(target: u64, mut nums: ArrayVec<N, u64>) -> bool {
    let num = nums.pop_unchecked();
    if nums.len == 0 {
        num == target
    } else {
        (unchecked_rem(target, num) == 0 && recurse_p1(unchecked_div(target, num), nums))
            || (target >= num && recurse_p1(unchecked_sub(target, num), nums))
    }
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn inner(input: &str) -> u64 {
        let input = input.as_bytes();
        let mut count = 0;
        let mut vec = ArrayVec::<12, u64>::new();
        let mut pos = 0;

        while input.len() > pos + 3 {
            vec.clear();

            let (target, bytes) = parse_num(input.get_unchecked(pos..));
            pos += bytes + 2;
            loop {
                let (next, bytes) = parse_num(input.get_unchecked(pos..));
                vec.push_unchecked(next);
                let term = *input.get_unchecked(pos + bytes);
                pos += bytes + 1;
                if term == EOL {
                    break;
                }
            }

            let math_checks_out = recurse_p2(target, vec);
            count += target * math_checks_out as u64;
        }

        count
    }
    unsafe { inner(input) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn recurse_p2<const N: usize>(target: u64, mut nums: ArrayVec<N, u64>) -> bool {
    let num = nums.pop_unchecked();
    if nums.len == 0 {
        num == target
    } else {
        (unchecked_rem(target, num) == 0 && recurse_p2(unchecked_div(target, num), nums))
            || (target >= num && recurse_p2(unchecked_sub(target, num), nums)
                || ({
                    let mut temp = num;
                    let mut tens = 1;
                    while temp > 0 {
                        tens = unchecked_mul(tens, 10);
                        temp = unchecked_div(temp, 10);
                    }
                    unchecked_rem(target, tens) == num
                        && recurse_p2(unchecked_div(target, tens), nums)
                }))
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
        assert_eq!(part2(input), 500_335_179_214_836);
    }
}
