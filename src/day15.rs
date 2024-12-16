use std::{
    fmt::Display,
    mem::transmute,
    ops::{Index, IndexMut},
};

use aoc_runner_derive::aoc;

use crate::{ArrayVec, Assume, IndexI8, Unreachable};

const WALL: u8 = b'#';
const EMPTY: u8 = b'.';
const OBJECT: u8 = b'O';
const ROBOT: u8 = b'@';

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum CellP1 {
    Empty = EMPTY,
    // Compiler sees it as never created, due to being created via [`transmute`]
    #[expect(dead_code)]
    Wall = WALL,
    // Compiler sees it as never created, due to being created via [`transmute`]
    #[expect(dead_code)]
    Object = OBJECT,
    // Compiler sees it as never created, due to being created via [`transmute`]
    #[expect(dead_code)]
    Robot = ROBOT,
}

impl CellP1 {
    fn is_robot(&self) -> bool {
        matches!(self, Self::Robot)
    }

    fn is_object(&self) -> bool {
        matches!(self, Self::Object)
    }
}

struct FieldP1<const DIM: usize> {
    inner: [CellP1; 50 * 50],
}

impl<const DIM: usize> Display for FieldP1<DIM> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..DIM {
            for x in 0..DIM {
                write!(f, "{}", self[y][x] as u8 as char)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<const DIM: usize> FieldP1<DIM> {
    const fn new() -> Self {
        Self {
            inner: [CellP1::Empty; _],
        }
    }

    fn value(&self) -> usize {
        let mut sum = 0;
        for y in 0..DIM {
            for x in 0..DIM {
                if self[y][x].is_object() {
                    sum += 100 * y + x;
                }
            }
        }

        sum
    }
}

impl<const DIM: usize> Index<usize> for FieldP1<DIM> {
    type Output = [CellP1; DIM];

    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            self.inner
                .as_ptr()
                .add(index * DIM)
                .cast::<Self::Output>()
                .as_ref_unchecked()
        }
    }
}

impl<const DIM: usize> IndexMut<usize> for FieldP1<DIM> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe {
            self.inner
                .as_mut_ptr()
                .add(index * DIM)
                .cast::<Self::Output>()
                .as_mut_unchecked()
        }
    }
}

impl<const DIM: usize> Index<IndexI8<DIM>> for FieldP1<DIM> {
    type Output = CellP1;

    fn index(&self, index: IndexI8<DIM>) -> &Self::Output {
        &self[index.y as usize][index.x as usize]
    }
}

