use std::{cmp::Reverse, collections::BinaryHeap, mem::transmute};

use aoc_runner_derive::aoc;

use crate::{ArrayVec, Assume as _, ConstDefault, Direction, IndexI16 as Index};

const WALL: u8 = b'#';

#[aoc(day16, part1)]
pub fn part1(input: &str) -> u32 {
    static mut COST: [[[u32; 4]; 141]; 141] = [[[u32::MAX; 4]; _]; _];

    unsafe { inner_p1::<141>(input, &mut COST) }
}

const fn estimate_cost<const DIM: usize>(Index { y, x }: Index<DIM>) -> u32 {
    (const { DIM * 2 - 2 } as u32) - x as u32 - y as u32 + (x != 0 && y != 0) as u32 * 1_000
}

#[derive(Debug, PartialEq, Eq)]
struct HeapEntry<const DIM: usize> {
    heuristic: u32,
    index: Index<DIM>,
    dir: Direction,
    #[cfg(test)]
    path: Vec<Direction>,
}

impl<const DIM: usize> PartialOrd for HeapEntry<DIM> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const DIM: usize> Ord for HeapEntry<DIM> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heuristic.cmp(&other.heuristic)
    }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_p1<const DIM: usize>(input: &str, curr_cost: &mut [[[u32; 4]; DIM]; DIM]) -> u32 {
    let input = input.as_bytes();
    let start = Index::<DIM> {
        x: 1,
        y: DIM as i16 - 2,
    };
    let end = Index::<DIM> {
        x: DIM as i16 - 2,
        y: 1,
    };

    curr_cost[start.y as usize][start.x as usize][Direction::East as usize] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(Reverse(HeapEntry {
        heuristic: estimate_cost(start),
        index: start,
        dir: Direction::East,
        #[cfg(test)]
        path: Vec::new(),
    }));

    while let Some(Reverse(HeapEntry {
        heuristic,
        index,
        dir,
        #[cfg(test)]
        path,
    })) = heap.pop()
    {
        if index == end {
            crate::debug!("Reached end with cost path: {path:?}");
            break;
        }

        #[cfg(test)]
        let mut path = path.clone();
        #[cfg(test)]
        path.push(dir);

        let base_cost = curr_cost[index.y as usize][index.x as usize][dir as usize];
        crate::debug!("Checking {index:?}, {dir:?} with cost {base_cost}");

        let rotation_cost = base_cost + 1_000;
        let clockwise =
            &mut curr_cost[index.y as usize][index.x as usize][dir.rotate_clockwise() as usize];

        if rotation_cost < *clockwise {
            *clockwise = rotation_cost;
            heap.push(Reverse(HeapEntry {
                heuristic: heuristic + 1_000,
                index,
                dir: dir.rotate_clockwise(),
                #[cfg(test)]
                path: path.clone(),
            }));
        } else {
            crate::debug!(
                "Skipping clockwise as existing cost of {clockwise} is lower than {rotation_cost}"
            );
        }

        let widdershins =
            &mut curr_cost[index.y as usize][index.x as usize][dir.rotate_widdershins() as usize];
        if rotation_cost < *widdershins {
            *widdershins = rotation_cost;
            heap.push(Reverse(HeapEntry {
                heuristic: heuristic + 1_000,
                index,
                dir: dir.rotate_widdershins(),
                #[cfg(test)]
                path: path.clone(),
            }));
        } else {
            crate::debug!("Skipping widdershins as existing cost of {widdershins} is lower than {rotation_cost}");
        }

        let new_index = index + dir.into();
        if *input.get_unchecked(new_index.to()) != WALL {
            let step_cost = base_cost + 1;
            let step = &mut curr_cost[new_index.y as usize][new_index.x as usize][dir as usize];

            if step_cost < *step {
                *step = step_cost;
                heap.push(Reverse(HeapEntry {
                    heuristic: step_cost + estimate_cost(new_index),
                    index: new_index,
                    dir,
                    #[cfg(test)]
                    path,
                }));
            } else {
                crate::debug!(
                    "Skipping step as existing cost of {step} is lower than {rotation_cost}"
                );
            }
        }
    }

    *curr_cost[end.y as usize][end.x as usize]
        .iter()
        .min()
        .assume()
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> u32 {
    static mut COST: [[[u32; 4]; 141]; 141] = [[[u32::MAX; 4]; _]; _];
    static mut STACK: ArrayVec<200, StackEntry<141>> = ArrayVec::new();
    static mut VISITS: [[bool; 141]; 141] = [[false; _]; _];

    unsafe { inner_p2::<141>(input, &mut COST, &mut STACK, &mut VISITS) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_p2<const DIM: usize>(
    input: &str,
    curr_cost: &mut [[[u32; 4]; DIM]; DIM],
    stack: &mut ArrayVec<200, StackEntry<DIM>>,
    visited: &mut [[bool; DIM]; DIM],
) -> u32 {
    let input = input.as_bytes();
    let start = Index::<DIM> {
        x: 1,
        y: DIM as i16 - 2,
    };
    let end = Index::<DIM> {
        x: DIM as i16 - 2,
        y: 1,
    };

    curr_cost[start.y as usize][start.x as usize][Direction::East as usize] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(Reverse(HeapEntry {
        heuristic: 0,
        index: start,
        dir: Direction::East,
        #[cfg(test)]
        path: Vec::new(),
    }));

    while let Some(Reverse(HeapEntry {
        heuristic,
        index,
        dir,
        #[cfg(test)]
        path,
    })) = heap.pop()
    {
        #[cfg(test)]
        let mut path = path.clone();
        #[cfg(test)]
        path.push(dir);

        let base_cost = curr_cost[index.y as usize][index.x as usize][dir as usize];
        crate::debug!("Checking {index:?}, {dir:?} with cost {base_cost}");

        let rotation_cost = base_cost + 1_000;
        let clockwise =
            &mut curr_cost[index.y as usize][index.x as usize][dir.rotate_clockwise() as usize];

        if rotation_cost < *clockwise {
            *clockwise = rotation_cost;
            heap.push(Reverse(HeapEntry {
                heuristic: heuristic + 1_000,
                index,
                dir: dir.rotate_clockwise(),
                #[cfg(test)]
                path: path.clone(),
            }));
        } else {
            crate::debug!(
                "Skipping clockwise as existing cost of {clockwise} is lower than {rotation_cost}"
            );
        }

        let widdershins =
            &mut curr_cost[index.y as usize][index.x as usize][dir.rotate_widdershins() as usize];
        if rotation_cost < *widdershins {
            *widdershins = rotation_cost;
            heap.push(Reverse(HeapEntry {
                heuristic: heuristic + 1_000,
                index,
                dir: dir.rotate_widdershins(),
                #[cfg(test)]
                path: path.clone(),
            }));
        } else {
            crate::debug!("Skipping widdershins as existing cost of {widdershins} is lower than {rotation_cost}");
        }

        let new_index = index + dir.into();
        if *input.get_unchecked(new_index.to()) != WALL {
            let step_cost = base_cost + 1;
            let step = &mut curr_cost[new_index.y as usize][new_index.x as usize][dir as usize];

            if step_cost < *step {
                *step = step_cost;
                heap.push(Reverse(HeapEntry {
                    heuristic: step_cost,
                    index: new_index,
                    dir,
                    #[cfg(test)]
                    path,
                }));
            } else {
                crate::debug!(
                    "Skipping step as existing cost of {step} is lower than {rotation_cost}"
                );
            }
        }
    }

    let (_value, index) = curr_cost[end.y as usize][end.x as usize]
        .iter()
        .enumerate()
        .map(|(index, val)| (*val, index))
        .min()
        .assume();

    stack.push_unchecked(StackEntry {
        index: end,
        dir: transmute(index as u8),
    });
    let mut count = 0;

    while let Some(StackEntry { index, dir }) = stack.pop() {
        let curr_tile = &mut curr_cost[index.y as usize][index.x as usize];
        let value = curr_tile[dir as usize];

        let visited = &mut visited[index.y as usize][index.x as usize];
        if !*visited {
            *visited = true;
            count += 1;
        }

        crate::debug!("Path through {index:?}, {dir:?} with value {value}");

        if value >= 1_000 {
            if curr_tile[dir.rotate_clockwise() as usize] == value - 1_000 {
                stack.push_unchecked(StackEntry {
                    index,
                    dir: dir.rotate_clockwise(),
                });
            }

            if curr_tile[dir.rotate_widdershins() as usize] == value - 1_000 {
                stack.push_unchecked(StackEntry {
                    index,
                    dir: dir.rotate_widdershins(),
                });
            }
        }

        if value >= 1 {
            let old_pos = index - dir.into();
            if curr_cost[old_pos.y as usize][old_pos.x as usize][dir as usize] == value - 1 {
                stack.push_unchecked(StackEntry {
                    index: old_pos,
                    dir,
                });
            }
        }
    }

    count
}

#[derive(Debug, Clone, Copy)]
struct StackEntry<const DIM: usize> {
    index: Index<DIM>,
    dir: Direction,
}

impl<const DIM: usize> ConstDefault for StackEntry<DIM> {
    const DEFAULT: Self = Self {
        index: Index::DEFAULT,
        dir: Direction::North,
    };
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn p1_example_1() {
        let input = indoc! {"
            ###############
            #.......#....E#
            #.#.###.#.###.#
            #.....#.#...#.#
            #.###.#####.#.#
            #.#.#.......#.#
            #.#.#####.###.#
            #...........#.#
            ###.#.#####.#.#
            #...#.....#.#.#
            #.#.#.###.#.#.#
            #.....#...#.#.#
            #.###.#.#.#.#.#
            #S..#.....#...#
            ###############
        "};

        let mut costs = [[[u32::MAX; _]; _]; _];

        assert_eq!(unsafe { inner_p1::<15>(input, &mut costs) }, 7_036);
    }

    #[test]
    fn p1_example_2() {
        let input = indoc! {"
            #################
            #...#...#...#..E#
            #.#.#.#.#.#.#.#.#
            #.#.#.#...#...#.#
            #.#.#.#.###.#.#.#
            #...#.#.#.....#.#
            #.#.#.#.#.#####.#
            #.#...#.#.#.....#
            #.#.#####.#.###.#
            #.#.#.......#...#
            #.#.###.#####.###
            #.#.#...#.....#.#
            #.#.#.#####.###.#
            #.#.#.........#.#
            #.#.#.#########.#
            #S#.............#
            #################
        "};

        let mut costs = [[[u32::MAX; _]; _]; _];

        assert_eq!(unsafe { inner_p1::<17>(input, &mut costs) }, 11_048);
    }

    #[test]
    fn real_p1() {
        let input = include_str!("../input/2024/day16.txt");
        assert_eq!(part1(input), 72_428);
    }

    #[test]
    fn p2_example_1() {
        let input = indoc! {"
            ###############
            #.......#....E#
            #.#.###.#.###.#
            #.....#.#...#.#
            #.###.#####.#.#
            #.#.#.......#.#
            #.#.#####.###.#
            #...........#.#
            ###.#.#####.#.#
            #...#.....#.#.#
            #.#.#.###.#.#.#
            #.....#...#.#.#
            #.###.#.#.#.#.#
            #S..#.....#...#
            ###############
        "};

        let mut costs = [[[u32::MAX; _]; _]; _];
        let mut stack = ArrayVec::new();
        let mut visits = [[false; _]; _];

        assert_eq!(
            unsafe { inner_p2::<15>(input, &mut costs, &mut stack, &mut visits) },
            45
        );
    }

    #[test]
    fn p2_example_2() {
        let input = indoc! {"
            #################
            #...#...#...#..E#
            #.#.#.#.#.#.#.#.#
            #.#.#.#...#...#.#
            #.#.#.#.###.#.#.#
            #...#.#.#.....#.#
            #.#.#.#.#.#####.#
            #.#...#.#.#.....#
            #.#.#####.#.###.#
            #.#.#.......#...#
            #.#.###.#####.###
            #.#.#...#.....#.#
            #.#.#.#####.###.#
            #.#.#.........#.#
            #.#.#.#########.#
            #S#.............#
            #################
        "};

        let mut costs = [[[u32::MAX; _]; _]; _];
        let mut stack = ArrayVec::new();
        let mut visits = [[false; _]; _];

        assert_eq!(
            unsafe { inner_p2::<17>(input, &mut costs, &mut stack, &mut visits) },
            64
        );
    }

    #[test]
    fn real_p2() {
        let input = include_str!("../input/2024/day16.txt");
        assert_eq!(part2(input), 456);
    }
}
