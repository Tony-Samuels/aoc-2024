use std::{cmp::Ordering, mem::transmute};

use aoc_runner_derive::aoc;

use crate::{ptr_add, Assume, Unreachable};

static LUT: [i8; 256 * 256 * 256] = unsafe { transmute(*include_bytes!("LUT14.bin")) };

const LUT_LOOKUP_MASK: u32 = 0x00FFFFFF;

#[inline]
fn calc_q<const WIDTH: i32, const HEIGHT: i32>(final_x: i32, final_y: i32) -> usize {
    let final_x = final_x.cmp(&(WIDTH / 2));
    let final_y = final_y.cmp(&(HEIGHT / 2));

    match (final_x, final_y) {
        (Ordering::Less, Ordering::Less) => 0,
        (Ordering::Greater, Ordering::Less) => 1,
        (Ordering::Less, Ordering::Greater) => 2,
        (Ordering::Greater, Ordering::Greater) => 3,
        (Ordering::Equal, _) | (_, Ordering::Equal) => 4,
    }
}

#[derive(Copy, Clone)]
struct Robot {
    x: i8,
    y: i8,
    dx: i8,
    dy: i8,
}

#[inline]
unsafe fn parse_num(input: &[u8], pos: usize) -> i8 {
    let index = ptr_add(input.as_ptr(), pos)
        .cast::<u32>()
        .read_unaligned()
        .swap_bytes()
        & LUT_LOOKUP_MASK;
    *LUT.get_unchecked(index as usize)
}

impl Robot {
    #[inline]
    unsafe fn x_at<const WIDTH: i32>(self, timestep: i32) -> i32 {
        timestep
            .unchecked_mul(self.dx as i32)
            .unchecked_add(self.x as i32)
            .checked_rem_euclid(WIDTH)
            .unwrap_or_else(|| Unreachable.assume())
    }
    #[inline]
    unsafe fn y_at<const HEIGHT: i32>(self, timestep: i32) -> i32 {
        timestep
            .unchecked_mul(self.dy as i32)
            .unchecked_add(self.y as i32)
            .checked_rem_euclid(HEIGHT)
            .unwrap_or_else(|| Unreachable.assume())
    }
}

