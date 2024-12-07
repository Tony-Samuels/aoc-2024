use std::{
    cmp::max,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use aoc_runner_derive::aoc;
use memchr::arch::x86_64::avx2::memchr;

use crate::{assume, debug, Assume};

const GUARD: u8 = b'^';
const BLOCK: u8 = b'#';
const EOL: u8 = b'\n';

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn find_guard(input: &[u8]) -> usize {
    memchr::One::new_unchecked(GUARD).find(input).assume()
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> i32 {
    unsafe { inner_p1::<130>(input) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_p1<const DIM: usize>(input: &str) -> i32
where
    [(); DIM * (DIM + 1)]:,
{
    let len = DIM * (DIM + 1);
    let mut visited = [false; DIM * (DIM + 1)];
    let input = input.as_bytes();
    let mut pos = find_guard(input);
    let mut total = 0;

    loop {
        // Up
        loop {
            let visit = visited.get_unchecked_mut(pos);
            if !*visit {
                total += 1;
                *visit = true;
            }

            if pos < DIM + 1 {
                // debug_map!(input, visited);
                return total;
            }

            let new_pos = pos - DIM - 1;
            if *input.get_unchecked(new_pos) == BLOCK {
                break;
            }

            pos = new_pos;
        }

        // Right
        loop {
            let visit = visited.get_unchecked_mut(pos);
            if !*visit {
                total += 1;
                *visit = true;
            }

            let new_pos = pos + 1;
            if new_pos > len {
                // debug_map!(input, visited);
                return total;
            }

            match *input.get_unchecked(new_pos) {
                BLOCK => break,
                EOL => {
                    // debug_map!(input, visited);
                    return total;
                }
                _ => (),
            }

            pos = new_pos
        }

        // Down
        loop {
            let visit = visited.get_unchecked_mut(pos);
            if !*visit {
                total += 1;
                *visit = true;
            }

            let new_pos = pos + DIM + 1;
            if new_pos > len {
                // debug_map!(input, visited);
                return total;
            }

            if *input.get_unchecked(new_pos) == BLOCK {
                break;
            }

            pos = new_pos;
        }

        // Left
        loop {
            let visit = visited.get_unchecked_mut(pos);
            if !*visit {
                total += 1;
                *visit = true;
            }

            if pos == 0 {
                // debug_map!(input, visited);
                return total;
            }

            let new_pos = pos - 1;
            match *input.get_unchecked(new_pos) {
                BLOCK => break,
                EOL => {
                    // debug_map!(input, visited);
                    return total;
                }
                _ => (),
            }

            pos = new_pos
        }
    }
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> i32 {
    unsafe { inner_p2::<130>(input) }
}

const LEFT_VISIT: u8 = 1 << 0;
const RIGHT_VISIT: u8 = 1 << 1;
const UP_VISIT: u8 = 1 << 2;
const DOWN_VISIT: u8 = 1 << 3;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Index<const DIM: usize> {
    y: i16,
    x: i16,
}

impl<const DIM: usize> Index<DIM> {
    #[inline]
    fn x(x: i16) -> Self {
        Self { x, y: 0 }
    }

    #[inline]
    fn y(y: i16) -> Self {
        Self { x: 0, y }
    }

    #[inline]
    fn to(self) -> usize {
        assume!(
            self.x < DIM as _ && self.y < DIM as _,
            "{self:?} is too large"
        );
        max(self.y, 0) as usize * (DIM + 1) + max(self.x, 0) as usize
    }

    #[inline]
    fn fro(i: usize) -> Self {
        Self {
            y: (i / (DIM + 1)) as _,
            x: (i % (DIM + 1)) as _,
        }
    }
}

impl<const DIM: usize> Add for Index<DIM> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<const DIM: usize> AddAssign for Index<DIM> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<const DIM: usize> Sub for Index<DIM> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<const DIM: usize> SubAssign for Index<DIM> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

#[allow(unused)]
#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn compute_jump_tables<const DIM: usize>(
    input: &[u8],
    jump_right: &mut [Option<Index<DIM>>; DIM * (DIM + 1)],
    jump_up: &mut [Option<Index<DIM>>; DIM * (DIM + 1)],
    jump_left: &mut [Option<Index<DIM>>; DIM * (DIM + 1)],
    jump_down: &mut [Option<Index<DIM>>; DIM * (DIM + 1)],
) {
    for y in 0..DIM {
        let mut before_object = None;
        for x in (1..DIM).rev() {
            let index = Index::<DIM> {
                x: x as _,
                y: y as _,
            };
            if *input.get_unchecked(index.to()) == BLOCK {
                before_object = Some(index - Index::x(1));
                // object = Some(NonZero::new_unchecked(index as u16));
            } else {
                *jump_right.get_unchecked_mut(index.to()) = before_object;
            }
        }
    }

    for y in 0..DIM {
        let mut before_object = None;
        for x in 0..DIM {
            let index = Index::<DIM> {
                x: x as _,
                y: y as _,
            };
            if *input.get_unchecked(index.to()) == BLOCK {
                before_object = Some(index + Index::x(1));
                // object = Some(NonZero::new_unchecked(index as u16));
            } else {
                *jump_left.get_unchecked_mut(index.to()) = before_object;
            }
        }
    }

    for x in 0..DIM {
        let mut before_object = None;
        for y in (1..DIM).rev() {
            let index = Index::<DIM> {
                x: x as _,
                y: y as _,
            };
            if *input.get_unchecked(index.to()) == BLOCK {
                before_object = Some(index - Index::y(1));
                // object = Some(NonZero::new_unchecked(index as u16));
            } else {
                *jump_down.get_unchecked_mut(index.to()) = before_object;
            }
        }
    }

    for x in 0..DIM {
        let mut before_object = None;
        for y in 0..DIM {
            let index = Index::<DIM> {
                x: x as _,
                y: y as _,
            };
            if *input.get_unchecked(index.to()) == BLOCK {
                before_object = Some(index + Index::y(1));
                // object = Some(NonZero::new_unchecked(index as u16));
            } else {
                *jump_up.get_unchecked_mut(index.to()) = before_object;
            }
        }
    }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_p2<const DIM: usize>(input: &str) -> i32
where
    [(); DIM * (DIM + 1)]:,
{
    let input = input.as_bytes();
    let mut count = 0;

    let mut jump_up = [None; DIM * (DIM + 1)];
    let mut jump_down = [None; DIM * (DIM + 1)];
    let mut jump_left = [None; DIM * (DIM + 1)];
    let mut jump_right = [None; DIM * (DIM + 1)];

    compute_jump_tables::<DIM>(
        input,
        &mut jump_right,
        &mut jump_up,
        &mut jump_left,
        &mut jump_down,
    );
    let len = DIM * (DIM + 1);
    let mut visited = [0; DIM * (DIM + 1)];
    let mut pos = find_guard(input);

    loop {
        // Up
        loop {
            let visit = visited.get_unchecked_mut(pos);
            *visit |= UP_VISIT;

            if pos < DIM + 1 {
                // debug_map!(input, visited);
                return count;
            }

            let new_pos = pos - DIM - 1;
            if *input.get_unchecked(new_pos) == BLOCK {
                break;
            }
            if *visited.get_unchecked(new_pos) == 0 {
                count += loops::<DIM>(
                    Index::fro(pos),
                    RIGHT_VISIT,
                    Index::fro(new_pos),
                    &jump_right,
                    &jump_up,
                    &jump_left,
                    &jump_down,
                ) as i32;
            }

            pos = new_pos;
        }

        // Right
        loop {
            let visit = visited.get_unchecked_mut(pos);
            *visit |= RIGHT_VISIT;

            let new_pos = pos + 1;
            if new_pos > len {
                // debug_map!(input, visited);
                return count;
            }

            match *input.get_unchecked(new_pos) {
                BLOCK => break,
                EOL => {
                    // debug_map!(input, visited);
                    return count;
                }
                _ => (),
            }

            if *visited.get_unchecked(new_pos) == 0 {
                count += loops::<DIM>(
                    Index::fro(pos),
                    DOWN_VISIT,
                    Index::fro(new_pos),
                    &jump_right,
                    &jump_up,
                    &jump_left,
                    &jump_down,
                ) as i32;
            }

            pos = new_pos
        }

        // Down
        loop {
            let visit = visited.get_unchecked_mut(pos);
            *visit |= DOWN_VISIT;

            let new_pos = pos + DIM + 1;
            if new_pos > len {
                // debug_map!(input, visited);
                return count;
            }

            if *input.get_unchecked(new_pos) == BLOCK {
                break;
            }

            if *visited.get_unchecked(new_pos) == 0 {
                count += loops::<DIM>(
                    Index::fro(pos),
                    LEFT_VISIT,
                    Index::fro(new_pos),
                    &jump_right,
                    &jump_up,
                    &jump_left,
                    &jump_down,
                ) as i32;
            }

            pos = new_pos;
        }

        // Left
        loop {
            let visit = visited.get_unchecked_mut(pos);
            *visit |= LEFT_VISIT;

            if pos == 0 {
                // debug_map!(input, visited);
                return count;
            }

            let new_pos = pos - 1;
            match *input.get_unchecked(new_pos) {
                BLOCK => break,
                EOL => {
                    // debug_map!(input, visited);
                    return count;
                }
                _ => (),
            }

            if *visited.get_unchecked(new_pos) == 0 {
                count += loops::<DIM>(
                    Index::fro(pos),
                    UP_VISIT,
                    Index::fro(new_pos),
                    &jump_right,
                    &jump_up,
                    &jump_left,
                    &jump_down,
                ) as i32;
            }

            pos = new_pos
        }
    }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn loops<const DIM: usize>(
    mut pos: Index<DIM>,
    start_dir: u8,
    obstruction: Index<DIM>,
    jump_right: &[Option<Index<DIM>>; DIM * (DIM + 1)],
    jump_up: &[Option<Index<DIM>>; DIM * (DIM + 1)],
    jump_left: &[Option<Index<DIM>>; DIM * (DIM + 1)],
    jump_down: &[Option<Index<DIM>>; DIM * (DIM + 1)],
) -> bool
where
    [(); DIM * (DIM + 1)]:,
{
    let mut visited = [0u8; DIM * (DIM + 1)];
    assume!(pos != obstruction);
    debug!("Hunting for loop, starting at {pos:?}, obstruction at {obstruction:?}");

    if start_dir == RIGHT_VISIT {
        let visit = visited.get_unchecked_mut(pos.to());
        if *visit & RIGHT_VISIT != 0 {
            println!("Obstruction at {obstruction:?} {}", obstruction.to());
            return true;
        }
        *visit |= RIGHT_VISIT;

        let before_obstruction = obstruction - Index::x(1);
        let maybe_obstructed = before_obstruction.x > 0
            && before_obstruction.y == pos.y
            && before_obstruction.x > pos.x;

        let new_pos = match jump_right.get_unchecked(pos.to()) {
            &Some(jump_pos) => {
                if maybe_obstructed && (pos.x..jump_pos.x).contains(&before_obstruction.x) {
                    before_obstruction
                } else {
                    jump_pos
                }
            }
            None if maybe_obstructed => before_obstruction,
            None => return false,
        };
        debug!("Jumping from {pos:?} to {new_pos:?}, dir: RIGHT");
        pos = new_pos;
    }

    if start_dir & (RIGHT_VISIT | DOWN_VISIT) != 0 {
        let visit = visited.get_unchecked_mut(pos.to());
        if *visit & DOWN_VISIT != 0 {
            println!("Obstruction at {obstruction:?} {}", obstruction.to());
            return true;
        }
        *visit |= DOWN_VISIT;

        let before_obstruction = obstruction - Index::y(1);
        let maybe_obstructed = before_obstruction.y > 0 as _
            && before_obstruction.x == pos.x
            && before_obstruction.y > pos.y;

        let new_pos = match jump_down.get_unchecked(pos.to()) {
            &Some(jump_pos) => {
                if maybe_obstructed && (pos.y..jump_pos.y).contains(&before_obstruction.y) {
                    before_obstruction
                } else {
                    jump_pos
                }
            }
            None if maybe_obstructed => before_obstruction,
            None => return false,
        };
        debug!("Jumping from {pos:?} to {new_pos:?}, dir: DOWN");
        pos = new_pos;
    }

    if start_dir != UP_VISIT {
        let visit = visited.get_unchecked_mut(pos.to());
        if *visit & LEFT_VISIT != 0 {
            println!("Obstruction at {obstruction:?} {}", obstruction.to());
            return true;
        }
        *visit |= LEFT_VISIT;

        let before_obstruction = obstruction + Index::x(1);
        let maybe_obstructed = before_obstruction.x < DIM as _
            && before_obstruction.y == pos.y
            && before_obstruction.x < pos.x;

        let new_pos = match jump_left.get_unchecked(pos.to()) {
            &Some(jump_pos) => {
                if maybe_obstructed && (jump_pos.x..pos.x).contains(&before_obstruction.x) {
                    before_obstruction
                } else {
                    jump_pos
                }
            }
            None if maybe_obstructed => before_obstruction,
            None => return false,
        };
        debug!("Jumping from {pos:?} to {new_pos:?}, dir: LEFT");
        pos = new_pos;
    }

    loop {
        // Up
        {
            let visit = visited.get_unchecked_mut(pos.to());
            if *visit & UP_VISIT != 0 {
                println!("Obstruction at {obstruction:?} {}", obstruction.to());
                return true;
            }
            *visit |= UP_VISIT;

            let before_obstruction = obstruction + Index::y(1);
            let maybe_obstructed = before_obstruction.y > DIM as _
                && before_obstruction.x == pos.x
                && before_obstruction.y < pos.y;

            let new_pos = match jump_up.get_unchecked(pos.to()) {
                &Some(jump_pos) => {
                    if maybe_obstructed && (jump_pos.y..pos.y).contains(&before_obstruction.y) {
                        before_obstruction
                    } else {
                        jump_pos
                    }
                }
                None if maybe_obstructed => before_obstruction,
                None => return false,
            };
            debug!("Jumping from {pos:?} to {new_pos:?}, dir: UP");
            pos = new_pos;
        }

        // Right
        {
            let visit = visited.get_unchecked_mut(pos.to());
            if *visit & RIGHT_VISIT != 0 {
                println!("Obstruction at {obstruction:?} {}", obstruction.to());
                return true;
            }
            *visit |= RIGHT_VISIT;

            let before_obstruction = obstruction - Index::x(1);
            let maybe_obstructed = before_obstruction.x > 0
                && before_obstruction.y == pos.y
                && before_obstruction.x > pos.x;

            let new_pos = match jump_right.get_unchecked(pos.to()) {
                &Some(jump_pos) => {
                    if maybe_obstructed && (pos.x..jump_pos.x).contains(&before_obstruction.x) {
                        before_obstruction
                    } else {
                        jump_pos
                    }
                }
                None if maybe_obstructed => before_obstruction,
                None => return false,
            };
            debug!("Jumping from {pos:?} to {new_pos:?}, dir: RIGHT");
            pos = new_pos;
        }

        // Down
        {
            let visit = visited.get_unchecked_mut(pos.to());
            if *visit & DOWN_VISIT != 0 {
                println!("Obstruction at {obstruction:?} {}", obstruction.to());
                return true;
            }
            *visit |= DOWN_VISIT;

            let before_obstruction = obstruction - Index::y(1);
            let maybe_obstructed = before_obstruction.y > 0
                && before_obstruction.x == pos.x
                && before_obstruction.y > pos.y;

            let new_pos = match jump_down.get_unchecked(pos.to()) {
                &Some(jump_pos) => {
                    if maybe_obstructed && (pos.y..jump_pos.y).contains(&before_obstruction.y) {
                        before_obstruction
                    } else {
                        jump_pos
                    }
                }
                None if maybe_obstructed => before_obstruction,
                None => return false,
            };
            debug!("Jumping from {pos:?} to {new_pos:?}, dir: DOWN");
            pos = new_pos;
        }

        // Left
        {
            let visit = visited.get_unchecked_mut(pos.to());
            if *visit & LEFT_VISIT != 0 {
                println!("Obstruction at {obstruction:?} {}", obstruction.to());
                return true;
            }
            *visit |= LEFT_VISIT;

            let before_obstruction = obstruction + Index::x(1);
            let maybe_obstructed = before_obstruction.x < DIM as _
                && before_obstruction.y == pos.y
                && before_obstruction.x < pos.x;

            let new_pos = match jump_left.get_unchecked(pos.to()) {
                &Some(jump_pos) => {
                    if maybe_obstructed && (jump_pos.x..pos.x).contains(&before_obstruction.x) {
                        before_obstruction
                    } else {
                        jump_pos
                    }
                }
                None if maybe_obstructed => before_obstruction,
                None => return false,
            };
            debug!("Jumping from {pos:?} to {new_pos:?}, dir: LEFT");
            pos = new_pos;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use indoc::indoc;
    const INPUT: &str = indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "};

    #[test]
    #[ignore = "can only run real or fake at once"]
    fn p1_example() {
        assert_eq!(unsafe { inner_p1::<10>(INPUT) }, 41);
    }

    #[test]
    #[ignore = "can only run real or fake at once"]
    fn p2_example() {
        assert_eq!(unsafe { inner_p2::<10>(INPUT) }, 6);
    }

    const REAL: &str = include_str!("../input/2024/day6.txt");

    #[test]
    // #[ignore = "can only run real or fake at once"]
    fn real_p1() {
        assert_eq!(part1(REAL), 4_665);
    }

    #[test]
    // #[ignore = "can only run real or fake at once"]
    fn real_p2() {
        assert_eq!(part2(REAL), 1_688);
    }

    // #[test]
    // fn loops_p2_real() {
    //     assert!(!unsafe {
    //         loops::<130>(
    //             REAL.as_bytes(),
    //             find_guard(REAL.as_bytes()),
    //             UP_VISIT,
    //             usize::MAX,
    //         )
    //     });
    // }

    // #[test]
    // fn loops_p2_example() {
    //     assert!(!unsafe {
    //         loops::<10>(
    //             INPUT.as_bytes(),
    //             find_guard(INPUT.as_bytes()),
    //             UP_VISIT,
    //             usize::MAX,
    //         )
    //     });
    // }

    // #[test]
    // fn loops_loop() {
    //     let input = indoc! {"
    //         ###
    //         #^#
    //         ###
    //     "};
    //     assert!(unsafe {
    //         loops::<3>(
    //             input.as_bytes(),
    //             find_guard(input.as_bytes()),
    //             UP_VISIT,
    //             usize::MAX,
    //         )
    //     });
    // }

    // #[test]
    // #[ignore = "Static muts don't do well with different data"]
    // fn jumps() {
    //     let input = indoc! {"
    //         ...
    //         .#.
    //         ...
    //     "};

    //     unsafe {
    //         compute_jump_tables::<3>(input.as_bytes());
    //     }

    //     unsafe {
    //         assert_eq!(
    //             &JUMP_DOWN[..3 * 4],
    //             [
    //                 None,
    //                 Some(NonZero::new_unchecked(1)),
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //             ]
    //         );
    //         assert_eq!(
    //             &JUMP_UP[..3 * 4],
    //             [
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 Some(NonZero::new_unchecked(9)),
    //                 None,
    //                 None,
    //             ]
    //         );
    //         assert_eq!(
    //             &JUMP_RIGHT[..3 * 4],
    //             [
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 Some(NonZero::new_unchecked(4)),
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //             ]
    //         );
    //         assert_eq!(
    //             &JUMP_LEFT[..3 * 4],
    //             [
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 Some(NonZero::new_unchecked(6)),
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //             ]
    //         );
    //     }
    // }
}
