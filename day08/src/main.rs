use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input, tracing_init, Point};
use itertools::Itertools;

fn main() {
    tracing_init();

    let input = get_input("day08.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[tracing::instrument(skip_all)]
fn solve(input: &[String]) -> (impl Display, impl Display) {
    let map = Map::from(input);
    let p1 = map.count_unique_antinode_pos();
    let p2 = map.count_unique_real_antinode_pos();

    (p1, p2)
}

type Position = Point<usize>;

struct Antenna {
    freq: char,
    pos: Position,
}

struct Map {
    antennas: Vec<Antenna>,
    antinodes: Vec<Position>,
    real_antinodes: Vec<Position>,

    height: usize,
    width: usize,
}

impl From<&[String]> for Map {
    fn from(value: &[String]) -> Self {
        let mut antennas = Vec::new();

        let height = value.len();
        let width = value[0].len();

        for (x, row) in value.iter().enumerate() {
            for (y, c) in row.chars().enumerate().filter(|(_, c)| *c != '.') {
                antennas.push(Antenna {
                    pos: Position::new(x, y),
                    freq: c,
                })
            }
        }

        let antinodes = Self::get_antinodes(&antennas, width, height);
        let real_antinodes = Self::get_real_antinodes(&antennas, width, height);

        Self {
            antennas,
            antinodes,
            real_antinodes,
            height,
            width,
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.height {
            for y in 0..self.width {
                let p = Position::new(x, y);

                let anode = self.real_antinodes.iter().any(|&a| a == p);

                let val = self
                    .antennas
                    .iter()
                    .find(|a| a.pos == p)
                    .map_or(if anode { '#' } else { '.' }, |a| a.freq);

                write!(f, "{}", val)?
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Map {
    fn get_antinodes(antennas: &[Antenna], width: usize, height: usize) -> Vec<Position> {
        let mut antinodes = Vec::new();

        let antenna_by_freq = {
            let mut map: HashMap<char, Vec<Position>> = HashMap::new();

            for a in antennas {
                map.entry(a.freq)
                    .and_modify(|v| v.push(a.pos))
                    .or_insert(vec![a.pos]);
            }

            map
        };

        for (_, positions) in antenna_by_freq {
            for pp in positions.iter().permutations(2) {
                let a = pp[0];
                let b = pp[1];

                let dx = ((a.x as i32) - (b.x as i32)).unsigned_abs() as usize;
                let dy = ((a.y as i32) - (b.y as i32)).unsigned_abs() as usize;

                if a.x < b.x && a.y < b.y {
                    if a.x >= dx && a.y >= dy {
                        antinodes.push(Position::new(a.x - dx, a.y - dy));
                    }

                    if b.x + dx < height && b.y + dy < width {
                        antinodes.push(Position::new(b.x + dx, b.y + dy));
                    }
                } else if a.x < b.x && a.y >= b.y {
                    if a.x >= dx && a.y + dy < width {
                        antinodes.push(Position::new(a.x - dx, a.y + dy));
                    }

                    if b.x + dx < height && b.y >= dy {
                        antinodes.push(Position::new(b.x + dx, b.y - dy));
                    }
                }
            }
        }

        antinodes
    }

    fn get_real_antinodes(antennas: &[Antenna], width: usize, height: usize) -> Vec<Position> {
        let mut antinodes = Vec::new();

        let antenna_by_freq = {
            let mut map: HashMap<char, Vec<Position>> = HashMap::new();

            for a in antennas {
                map.entry(a.freq)
                    .and_modify(|v| v.push(a.pos))
                    .or_insert(vec![a.pos]);
            }

            map
        };

        for (_, positions) in antenna_by_freq {
            for pp in positions.iter().combinations(2) {
                let a = pp[0];
                let b = pp[1];

                let dx = ((a.x as i32) - (b.x as i32)).unsigned_abs() as usize;
                let dy = ((a.y as i32) - (b.y as i32)).unsigned_abs() as usize;

                antinodes.push(*a);
                antinodes.push(*b);

                if a.x < b.x && a.y < b.y {
                    let mut x = a.x;
                    let mut y = a.y;

                    while x >= dx && y >= dy {
                        x -= dx;
                        y -= dy;

                        antinodes.push(Position::new(x, y));
                    }

                    x = b.x;
                    y = b.y;

                    while x + dx < height && y + dy < width {
                        x += dx;
                        y += dy;

                        antinodes.push(Position::new(x, y));
                    }
                } else if a.x < b.x && a.y >= b.y {
                    let mut x = a.x;
                    let mut y = a.y;

                    while x >= dx && y + dy < width {
                        x -= dx;
                        y += dy;

                        antinodes.push(Position::new(x, y));
                    }

                    x = b.x;
                    y = b.y;
                    while x + dx < height && y >= dy {
                        x += dx;
                        y -= dy;

                        antinodes.push(Position::new(x, y));
                    }
                }
            }
        }

        antinodes
    }

    fn count_unique_antinode_pos(&self) -> usize {
        self.antinodes.iter().collect::<HashSet<&Position>>().len()
    }

    fn count_unique_real_antinode_pos(&self) -> usize {
        self.real_antinodes
            .iter()
            .collect::<HashSet<&Position>>()
            .len()
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
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day08.txt")
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let map = Map::from(test_input.as_slice());
        let res = map.count_unique_antinode_pos();

        assert_eq!(res, 14);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let map = Map::from(puzzle_input.as_slice());
        let res = map.count_unique_antinode_pos();

        assert_eq!(res, 247);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let map = Map::from(test_input.as_slice());
        println!("{}", map);
        let res = map.count_unique_real_antinode_pos();

        assert_eq!(res, 34);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let map = Map::from(puzzle_input.as_slice());
        println!("{}", map);
        let res = map.count_unique_real_antinode_pos();

        assert_eq!(res, 861);
    }
}
