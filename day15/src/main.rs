use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input, tracing_init, Point};
use itertools::Itertools;

fn main() {
    tracing_init();

    let input = get_input("day15.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[tracing::instrument(skip_all)]
fn solve(input: &[String]) -> (impl Display, impl Display) {
    let (mut map, instrs) = parse(input);
    map.run(&instrs);

    let p1 = map.sum_gps_coords();
    let p2 = 0;

    (p1, p2)
}

#[tracing::instrument(skip_all)]
fn parse(input: &[String]) -> (Map, Vec<Direction>) {
    let (layout, instrs) = input.splitn(2, |s| s.is_empty()).collect_tuple().unwrap();

    let mut robot = Position::new(0, 0);
    let height = layout.len();
    let width = layout[0].len();

    let mut tiles = Vec::with_capacity(height);
    for (y, l) in layout.iter().enumerate() {
        let mut row = Vec::with_capacity(width);

        for (x, c) in l.chars().enumerate() {
            let tile = match c {
                '@' => {
                    robot.x = x;
                    robot.y = y;

                    Tile::Empty
                }
                '#' => Tile::Wall,
                'O' => Tile::Box,
                _ => Tile::Empty,
            };

            row.push(tile);
        }

        tiles.push(row);
    }

    let map = Map { robot, tiles };

    let instructions = instrs
        .iter()
        .flat_map(|i| {
            i.chars().map(|c| match c {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => panic!("Invalid instruction: {}", c),
            })
        })
        .collect_vec();

    (map, instructions)
}

type Position = Point<usize>;

struct Map {
    tiles: Vec<Vec<Tile>>,
    robot: Position,
}

impl Map {
    #[tracing::instrument(skip_all)]
    fn run(&mut self, instrs: &[Direction]) {
        // println!("{}", self);

        for d in instrs {
            let dst = match d {
                Direction::Up => Point::new(self.robot.x, self.robot.y - 1),
                Direction::Down => Point::new(self.robot.x, self.robot.y + 1),
                Direction::Left => Point::new(self.robot.x - 1, self.robot.y),
                Direction::Right => Point::new(self.robot.x + 1, self.robot.y),
            };

            if self.tiles[dst.y][dst.x] == Tile::Empty {
                self.robot.x = dst.x;
                self.robot.y = dst.y;
            } else if let Some(e) = self.find_next_empty(d) {
                self.robot.x = dst.x;
                self.robot.y = dst.y;

                self.tiles[dst.y][dst.x] = Tile::Empty;
                self.tiles[e.y][e.x] = Tile::Box;
            }

            // println!("Move: {:?}", d);
            // println!("{}\n", self);
        }
    }

    fn sum_gps_coords(&self) -> usize {
        self.tiles
            .iter()
            .enumerate()
            .map(|(y, r)| {
                r.iter()
                    .enumerate()
                    .filter_map(|(x, t)| {
                        if let Tile::Box = t {
                            Some(x + 100 * y)
                        } else {
                            None
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn find_next_empty(&self, dir: &Direction) -> Option<Position> {
        let mut p = self.robot;

        loop {
            match dir {
                Direction::Up => p.y -= 1,
                Direction::Down => p.y += 1,
                Direction::Left => p.x -= 1,
                Direction::Right => p.x += 1,
            };

            match self.tiles[p.y][p.x] {
                Tile::Box => {
                    continue;
                }
                Tile::Wall => {
                    return None;
                }
                Tile::Empty => {
                    return Some(p);
                }
            };
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.tiles.iter().enumerate() {
            let s = row
                .iter()
                .enumerate()
                .map(|(x, t)| {
                    if x == self.robot.x && y == self.robot.y {
                        '@'
                    } else {
                        t.as_char()
                    }
                })
                .collect::<String>();
            writeln!(f, "{}", s)?;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Box,
    Wall,
    Empty,
}

impl Tile {
    fn as_char(&self) -> char {
        match self {
            Tile::Box => 'O',
            Tile::Wall => '#',
            Tile::Empty => ' ',
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use aoc_common::parse_test_input;
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn small_test_input() -> Vec<String> {
        parse_test_input(
            "
            ########
            #..O.O.#
            ##@.O..#
            #...O..#
            #.#.O..#
            #...O..#
            #......#
            ########

            <^^>>>vv<v>>v<<
        ",
        )
    }

    #[fixture]
    fn test_input() -> Vec<String> {
        parse_test_input(
            "
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
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day15.txt")
    }

    #[rstest]
    fn test_p1_small(small_test_input: Vec<String>) {
        let (mut map, instrs) = parse(&small_test_input);
        map.run(&instrs);

        let res = map.sum_gps_coords();

        assert_eq!(res, 2028);
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let (mut map, instrs) = parse(&test_input);
        map.run(&instrs);

        let res = map.sum_gps_coords();

        assert_eq!(res, 10092);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let (mut map, instrs) = parse(&puzzle_input);
        map.run(&instrs);

        let res = map.sum_gps_coords();

        assert_eq!(res, 1437174);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let res = 0;

        assert_eq!(res, 1);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let res = 0;

        assert_eq!(res, 1);
    }
}
