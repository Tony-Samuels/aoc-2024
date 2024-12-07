use std::{
    cmp::max,
    mem::replace,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use aoc_runner_derive::aoc;
use memchr::arch::x86_64::avx2::memchr;

use crate::{assume, debug, Assume, Unreachable};

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

    macro_rules! debug_map {
        ($input:ident, $visited:ident) => {
            debug!("Current map:\n{}", {
                let mut s = String::with_capacity($input.len());
                for (i, c) in $input.iter().enumerate() {
                    s.push(match c {
                        b'.' if $visited[i] => b'O',
                        c => *c,
                    } as char);
                }
                s
            });
        };
    }

    loop {
        // Up
        loop {
            let visit = visited.get_unchecked_mut(pos);
            if !*visit {
                total += 1;
                *visit = true;
            }

            if pos < DIM + 1 {
                debug_map!(input, visited);
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
                debug_map!(input, visited);
                return total;
            }

            match *input.get_unchecked(new_pos) {
                BLOCK => break,
                EOL => {
                    debug_map!(input, visited);
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
                debug_map!(input, visited);
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
                debug_map!(input, visited);
                return total;
            }

            let new_pos = pos - 1;
            match *input.get_unchecked(new_pos) {
                BLOCK => break,
                EOL => {
                    debug_map!(input, visited);
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
    fn with_x(self, x: i16) -> Self {
        Self { x, ..self }
    }

    #[inline]
    fn with_y(self, y: i16) -> Self {
        Self { y, ..self }
    }

    #[inline]
    fn to(self) -> usize {
        assume!(
            self.x < DIM as i16 && self.y < DIM as i16,
            "{self:?} is too large"
        );
        max(self.y, 0) as usize * (DIM + 1) + max(self.x, 0) as usize
    }

    #[inline]
    fn fro(i: usize) -> Self {
        Self {
            y: (i / (DIM + 1)) as i16,
            x: (i % (DIM + 1)) as i16,
        }
    }

    #[inline]
    fn step_size(self, other: Self) -> Self {
        assume!(other.x == self.x || other.y == self.y);
        Self {
            y: (other.y - self.y).signum(),
            x: (other.x - self.x).signum(),
        }
    }

    #[inline]
    fn rot90(self) -> Self {
        Self {
            y: self.x,
            x: -self.y,
        }
    }

    #[inline]
    fn as_visit(self) -> u8 {
        match self {
            Self { x: 1, y: 0 } => RIGHT_VISIT,
            Self { x: 0, y: 1 } => DOWN_VISIT,
            Self { x: -1, y: 0 } => LEFT_VISIT,
            Self { x: 0, y: -1 } => UP_VISIT,
            _ => Unreachable.assume(),
        }
    }

    #[inline]
    fn dir(self, other: Self) -> Option<Self> {
        if self.x == other.x && self.y != other.y {
            return Some(Self {
                y: (other.y - self.y).signum(),
                x: self.x,
            });
        } else if self.y == other.y && self.x != other.x {
            Some(Self {
                x: (other.x - self.x).signum(),
                y: self.y,
            })
        } else {
            None
        }
    }

    #[inline]
    fn manhattan(self, other: Self) -> u16 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
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
                x: x as i16,
                y: y as i16,
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
                x: x as i16,
                y: y as i16,
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
                x: x as i16,
                y: y as i16,
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
                x: x as i16,
                y: y as i16,
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

struct Stepper<const DIM: usize> {
    curr: Index<DIM>,
    end: Index<DIM>,
    step: Index<DIM>,
}

impl<const DIM: usize> Stepper<DIM> {
    #[inline]
    fn new(start: Index<DIM>, end: Index<DIM>) -> Self {
        assume!(start.x == end.x || start.y == end.y);
        Self {
            curr: start,
            end,
            step: start.step_size(end),
        }
    }

    fn touches(&self, target: Index<DIM>) -> bool {
        if self.curr.x == target.x {
            self.curr.y.cmp(&target.y) != self.end.y.cmp(&target.y)
        } else if self.curr.y == target.y {
            self.curr.x.cmp(&target.x) != self.end.x.cmp(&target.x)
        } else {
            false
        }
    }

    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    #[inline]
    unsafe fn peek(&self) -> Index<DIM> {
        self.curr
    }

    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    #[inline]
    unsafe fn step(&mut self) -> Option<Index<DIM>> {
        if self.curr == self.end {
            None
        } else {
            let new_val = self.curr + self.step;
            Some(replace(&mut self.curr, new_val))
        }
    }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_p2<const DIM: usize>(input: &str) -> i32
where
    [(); DIM * (DIM + 1)]:,
{
    let input = input.as_bytes();
    let visited = find_guard_rota::<DIM>(input);
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

    for y in 0..DIM as i16 {
        for x in 0..DIM as i16 {
            let pos = Index { y, x };

            let visits = visited.get_unchecked(pos.to());
            if visits & UP_VISIT != 0 && pos.y != 0 {
                count += loops::<DIM>(
                    pos,
                    Index::y(-1),
                    pos - Index::y(1),
                    &jump_right,
                    &jump_up,
                    &jump_left,
                    &jump_down,
                ) as i32;
            }
            if visits & DOWN_VISIT != 0 && pos.y < DIM as i16 - 1 {
                count += loops::<DIM>(
                    pos,
                    Index::y(1),
                    pos + Index::y(1),
                    &jump_right,
                    &jump_up,
                    &jump_left,
                    &jump_down,
                ) as i32;
            }
            if visits & RIGHT_VISIT != 0 && pos.x < DIM as i16 - 1 {
                count += loops::<DIM>(
                    pos,
                    Index::x(1),
                    pos + Index::x(1),
                    &jump_right,
                    &jump_up,
                    &jump_left,
                    &jump_down,
                ) as i32;
            }
            if visits & LEFT_VISIT != 0 && pos.x != 0 {
                count += loops::<DIM>(
                    pos,
                    Index::x(-1),
                    pos - Index::x(1),
                    &jump_right,
                    &jump_up,
                    &jump_left,
                    &jump_down,
                ) as i32;
            }
        }
    }

    count
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn find_guard_rota<const DIM: usize>(input: &[u8]) -> [u8; DIM * (DIM + 1)]
where
    [(); DIM * (DIM + 1)]:,
{
    let len = DIM * (DIM + 1);
    let mut visited = [0; DIM * (DIM + 1)];
    let mut pos = find_guard(input);

    macro_rules! debug_map {
        ($input:ident, $visited:ident) => {
            debug!("Current map:\n{}", {
                let mut s = String::with_capacity($input.len());
                for (i, c) in $input.iter().enumerate() {
                    s.push(match c {
                        b'.' => match $visited[i] {
                            0 => b'.',
                            LEFT_VISIT | RIGHT_VISIT | const { LEFT_VISIT | RIGHT_VISIT } => b'-',
                            UP_VISIT | DOWN_VISIT | const { UP_VISIT | DOWN_VISIT } => b'|',
                            _ => b'+',
                        },
                        c => *c,
                    } as char);
                }
                s
            });
        };
    }

    loop {
        // Up
        loop {
            let visit = visited.get_unchecked_mut(pos);
            *visit |= UP_VISIT;

            if pos < DIM + 1 {
                debug_map!(input, visited);
                return visited;
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
            *visit |= RIGHT_VISIT;

            let new_pos = pos + 1;
            if new_pos > len {
                debug_map!(input, visited);
                return visited;
            }

            match *input.get_unchecked(new_pos) {
                BLOCK => break,
                EOL => {
                    debug_map!(input, visited);
                    return visited;
                }
                _ => (),
            }

            pos = new_pos
        }

        // Down
        loop {
            let visit = visited.get_unchecked_mut(pos);
            *visit |= DOWN_VISIT;

            let new_pos = pos + DIM + 1;
            if new_pos > len {
                debug_map!(input, visited);
                return visited;
            }

            if *input.get_unchecked(new_pos) == BLOCK {
                break;
            }

            pos = new_pos;
        }

        // Left
        loop {
            let visit = visited.get_unchecked_mut(pos);
            *visit |= LEFT_VISIT;

            if pos == 0 {
                debug_map!(input, visited);
                return visited;
            }

            let new_pos = pos - 1;
            match *input.get_unchecked(new_pos) {
                BLOCK => break,
                EOL => {
                    debug_map!(input, visited);
                    return visited;
                }
                _ => (),
            }

            pos = new_pos
        }
    }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn loops<const DIM: usize>(
    mut pos: Index<DIM>,
    mut dir: Index<DIM>,
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

    loop {
        // Up
        {
            let visit = visited.get_unchecked_mut(pos.to());
            if *visit & UP_VISIT != 0 {
                return true;
            }
            *visit |= UP_VISIT;

            let maybe_obstructed = obstruction.x == pos.x && obstruction.y < pos.y;

            pos = match jump_up.get_unchecked(pos.to()) {
                &Some(jump_pos) => {
                    if maybe_obstructed && obstruction.y > jump_pos.y {
                        obstruction
                    } else {
                        jump_pos
                    }
                }
                None if maybe_obstructed => obstruction,
                None => return false,
            };
        }

        // Right
        {
            let visit = visited.get_unchecked_mut(pos.to());
            if *visit & RIGHT_VISIT != 0 {
                return true;
            }
            *visit |= RIGHT_VISIT;

            let maybe_obstructed = obstruction.y == pos.y && obstruction.x > pos.x;

            pos = match jump_right.get_unchecked(pos.to()) {
                &Some(jump_pos) => {
                    if maybe_obstructed && obstruction.x < jump_pos.x {
                        obstruction
                    } else {
                        jump_pos
                    }
                }
                None if maybe_obstructed => obstruction,
                None => return false,
            };
        }

        // Down
        {
            let visit = visited.get_unchecked_mut(pos.to());
            if *visit & DOWN_VISIT != 0 {
                return true;
            }
            *visit |= DOWN_VISIT;

            let maybe_obstructed = obstruction.x == pos.x && obstruction.y > pos.y;

            pos = match jump_down.get_unchecked(pos.to()) {
                &Some(jump_pos) => {
                    if maybe_obstructed && obstruction.y < jump_pos.y {
                        obstruction
                    } else {
                        jump_pos
                    }
                }
                None if maybe_obstructed => obstruction,
                None => return false,
            };
        }

        // Left
        {
            let visit = visited.get_unchecked_mut(pos.to());
            if *visit & LEFT_VISIT != 0 {
                return true;
            }
            *visit |= LEFT_VISIT;

            let maybe_obstructed = obstruction.y == pos.y && obstruction.x < pos.x;

            pos = match jump_left.get_unchecked(pos.to()) {
                &Some(jump_pos) => {
                    if maybe_obstructed && obstruction.x > jump_pos.x {
                        obstruction
                    } else {
                        jump_pos
                    }
                }
                None if maybe_obstructed => obstruction,
                None => return false,
            };
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // use indoc::indoc;
    // const INPUT: &str = indoc! {"
    //     ....#.....
    //     .........#
    //     ..........
    //     ..#.......
    //     .......#..
    //     ..........
    //     .#..^.....
    //     ........#.
    //     #.........
    //     ......#...
    // "};

    // #[test]
    // fn p1_example() {
    //     assert_eq!(unsafe { inner_p1::<10>(INPUT) }, 41);
    // }

    // #[test]
    // fn p2_example() {
    //     assert_eq!(unsafe { inner_p2::<10>(INPUT) }, 6);
    // }

    const REAL: &str = include_str!("../input/2024/day6.txt");

    #[test]
    fn real_p1() {
        assert_eq!(part1(REAL), 4_665);
    }

    #[test]
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
