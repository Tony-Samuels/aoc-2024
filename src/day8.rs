use std::intrinsics::unchecked_sub;

use aoc_runner_derive::aoc;

use crate::{debug, ArrayVec, BigBitSet, Index};

const ZERO: u8 = b'0';
const ANTENNA_OPTS: usize = (b'z' - b'0' + 1) as usize;
const _: () = {
    assert!(b'0' < b'Z');
    assert!(b'Z' < b'z');
};

#[aoc(day8, part1)]
pub fn part1(input: &str) -> i32 {
    unsafe { inner_p1::<50>(input) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_p1<const DIM: usize>(input: &str) -> i32
where
    [(); DIM * (DIM + 1)]:,
    [(); DIM * (DIM + 1) / 8 + 1]:,
{
    let input = input.as_bytes();

    let mut antennae = [ArrayVec::<4, Index<DIM>>::new_unchecked(); ANTENNA_OPTS];
    let mut antinodes = BigBitSet::<{ DIM * (DIM + 1) / 8 + 1 }>::new();
    let mut count = 0;

    for y in 0..DIM as i8 {
        for x in 0..DIM as i8 {
            let index = Index::<DIM> { x, y };
            let &c = input.get_unchecked(index.to());
            if c == b'.' {
                continue;
            }

            debug!("Checking {}, count {count}", c as char);
            let antennae = antennae.get_unchecked_mut(unchecked_sub(c, ZERO) as usize);

            for &antenna in antennae.into_iter() {
                let diff = index - antenna;
                if let Some(pos) = (antenna - diff).checked_to() {
                    let (antinode_index, antinode_mask) = antinodes.calc_byte_mask(pos);
                    let antinode = antinodes.get_byte_unchecked_mut(antinode_index);
                    count += ((*antinode & antinode_mask) == 0) as i32;
                    *antinode |= antinode_mask;
                }
                if let Some(pos) = (index + diff).checked_to() {
                    let (antinode_index, antinode_mask) = antinodes.calc_byte_mask(pos);
                    let antinode = antinodes.get_byte_unchecked_mut(antinode_index);
                    count += ((*antinode & antinode_mask) == 0) as i32;
                    *antinode |= antinode_mask;
                }
            }

            antennae.push_unchecked(index);
        }
    }

    debug!("Final map:\n{}", {
        let mut s = String::new();
        for (i, &c) in input.iter().enumerate() {
            s.push(if antinodes.get_unchecked(i) {
                if c == b'.' {
                    '+'
                } else {
                    '#'
                }
            } else {
                c as char
            })
        }
        s
    });

    count
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> i32 {
    unsafe { inner_p2::<50>(input) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_p2<const DIM: usize>(input: &str) -> i32
where
    [(); DIM * (DIM + 1)]:,
{
    let input = input.as_bytes();

    let mut antennae = [ArrayVec::<4, Index<DIM>>::new_unchecked(); ANTENNA_OPTS];
    let mut antinodes = [false; DIM * (DIM + 1)];
    let mut count = 0;

    for y in 0..DIM as i8 {
        for x in 0..DIM as i8 {
            let index = Index::<DIM> { x, y };
            let &c = input.get_unchecked(index.to());
            if c == b'.' || c == b'\n' {
                continue;
            }

            debug!("Checking {}, count {count}", c as char);
            let antennae = antennae.get_unchecked_mut((c - ZERO) as usize);

            for &antenna in antennae.into_iter() {
                let diff = index - antenna;
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
                    let mut index = index;
                    while let Some(pos) = index.checked_to() {
                        let antinode = antinodes.get_unchecked_mut(pos);
                        count += !*antinode as i32;
                        *antinode = true;
                        index += diff;
                    }
                }
            }

            antennae.push_unchecked(index);
        }
    }

    debug!("Final map:\n{}", {
        let mut s = String::new();
        for (i, &c) in input.iter().enumerate() {
            s.push(if antinodes[i] {
                if c == b'.' {
                    '+'
                } else {
                    '#'
                }
            } else {
                c as char
            })
        }
        s
    });

    count
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    "};

    #[test]
    fn p1_example() {
        assert_eq!(unsafe { inner_p1::<12>(INPUT) }, 14);
    }

    #[test]
    fn p1_simple_examples() {
        let input = indoc! {"
            ..........
            ..........
            ..........
            ....a.....
            ..........
            .....a....
            ..........
            ..........
            ..........
            ..........
        "};
        assert_eq!(unsafe { inner_p1::<10>(input) }, 2);

        let input = indoc! {"
            ..........
            ..........
            ..........
            ....a.....
            ........a.
            .....a....
            ..........
            ..........
            ..........
            ..........
        "};
        assert_eq!(unsafe { inner_p1::<10>(input) }, 4);
    }

    #[test]
    fn p2_example() {
        assert_eq!(unsafe { inner_p2::<12>(INPUT) }, 34);
    }

    #[test]
    fn p2_simple_example() {
        let input = indoc! {"
            T.........
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........
        "};
        assert_eq!(unsafe { inner_p2::<10>(input) }, 0);
        let input = indoc! {"
            T.........
            ...T......
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........
        "};
        assert_eq!(unsafe { inner_p2::<10>(input) }, 4);
        let input = indoc! {"
            T.........
            ..........
            .T........
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........
        "};
        assert_eq!(unsafe { inner_p2::<10>(input) }, 5);
        let input = indoc! {"
            T.........
            ...T......
            .T........
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........
        "};
        assert_eq!(unsafe { inner_p2::<10>(input) }, 9);
    }

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
