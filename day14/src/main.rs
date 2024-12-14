use core::f32;
use std::cmp::Ordering;
use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input, tracing_init, Point};
use regex::Regex;

fn main() {
    tracing_init();

    let input = get_input("day14.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[tracing::instrument(skip_all)]
fn solve(input: &[String]) -> (impl Display, impl Display) {
    let mut map = parse_map(input, 101, 103);

    let p1 = get_factor_after_ticks(map.clone(), 100);
    let p2 = get_tick_least_deviation(&mut map);

    (p1, p2)
}

#[tracing::instrument(skip_all)]
fn get_factor_after_ticks(mut map: Map, ticks: i32) -> u32 {
    map.tick(ticks);
    map.get_safety_factor()
}

#[tracing::instrument(skip_all)]
fn get_tick_least_deviation(map: &mut Map) -> i32 {
    let mut min_deviation: f32 = f32::MAX;
    let mut tick = 0;

    let max_ticks = map.width * map.height;

    for t in 0..max_ticks {
        map.tick(1);

        let positions: Vec<f32> = map.robots.iter().map(Robot::dist).collect();

        if let Some(dev) = std_deviation(&positions) {
            if dev < min_deviation {
                min_deviation = dev;
                tick = t + 1;
            }
        }
    }

    tick
}

fn mean(data: &[f32]) -> Option<f32> {
    let sum = data.iter().sum::<f32>();
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

fn std_deviation(data: &[f32]) -> Option<f32> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean - (*value);

                    diff * diff
                })
                .sum::<f32>()
                / count as f32;

            Some(variance.sqrt())
        }
        _ => None,
    }
}

type Position = Point<i32>;
type Velocity = Point<i32>;

#[derive(Clone)]
struct Map {
    width: i32,
    height: i32,

    robots: Vec<Robot>,
}

impl Map {
    fn tick(&mut self, n: i32) {
        for r in self.robots.iter_mut() {
            r.position.x = (r.position.x + r.velocity.x * n).rem_euclid(self.width);
            r.position.y = (r.position.y + r.velocity.y * n).rem_euclid(self.height);
        }
    }

    fn get_safety_factor(&self) -> u32 {
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut d = 0;

        let mx = self.width / 2;
        let my = self.height / 2;

        for r in &self.robots {
            match (r.position.x.cmp(&mx), r.position.y.cmp(&my)) {
                (Ordering::Less, Ordering::Less) => a += 1,
                (Ordering::Greater, Ordering::Less) => b += 1,
                (Ordering::Less, Ordering::Greater) => c += 1,
                (Ordering::Greater, Ordering::Greater) => d += 1,
                _ => {}
            }
        }

        a * b * c * d
    }
}

const NUM_CHARS: &[char] = &[
    '.', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self
                    .robots
                    .iter()
                    .filter(|r| r.position.x == x && r.position.y == y)
                    .count();
                write!(f, "{}", NUM_CHARS[c])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

impl Robot {
    fn dist(&self) -> f32 {
        ((self.position.x.pow(2) + self.position.y.pow(2)) as f32).sqrt()
    }
}

#[tracing::instrument(skip_all)]
fn parse_map(input: &[String], width: i32, height: i32) -> Map {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").expect("Invalid regex");

    let robots = input
        .iter()
        .map(|i| {
            let m = re
                .captures(i)
                .unwrap_or_else(|| panic!("Unable to parse {}", i));
            let px = m.get(1).unwrap().as_str().parse().unwrap();
            let py = m.get(2).unwrap().as_str().parse().unwrap();
            let vx = m.get(3).unwrap().as_str().parse().unwrap();
            let vy = m.get(4).unwrap().as_str().parse().unwrap();

            Robot {
                position: Position::new(px, py),
                velocity: Velocity::new(vx, vy),
            }
        })
        .collect::<Vec<Robot>>();

    Map {
        width,
        height,
        robots,
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
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day14.txt")
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let mut map = parse_map(&test_input, 11, 7);
        map.tick(100);

        let res = map.get_safety_factor();

        assert_eq!(res, 12);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let mut map = parse_map(&puzzle_input, 101, 103);
        map.tick(100);

        let res = map.get_safety_factor();

        assert_eq!(res, 211773366);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let mut map = parse_map(&puzzle_input, 101, 103);

        let res = get_tick_least_deviation(&mut map);

        assert_eq!(res, 7344);
    }
}