#[inline]
const fn len(num: i8) -> usize {
    if num >= 100 {
        3
    } else if num >= 10 {
        2
    } else if num >= 0 {
        1
    } else if num >= -9 {
        2
    } else {
        3
    }
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> i32 {
    unsafe { inner_p1::<500, 101, 103>(input) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_p1<const INPUT_LINES: usize, const WIDTH: i32, const HEIGHT: i32>(
    input: &str,
) -> i32 {
    let input = input.as_bytes();
    let mut quadrants = [0; 5];
    let mut pos = 1;

    for _ in 0..INPUT_LINES - 1 {
        let x = parse_num(input, pos);
        pos = pos.unchecked_add(len(x)).unchecked_add(1);

        let y = parse_num(input, pos);
        pos = pos.unchecked_add(len(y)).unchecked_add(3);

        let dx = parse_num(input, pos);
        pos = pos.unchecked_add(len(dx)).unchecked_add(1);

        let dy = parse_num(input, pos);
        pos = pos.unchecked_add(len(dy)).unchecked_add(3);

        let robot = Robot { x, y, dx, dy };
        let final_x = robot.x_at::<WIDTH>(100);
        let final_y = robot.y_at::<HEIGHT>(100);

        let quadrant = calc_q::<WIDTH, HEIGHT>(final_x, final_y);
        *quadrants.get_unchecked_mut(quadrant) += 1;
    }

    crate::debug!(
        "Remaining text: {}",
        std::str::from_utf8(&input[pos..input.len() - 1]).unwrap()
    );

    // Last line may not have 3 bytes for final number
    {
        let x = parse_num(input, pos);
        pos = pos.unchecked_add(len(x)).unchecked_add(1);

        let y = parse_num(input, pos);
        pos = pos.unchecked_add(len(y)).unchecked_add(3);

        let dx = parse_num(input, pos);
        pos = pos.unchecked_add(len(dx)).unchecked_add(1);

        let index = u32::from_ne_bytes([
            *input.get(pos.unchecked_add(3)).unwrap_or(&0),
            *input.get(pos.unchecked_add(2)).unwrap_or(&0),
            *input.get_unchecked(pos.unchecked_add(1)),
            0,
        ]);
        let dy = *LUT.get_unchecked(index as usize);

        let robot = Robot { x, y, dx, dy };
        let final_x = robot.x_at::<WIDTH>(100);
        let final_y = robot.y_at::<HEIGHT>(100);
        let quadrant = calc_q::<WIDTH, HEIGHT>(final_x, final_y);
        *quadrants.get_unchecked_mut(quadrant) += 1;
    }

    crate::debug!("Quadrants: {quadrants:?}");
    quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
}

static mut ROBOT_X: [i8; 500] = [0; 500];
static mut ROBOT_DX: [i8; 500] = [0; 500];
static mut ROBOT_Y: [i8; 500] = [0; 500];
static mut ROBOT_DY: [i8; 500] = [0; 500];

unsafe fn parse_robots(input: &str) {
    let input = input.as_bytes();
    let mut pos = 1;

    for n in 0..499 {
        let x = parse_num(input, pos);
        pos = pos.unchecked_add(len(x)).unchecked_add(1);
        *ROBOT_X.get_unchecked_mut(n) = x;

        let y = parse_num(input, pos);
        pos = pos.unchecked_add(len(y)).unchecked_add(3);
        *ROBOT_Y.get_unchecked_mut(n) = y;

        let dx = parse_num(input, pos);
        pos = pos.unchecked_add(len(dx)).unchecked_add(1);
        *ROBOT_DX.get_unchecked_mut(n) = dx;

        let dy = parse_num(input, pos);
        pos = pos.unchecked_add(len(dy)).unchecked_add(3);
        *ROBOT_DY.get_unchecked_mut(n) = dy;
    }

    crate::debug!(
        "Remaining text: {}",
        std::str::from_utf8(&input[pos..input.len() - 1]).unwrap()
    );

    // Last line may not have 3 bytes for final number
    {
        let n = 499;

        let x = parse_num(input, pos);
        pos = pos.unchecked_add(len(x)).unchecked_add(1);
        *ROBOT_X.get_unchecked_mut(n) = x;

        let y = parse_num(input, pos);
        pos = pos.unchecked_add(len(y)).unchecked_add(3);
        *ROBOT_Y.get_unchecked_mut(n) = y;

        let dx = parse_num(input, pos);
        pos = pos.unchecked_add(len(dx)).unchecked_add(1);
        *ROBOT_DX.get_unchecked_mut(n) = dx;

        let index = u32::from_ne_bytes([
            *input.get(pos.unchecked_add(3)).unwrap_or(&0),
            *input.get(pos.unchecked_add(2)).unwrap_or(&0),
            *input.get_unchecked(pos.unchecked_add(1)),
            0,
        ]);
        let dy = *LUT.get_unchecked(index as usize);
        *ROBOT_DY.get_unchecked_mut(n) = dy;
    }
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> i32 {
    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn inner(input: &str) -> i32 {
        const WIDTH: i32 = 101;
        const HEIGHT: i32 = 103;

        parse_robots(input);

        let mut x_timestep = 0;
        let mut y_timestep = 0;

        'outer: for timestep in 0..WIDTH {
            let mut arr = [0u8; WIDTH as _];

            for (&x, &dx) in ROBOT_X.iter().zip(ROBOT_DX.iter()) {
                let x = timestep
                    .unchecked_mul(dx as i32)
                    .unchecked_add(x as i32)
                    .checked_rem_euclid(WIDTH)
                    .unwrap_or_else(|| Unreachable.assume());
                *arr.get_unchecked_mut(x as usize) += 1;
            }

            for x in 0..WIDTH - 30 {
                if *arr.get_unchecked(x as usize) >= 33
                    && *arr.get_unchecked(x.unchecked_add(30) as usize) >= 33
                {
                    x_timestep = timestep;
                    break 'outer;
                }
            }
        }

        'outer: for timestep in 0..HEIGHT {
            let mut arr = [0u8; HEIGHT as _];

            for (&y, &dy) in ROBOT_Y.iter().zip(ROBOT_DY.iter()) {
                let y = timestep
                    .unchecked_mul(dy as i32)
                    .unchecked_add(y as i32)
                    .checked_rem_euclid(HEIGHT)
                    .unwrap_or_else(|| Unreachable.assume());
                *arr.get_unchecked_mut(y as usize) += 1;
            }

            for y in 0..HEIGHT - 32 {
                if *arr.get_unchecked(y as usize) >= 31
                    && *arr.get_unchecked(y.unchecked_add(32) as usize) >= 31
                {
                    y_timestep = timestep;
                    break 'outer;
                }
            }
        }

        // n * 103 + 86 = a
        // m * 101 + 57 = a
        // n * 101 * 103 + 86 * 103 = 103 a
        // m * 101 * 103 + 57 * 101 = 101 a
        // 2a = 86 * 103 - 57 * 101 + (n - m) * 101 * 103

        let x_factor = HEIGHT.unchecked_mul(x_timestep);
        let y_factor = WIDTH.unchecked_mul(y_timestep);
        let factor = x_factor.unchecked_sub(y_factor);
        let factor = const { WIDTH * HEIGHT }
            .unchecked_mul((factor < 0) as _)
            .unchecked_add(factor);
        let factor = const { WIDTH * HEIGHT }
            .unchecked_mul(factor % 2)
            .unchecked_add(factor);

        factor.unchecked_shr(1)
    }
    unsafe { inner(input) }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
    "};

    #[test]
    fn p1_example() {
        assert_eq!(unsafe { inner_p1::<12, 11, 7>(INPUT) }, 12);
    }

    #[test]
    fn real_p1() {
        let input = include_str!("../input/2024/day14.txt");
        assert_eq!(part1(input), 225_810_288);
    }

    #[test]
    fn real_p2() {
        let input = include_str!("../input/2024/day14.txt");
        assert_eq!(part2(input), 6_752);
    }

    #[test]
    fn lut_check() {
        for (bytes, val) in [("0,4", 0), ("-1,", -1), ("66,", 66), ("101", 101)] {
            let bytes = bytes.as_bytes();
            let lookup =
                ((bytes[0] as usize) << 16) + ((bytes[1] as usize) << 8) + bytes[2] as usize;
            assert_eq!(LUT[lookup] as i8, val);
        }
    }
}
