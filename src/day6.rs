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

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_p2<const DIM: usize>(input: &str) -> i32
where
    [(); DIM * (DIM + 1)]:,
{
    let input = input.as_bytes();
    let len = DIM * (DIM + 1);
    let mut visited = [0u8; DIM * (DIM + 1)];
    let mut pos = find_guard(input);
    let mut count = 0;

    macro_rules! debug_map {
        ($input:ident, $visited:ident, $obstruction:ident) => {
            debug!("Current map:\n{}", {
                let mut s = String::with_capacity($input.len());
                for (i, c) in $input.iter().enumerate() {
                    s.push(match c {
                        _ if i == $obstruction => b'O',
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
                return count;
            }

            let new_pos = pos - DIM - 1;
            if *input.get_unchecked(new_pos) == BLOCK {
                break;
            }

            if *visited.get_unchecked(new_pos) == 0 {
                let loops = loops::<DIM>(input, pos, UP_VISIT, new_pos);
                if loops {
                    debug!("Looping at {}, {}", new_pos / (DIM + 1), new_pos % DIM);
                    debug_map!(input, visited, new_pos);
                }
                count += loops as i32;
            }
            pos = new_pos;
        }

        // Right
        loop {
            let visit = visited.get_unchecked_mut(pos);
            *visit |= RIGHT_VISIT;

            let new_pos = pos + 1;
            if new_pos > len {
                return count;
            }

            match *input.get_unchecked(new_pos) {
                BLOCK => break,
                EOL => {
                    return count;
                }
                _ => (),
            }

            if *visited.get_unchecked(new_pos) == 0 {
                let loops = loops::<DIM>(input, pos, RIGHT_VISIT, new_pos);
                if loops {
                    debug!("Looping at {}, {}", new_pos / (DIM + 1), new_pos % DIM);
                    debug_map!(input, visited, new_pos);
                }
                count += loops as i32;
            }
            pos = new_pos;
        }

        // Down
        loop {
            let visit = visited.get_unchecked_mut(pos);
            *visit |= DOWN_VISIT;

            let new_pos = pos + DIM + 1;
            if new_pos > len {
                return count;
            }

            if *input.get_unchecked(new_pos) == BLOCK {
                break;
            }

            if *visited.get_unchecked(new_pos) == 0 {
                let loops = loops::<DIM>(input, pos, DOWN_VISIT, new_pos);
                if loops {
                    debug!("Looping at {}, {}", new_pos / (DIM + 1), new_pos % DIM);
                    debug_map!(input, visited, new_pos);
                }
                count += loops as i32;
            }
            pos = new_pos;
        }

        // Left
        loop {
            let visit = visited.get_unchecked_mut(pos);
            *visit |= LEFT_VISIT;

            if pos == 0 {
                return count;
            }

            let new_pos = pos - 1;
            match *input.get_unchecked(new_pos) {
                BLOCK => break,
                EOL => {
                    return count;
                }
                _ => (),
            }

            if *visited.get_unchecked(new_pos) == 0 {
                let loops = loops::<DIM>(input, pos, LEFT_VISIT, new_pos);
                if loops {
                    debug!("Looping at {}, {}", new_pos / (DIM + 1), new_pos % DIM);
                    debug_map!(input, visited, new_pos);
                }
                count += loops as i32;
            }
            pos = new_pos;
        }
    }
}

const LEFT_VISIT: u8 = 1 << 0;
const RIGHT_VISIT: u8 = 1 << 1;
const UP_VISIT: u8 = 1 << 2;
const DOWN_VISIT: u8 = 1 << 3;

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn loops<const DIM: usize>(
    input: &[u8],
    mut pos: usize,
    start_dir: u8,
    obstruction: usize,
) -> bool
where
    [(); DIM * (DIM + 1)]:,
{
    let len = DIM * (DIM + 1);
    let mut visited = [0u8; DIM * (DIM + 1)];
    assume!(pos != obstruction);

    if start_dir == RIGHT_VISIT {
        // Right
        loop {
            let visit = visited.get_unchecked_mut(pos);
            if *visit & RIGHT_VISIT != 0 {
                return true;
            }
            *visit |= RIGHT_VISIT;

            let new_pos = pos + 1;
            if new_pos > len {
                return false;
            }

            match *input.get_unchecked(new_pos) {
                BLOCK => break,
                EOL => {
                    return false;
                }
                _ => (),
            }
            if new_pos == obstruction {
                break;
            }

            pos = new_pos
        }
    }

    if start_dir & (RIGHT_VISIT | DOWN_VISIT) != 0 {
        // Down
        loop {
            let visit = visited.get_unchecked_mut(pos);
            if *visit & DOWN_VISIT != 0 {
                return true;
            }
            *visit |= DOWN_VISIT;

            let new_pos = pos + DIM + 1;
            if new_pos > len {
                return false;
            }

            if new_pos == obstruction || *input.get_unchecked(new_pos) == BLOCK {
                break;
            }

            pos = new_pos;
        }
    }

    if start_dir != UP_VISIT {
        // Left
        loop {
            let visit = visited.get_unchecked_mut(pos);
            if *visit & LEFT_VISIT != 0 {
                return true;
            }
            *visit |= LEFT_VISIT;

            if pos == 0 {
                return false;
            }

            let new_pos = pos - 1;
            match *input.get_unchecked(new_pos) {
                BLOCK => break,
                EOL => {
                    return false;
                }
                _ => (),
            }
            if new_pos == obstruction {
                break;
            }

            pos = new_pos
        }
    }

    loop {
        // Up
        loop {
            let visit = visited.get_unchecked_mut(pos);
            if *visit & UP_VISIT != 0 {
                return true;
            }
            *visit |= UP_VISIT;

            if pos < DIM + 1 {
                return false;
            }

            let new_pos = pos - DIM - 1;
            if new_pos == obstruction || *input.get_unchecked(new_pos) == BLOCK {
                break;
            }

            pos = new_pos;
        }

        // Right
        loop {
            let visit = visited.get_unchecked_mut(pos);
            if *visit & RIGHT_VISIT != 0 {
                return true;
            }
            *visit |= RIGHT_VISIT;

            let new_pos = pos + 1;
            if new_pos > len {
                return false;
            }

            match *input.get_unchecked(new_pos) {
                BLOCK => break,
                EOL => {
                    return false;
                }
                _ => (),
            }
            if new_pos == obstruction {
                break;
            }

            pos = new_pos
        }

        // Down
        loop {
            let visit = visited.get_unchecked_mut(pos);
            if *visit & DOWN_VISIT != 0 {
                return true;
            }
            *visit |= DOWN_VISIT;

            let new_pos = pos + DIM + 1;
            if new_pos > len {
                return false;
            }

            if new_pos == obstruction || *input.get_unchecked(new_pos) == BLOCK {
                break;
            }

            pos = new_pos;
        }

        // Left
        loop {
            let visit = visited.get_unchecked_mut(pos);
            if *visit & LEFT_VISIT != 0 {
                return true;
            }
            *visit |= LEFT_VISIT;

            if pos == 0 {
                return false;
            }

            let new_pos = pos - 1;
            match *input.get_unchecked(new_pos) {
                BLOCK => break,
                EOL => {
                    return false;
                }
                _ => (),
            }
            if new_pos == obstruction {
                break;
            }

            pos = new_pos
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

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
    fn p1_example() {
        assert_eq!(unsafe { inner_p1::<10>(INPUT) }, 41);
    }

    #[test]
    fn p2_example() {
        assert_eq!(unsafe { inner_p2::<10>(INPUT) }, 6);
    }

    const REAL: &str = include_str!("../input/2024/day6.txt");

    #[test]
    fn real_p1() {
        assert_eq!(part1(REAL), 4665);
    }

    #[test]
    fn real_p2() {
        assert_eq!(part2(REAL), 1_688);
    }

    #[test]
    fn loops_p2_real() {
        assert!(!unsafe {
            loops::<130>(
                REAL.as_bytes(),
                find_guard(REAL.as_bytes()),
                UP_VISIT,
                usize::MAX,
            )
        });
    }

    #[test]
    fn loops_p2_example() {
        assert!(!unsafe {
            loops::<10>(
                INPUT.as_bytes(),
                find_guard(INPUT.as_bytes()),
                UP_VISIT,
                usize::MAX,
            )
        });
    }

    #[test]
    fn loops_loop() {
        let input = indoc! {"
            ###
            #^#
            ###
        "};
        assert!(unsafe {
            loops::<3>(
                input.as_bytes(),
                find_guard(input.as_bytes()),
                UP_VISIT,
                usize::MAX,
            )
        });
    }
}