impl<const DIM: usize> IndexMut<IndexI8<DIM>> for FieldP1<DIM> {
    fn index_mut(&mut self, index: IndexI8<DIM>) -> &mut Self::Output {
        &mut self[index.y as usize][index.x as usize]
    }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn read_field_p1<const DIM: usize>(input: &[u8], field: &mut FieldP1<DIM>) -> IndexI8<DIM> {
    let input: &[CellP1] = transmute(input);
    let mut start_index = IndexI8 { x: 0, y: 0 };

    for line in 0..DIM {
        let pos = line * (DIM + 1);
        let cell_line = &mut field[line];
        *cell_line = input
            .as_ptr()
            .add(pos)
            .cast::<[CellP1; DIM]>()
            .read_unaligned();

        if let Some(x) = cell_line.iter().position(CellP1::is_robot) {
            start_index = IndexI8 {
                x: x as _,
                y: line as _,
            };
        }
    }

    debug_assert!(start_index != IndexI8 { x: 0, y: 0 });
    field[start_index] = CellP1::Empty;
    start_index
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    static mut FIELD: FieldP1<50> = FieldP1::new();
    let input = input.as_bytes();

    unsafe {
        let initial = read_field_p1(input, &mut FIELD);
        inner_p1(input, &mut FIELD, initial)
    }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_p1<const DIM: usize>(
    input: &[u8],
    field: &mut FieldP1<DIM>,
    mut pos: IndexI8<DIM>,
) -> usize {
    let input = input.get_unchecked((DIM + 1) * DIM + 1..);

    for c in input {
        let dir = match c {
            b'>' => IndexI8::RIGHT,
            b'^' => IndexI8::UP,
            b'<' => IndexI8::LEFT,
            b'V' | b'v' => IndexI8::DOWN,
            b'\n' => IndexI8::ZERO,
            _c => {
                crate::debug!("Unexpected character {_c} ({})", *_c as char);
                Unreachable.assume();
            }
        };

        crate::debug!("At {pos:?} moving by {} ({dir:?})", *c as char);

        let new_pos = pos + dir;
        let mut object_push = pos + dir;
        match field[object_push] {
            CellP1::Empty | CellP1::Robot => pos = new_pos,
            CellP1::Wall => (),
            CellP1::Object => {
                while field[object_push].is_object() {
                    object_push += dir;
                }

                match field[object_push] {
                    CellP1::Empty | CellP1::Robot => {
                        field[object_push] = CellP1::Object;
                        field[new_pos] = CellP1::Empty;
                        pos = new_pos;
                    }
                    CellP1::Wall => (),
                    CellP1::Object => Unreachable.assume(),
                }
            }
        }

        crate::debug!("Map:\n{field}");
    }

    field.value()
}

#[derive(Debug, Clone, Copy)]
enum CellP2 {
    Empty,
    Wall,
    ObjectLeft,
    ObjectRight,
}

impl CellP2 {
    fn _as_char(&self) -> char {
        match self {
            CellP2::Empty => '.',
            CellP2::Wall => '#',
            CellP2::ObjectLeft => '[',
            CellP2::ObjectRight => ']',
        }
    }
}

struct FieldP2<const DIM: usize> {
    inner: [CellP2; 100 * 50],
}

impl<const DIM: usize> FieldP2<DIM>
where
    [(); 2 * DIM]:,
{
    const fn new() -> Self {
        Self {
            inner: [CellP2::Empty; _],
        }
    }

    fn value(&self) -> usize {
        let mut sum = 0;
        for y in 0..DIM {
            for x in 0..DIM * 2 {
                if matches!(self[y][x], CellP2::ObjectLeft) {
                    sum += 100 * y + x;
                }
            }
        }

        sum
    }

    fn _print(&self, robot: IndexI8<{ 2 * DIM }>) -> String {
        let mut s = String::with_capacity(DIM * 2 * DIM);
        for y in 0..DIM {
            for x in 0..DIM * 2 {
                s.push(
                    if (IndexI8 {
                        x: x as _,
                        y: y as _,
                    }) == robot
                    {
                        '@'
                    } else {
                        self[y][x]._as_char()
                    },
                );
            }
            s.push('\n');
        }

        s
    }
}

impl<const DIM: usize> Index<usize> for FieldP2<DIM>
where
    [(); 2 * DIM]:,
{
    type Output = [CellP2; 2 * DIM];

    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            self.inner
                .as_ptr()
                .add(index * 2 * DIM)
                .cast::<Self::Output>()
                .as_ref_unchecked()
        }
    }
}

impl<const DIM: usize> IndexMut<usize> for FieldP2<DIM>
where
    [(); 2 * DIM]:,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe {
            self.inner
                .as_mut_ptr()
                .add(index * 2 * DIM)
                .cast::<Self::Output>()
                .as_mut_unchecked()
        }
    }
}

impl<const DIM: usize> Index<IndexI8<{ 2 * DIM }>> for FieldP2<DIM> {
    type Output = CellP2;

    fn index(&self, index: IndexI8<{ 2 * DIM }>) -> &Self::Output {
        &self[index.y as usize][index.x as usize]
    }
}

