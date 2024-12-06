use std::collections::HashSet;
use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input, tracing_init, Point};

fn main() {
    tracing_init();

    let input = get_input("day06.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[tracing::instrument(skip_all)]
fn solve(input: &Vec<String>) -> (impl Display, impl Display) {
    let mut map: Map = input.into();
    let m2 = map.clone();

    let path = map.get_path();

    let p1 = path.len();
    let p2 = m2.find_loops(&path);

    (p1, p2)
}

type Position = Point<usize>;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Clone)]
struct Map {
    width: usize,
    height: usize,
    guard_pos: Position,
    guard_dir: Direction,
    obstacles: Vec<Position>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            guard_pos: Point::new(0, 0),
            guard_dir: Direction::Up,
            obstacles: Vec::new(),
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.height {
            for y in 0..self.width {
                let p = Point::new(x, y);
                if p == self.guard_pos {
                    write!(f, "^")?;
                } else if self.obstacles.contains(&p) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl From<&Vec<String>> for Map {
    fn from(value: &Vec<String>) -> Self {
        let mut guard_pos = None;
        let mut obstacles = Vec::new();

        for (x, row) in value.iter().enumerate() {
            for (y, p) in row.chars().enumerate() {
                match p {
                    '^' => guard_pos = Some(Point::new(x, y)),
                    '#' => obstacles.push(Point::new(x, y)),
                    _ => {}
                }
            }
        }

        Self {
            width: value[0].len(),
            height: value.len(),
            guard_pos: guard_pos.unwrap(),
            obstacles,
            ..Default::default()
        }
    }
}

impl Map {
    fn get_path(&mut self) -> Vec<Position> {
        let mut visited = HashSet::new();

        visited.insert(self.guard_pos);

        loop {
            if let Err(()) = self.move_guard() {
                break;
            }

            visited.insert(self.guard_pos);
        }

        visited.iter().copied().collect()
    }

    fn find_loops(&self, path: &[Position]) -> usize {
        let mut count = 0;

        // Skip the starting position
        for p in path.iter().filter(|&&p| p != self.guard_pos) {
            println!("Testing x={}, y={}", p.x, p.y);

            let mut map = self.clone();
            map.obstacles.push(*p);

            let mut visited = Vec::new();
            visited.push((map.guard_pos, map.guard_dir));

            loop {
                if let Err(()) = map.move_guard() {
                    break;
                }

                if visited.contains(&(map.guard_pos, map.guard_dir)) {
                    count += 1;
                    break;
                }

                visited.push((map.guard_pos, map.guard_dir));
            }
        }

        count
    }

    fn move_guard(&mut self) -> Result<(), ()> {
        let next = match self.guard_dir {
            Direction::Up => {
                if self.guard_pos.x == 0 {
                    return Err(());
                }

                Point::new(self.guard_pos.x - 1, self.guard_pos.y)
            }
            Direction::Down => {
                if self.guard_pos.x == self.height - 1 {
                    return Err(());
                }

                Point::new(self.guard_pos.x + 1, self.guard_pos.y)
            }
            Direction::Left => {
                if self.guard_pos.y == 0 {
                    return Err(());
                }

                Point::new(self.guard_pos.x, self.guard_pos.y - 1)
            }
            Direction::Right => {
                if self.guard_pos.y == self.width - 1 {
                    return Err(());
                }

                Point::new(self.guard_pos.x, self.guard_pos.y + 1)
            }
        };

        if self.obstacles.contains(&next) {
            self.guard_dir = self.guard_dir.turn_right();
        } else {
            self.guard_pos = next;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use aoc_common::parse_test_input;
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn test_input() -> Vec<String> {
        parse_test_input(
            "
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
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day06.txt")
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let mut map: Map = (&test_input).into();
        let res = map.get_path().len();

        assert_eq!(res, 41);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let mut map: Map = (&puzzle_input).into();
        let res = map.get_path().len();

        assert_eq!(res, 4826);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let mut map: Map = (&test_input).into();
        let m2 = map.clone();

        let path = map.get_path();
        let res = m2.find_loops(&path);

        assert_eq!(res, 6);
    }

    #[rstest]
    #[ignore] // Test is very slow
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let mut map: Map = (&puzzle_input).into();
        let m2 = map.clone();

        let path = map.get_path();
        let res = m2.find_loops(&path);

        assert_eq!(res, 1721);
    }
}
