use core::str;
use std::simd::{cmp::SimdPartialOrd as _, Simd};

use aoc_runner_derive::aoc;

use crate::{debug, ArrayVec, BigBitSet, BitIterU64 as BitIter, IndexI8 as Index, ZERO};

const ANTENNA_OPTS: usize = (b'z' - b'0' + 1) as usize;
const _: () = {
    assert!(b'0' < b'Z');
    assert!(b'Z' < b'z');
};

#[aoc(day8, part1)]
pub fn part1(input: &str) -> i32 {
    unsafe { inner_p1(input) }
}

const DIM: usize = 50;
const SECOND_HALF_START: usize = 50 - 32;

#[inline]
unsafe fn ptr_add(ptr: *const u8, val: usize) -> *const u8 {
    (ptr as usize).unchecked_add(val) as _
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_p1(input: &str) -> i32 {
    let mut input = input.as_bytes().as_ptr();

    let mut antennae = [ArrayVec::<4, Index<DIM>>::new_unchecked(); ANTENNA_OPTS];
    let mut antinodes = BigBitSet::<{ DIM * (DIM + 1) / 8 + 1 }>::new();
    let mut count = 0;

    let zeroes = Simd::splat(ZERO);

    for y in 0..DIM {
        let first_half = input.cast::<Simd<u8, 32>>().read_unaligned();
        let second_half = ptr_add(input, SECOND_HALF_START as _)
            .cast::<Simd<u8, 32>>()
            .read_unaligned();
        let mask = first_half.simd_ge(zeroes).to_bitmask()
            | second_half
                .simd_ge(zeroes)
                .to_bitmask()
                .unchecked_shl(SECOND_HALF_START as _);
        debug!("Mask:\n{mask:050b}\n{}", {
            let mut line = input.cast::<[u8; DIM]>().read_unaligned();
            line.reverse();
            str::from_utf8(&line).unwrap().to_owned()
        });

        for x in BitIter(mask) {
            let pos = Index {
                x: x as _,
                y: y as _,
            };
            let c: u8 = *ptr_add(input, x);
            debug!("Found char {c:x} ({}) at {pos:?}", c as char);
            let antennae = antennae.get_unchecked_mut(c.unchecked_sub(ZERO) as usize);

            for &antenna in antennae.into_iter() {
                let diff = pos - antenna;
                debug!("{antenna:?}, {pos:?}, {diff:?}");
                if let Some(pos) = (antenna - diff).checked_to() {
                    let (antinode_index, antinode_mask) = antinodes.calc_byte_mask(pos);
                    let antinode = antinodes.get_byte_unchecked_mut(antinode_index);
                    count += ((*antinode & antinode_mask) == 0) as i32;
                    *antinode |= antinode_mask;
                }
                if let Some(pos) = (pos + diff).checked_to() {
                    let (antinode_index, antinode_mask) = antinodes.calc_byte_mask(pos);
                    let antinode = antinodes.get_byte_unchecked_mut(antinode_index);
                    count += ((*antinode & antinode_mask) == 0) as i32;
                    *antinode |= antinode_mask;
                }
            }

            antennae.push_unchecked(pos);
        }

        input = ptr_add(input, const { DIM + 1 } as _);
    }

    // debug!("Final map:\n{}", {
    //     let mut s = String::new();
    //     for (i, &c) in input.iter().enumerate() {
    //         s.push(if antinodes.get_unchecked(i) {
    //             if c == b'.' {
    //                 '+'
    //             } else {
    //                 '#'
    //             }
    //         } else {
    //             c as char
    //         })
    //     }
    //     s
    // });

    count
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> i32 {
    unsafe { inner_p2(input) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_p2(input: &str) -> i32 {
    let mut input = input.as_bytes().as_ptr();

    let mut antennae = [ArrayVec::<4, Index<DIM>>::new_unchecked(); ANTENNA_OPTS];
    let mut antinodes = [false; DIM * (DIM + 1)];
    let mut count = 0;

    let zeroes = Simd::splat(ZERO);

    for y in 0..DIM {
        let first_half = input.cast::<Simd<u8, 32>>().read_unaligned();
        let second_half = ptr_add(input, SECOND_HALF_START as _)
            .cast::<Simd<u8, 32>>()
            .read_unaligned();
        let mask = first_half.simd_ge(zeroes).to_bitmask()
            | second_half
                .simd_ge(zeroes)
                .to_bitmask()
                .unchecked_shl(SECOND_HALF_START as _);
        debug!("Mask:\n{mask:050b}\n{}", {
            let mut line = input.cast::<[u8; DIM]>().read_unaligned();
            line.reverse();
            str::from_utf8(&line).unwrap().to_owned()
        });

        for x in BitIter(mask) {
            let pos = Index {
                x: x as _,
                y: y as _,
            };
            let c: u8 = *ptr_add(input, x);
            debug!("Found char {c:x} ({}) at {pos:?}", c as char);
            let antennae = antennae.get_unchecked_mut(c.unchecked_sub(ZERO) as usize);

            for &antenna in antennae.into_iter() {
                let diff = pos - antenna;
                {
                    let mut index = antenna;
                    while let Some(pos) = index.checked_to() {
                        let antinode = antinodes.get_unchecked_mut(pos);
                        count += !*antinode as i32;
                        *antinode = true;
                        index -= diff;
                    }
                }
                {
                    let mut index = pos;
                    while let Some(pos) = index.checked_to() {
                        let antinode = antinodes.get_unchecked_mut(pos);
                        count += !*antinode as i32;
                        *antinode = true;
                        index += diff;
                    }
                }
            }

            antennae.push_unchecked(pos);
        }

        input = ptr_add(input, const { DIM + 1 } as _);
    }

    // debug!("Final map:\n{}", {
    //     let mut s = String::new();
    //     for (i, &c) in input.iter().enumerate() {
    //         s.push(if antinodes[i] {
    //             if c == b'.' {
    //                 '+'
    //             } else {
    //                 '#'
    //             }
    //         } else {
    //             c as char
    //         })
    //     }
    //     s
    // });

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_p1() {
        let input = include_str!("../input/2024/day8.txt");
        assert_eq!(part1(input), 348);
    }

    #[test]
    fn real_p2() {
        let input = include_str!("../input/2024/day8.txt");
        assert_eq!(part2(input), 1_221);
    }
}