impl<const DIM: usize> IndexMut<IndexI8<{ 2 * DIM }>> for FieldP2<DIM> {
    fn index_mut(&mut self, index: IndexI8<{ 2 * DIM }>) -> &mut Self::Output {
        &mut self[index.y as usize][index.x as usize]
    }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn read_field_p2<const DIM: usize>(
    input: &[u8],
    field: &mut FieldP2<DIM>,
) -> IndexI8<{ 2 * DIM }>
where
    [(); 2 * DIM]:,
{
    let mut start_index = IndexI8 { x: 0, y: 0 };

    for y in 0..DIM {
        for x in 0..DIM {
            let c = input[y * (DIM + 1) + x];
            let cell = match c {
                ROBOT => {
                    start_index = IndexI8 {
                        x: x as i8 * 2,
                        y: y as _,
                    };
                    [CellP2::Empty; 2]
                }
                WALL => [CellP2::Wall; 2],
                EMPTY => [CellP2::Empty; 2],
                OBJECT => [CellP2::ObjectLeft, CellP2::ObjectRight],
                _c => {
                    crate::debug!("Unreachable char {_c} ({})", _c as char);
                    Unreachable.assume();
                }
            };

            field[y][2 * x] = cell[0];
            field[y][2 * x + 1] = cell[1];

            // crate::debug!(
            //     "Field builder at ({y}, {x}) with {}:\n{}",
            //     c as char,
            //     field.print(start_index)
            // )
        }
    }

    debug_assert!(start_index != IndexI8 { x: 0, y: 0 });
    start_index
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> usize {
    static mut FIELD: FieldP2<50> = FieldP2::new();
    static mut STACK: ArrayVec<1_000, IndexI8<100>> = ArrayVec::new();
    let input = input.as_bytes();

    unsafe {
        let initial = read_field_p2(input, &mut FIELD);
        inner_p2(input, &mut FIELD, initial, &mut STACK)
    }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_p2<const DIM: usize>(
    input: &[u8],
    field: &mut FieldP2<DIM>,
    mut pos: IndexI8<{ 2 * DIM }>,
    stack: &mut ArrayVec<1_000, IndexI8<{ 2 * DIM }>>,
) -> usize
where
    [(); 2 * DIM]:,
{
    let input = input.get_unchecked((DIM + 1) * DIM + 1..);

    'outer: for c in input {
        let dir = match c {
            b'>' => IndexI8::RIGHT,
            b'^' => IndexI8::UP,
            b'<' => IndexI8::LEFT,
            b'V' | b'v' => IndexI8::DOWN,
            b'\n' => IndexI8::ZERO,
            _c => {
                crate::debug!("Unexpected character {_c} ({})", *_c as char);
                Unreachable.assume();
            }
        };

        crate::debug!("At {pos:?} moving by {} ({dir:?})", *c as char);

        let new_pos = pos + dir;
        match field[new_pos] {
            CellP2::Empty => pos = new_pos,
            CellP2::Wall => (),
            CellP2::ObjectLeft | CellP2::ObjectRight => {
                let mut queued = [0_u128; DIM];

                stack.push_unchecked(pos);
                for index in 0.. {
                    if index >= stack.len {
                        break;
                    }

                    let pos = stack.get_unchecked(index);
                    let new_pos = pos + dir;
                    match field[new_pos] {
                        CellP2::Empty => (),
                        CellP2::Wall => {
                            crate::debug!("Hit a wall");
                            stack.clear();
                            continue 'outer;
                        }
                        CellP2::ObjectLeft => {
                            {
                                let queued = &mut queued[new_pos.y as usize];
                                if (*queued & 1 << new_pos.x) == 0 {
                                    *queued |= 1 << new_pos.x;
                                    stack.push_unchecked(new_pos);
                                }
                            }
                            {
                                let new_pos = new_pos + IndexI8::RIGHT;
                                let queued = &mut queued[new_pos.y as usize];
                                if (*queued & 1 << new_pos.x) == 0 {
                                    *queued |= 1 << new_pos.x;
                                    stack.push_unchecked(new_pos);
                                }
                            }
                        }
                        CellP2::ObjectRight => {
                            {
                                let queued = &mut queued[new_pos.y as usize];
                                if (*queued & 1 << new_pos.x) == 0 {
                                    *queued |= 1 << new_pos.x;
                                    stack.push_unchecked(new_pos);
                                }
                            }
                            {
                                let new_pos = new_pos + IndexI8::LEFT;
                                let queued = &mut queued[new_pos.y as usize];
                                if (*queued & 1 << new_pos.x) == 0 {
                                    *queued |= 1 << new_pos.x;
                                    stack.push_unchecked(new_pos);
                                }
                            }
                        }
                    }
                }

                crate::debug!("Moving {} tiles by {} ({dir:?})", stack.len, *c as char);
                pos += dir;
                while let Some(pos) = stack.pop() {
                    crate::debug!(
                        "Moving {} from {pos:?} to {:?}",
                        field[pos]._as_char(),
                        pos + dir
                    );
                    field[pos + dir] = field[pos];
                    field[pos] = CellP2::Empty;
                }
            }
        }

        crate::debug!("Map:\n{}", field._print(pos));
    }

    field.value()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn p1_large_example() {
        let input = indoc! {"
            ##########
            #..O..O.O#
            #......O.#
            #.OO..O.O#
            #..O@..O.#
            #O#..O...#
            #O..O..O.#
            #.OO.O.OO#
            #....O...#
            ##########

            <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
            vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
            ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
            <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
            ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
            ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
            >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
            <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
            ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
            v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
        "}
        .as_bytes();

        let mut field = FieldP1::<10>::new();
        unsafe {
            let start = read_field_p1(input, &mut field);
            assert_eq!(inner_p1(input, &mut field, start), 10_092);
        }
    }

    #[test]
    fn p1_small_example() {
        let input = indoc! {"
            ########
            #..O.O.#
            ##@.O..#
            #...O..#
            #.#.O..#
            #...O..#
            #......#
            ########

            <^^>>>vv<v>>v<<
        "}
        .as_bytes();

        let mut field = FieldP1::<8>::new();
        unsafe {
            let start = read_field_p1(input, &mut field);
            assert_eq!(inner_p1(input, &mut field, start), 2_028);
        }
    }

    #[test]
    fn real_p1() {
        let input = include_str!("../input/2024/day15.txt");
        assert_eq!(part1(input), 1_441_031);
    }

    #[test]
    fn real_p2() {
        let input = include_str!("../input/2024/day15.txt");
        assert_eq!(part2(input), 1_425_169);
    }

    #[test]
    fn p2_small_example() {
        let input = indoc! {"
            #######
            #...#.#
            #.....#
            #..OO@#
            #..O..#
            #.....#
            #######

            <vv<<^^<<^^
        "}
        .as_bytes();

        let mut stack = ArrayVec::new();

        let mut field = FieldP2::<7>::new();
        unsafe {
            let start = read_field_p2(input, &mut field);
            assert_eq!(
                inner_p2(input, &mut field, start, &mut stack),
                105 + 207 + 306
            );
        }
    }

    #[test]
    fn p2_large_example() {
        let input = indoc! {"
            ##########
            #..O..O.O#
            #......O.#
            #.OO..O.O#
            #..O@..O.#
            #O#..O...#
            #O..O..O.#
            #.OO.O.OO#
            #....O...#
            ##########

            <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
            vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
            ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
            <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
            ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
            ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
            >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
            <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
            ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
            v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
        "}
        .as_bytes();

        let mut stack = ArrayVec::new();

        let mut field = FieldP2::<10>::new();
        unsafe {
            let start = read_field_p2(input, &mut field);
            assert_eq!(inner_p2(input, &mut field, start, &mut stack), 9_021);
        }
    }
}
