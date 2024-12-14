use std::cmp::Ordering;
use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input, tracing_init, Point};
use draw::{render, Canvas, Color, Drawing, Style, SvgRenderer};
use itertools::Itertools;
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
    render_possibilities(&mut map, 10000);

    (p1, 0)
}

#[tracing::instrument(skip_all)]
fn get_factor_after_ticks(mut map: Map, ticks: u32) -> u32 {
    map.tick(ticks);
    map.get_safety_factor()
}

#[tracing::instrument(skip_all)]
fn render_possibilities(map: &mut Map, max_ticks: u32) {
    let mut nb_renders = 0;

    for t in 0..max_ticks {
        map.tick(1);

        let mut keep = false;
        let line_length: u8 = 10;
        for x in 0..map.width {
            let ys = map
                .robots
                .iter()
                .filter(|r| r.position.x == x)
                .map(|r| r.position.y)
                .sorted()
                .unique()
                .collect_vec();
            if ys
                .windows(line_length as usize + 1)
                .any(|w| w[line_length as usize] - w[0] == line_length)
            {
                keep = true;
                break;
            }
        }

        if !keep {
            continue;
        }

        nb_renders += 1;

        map.render(t + 1);
    }

    println!("Rendered {} possibilities, have fun.", nb_renders);
}

type Position = Point<u8>;
type Velocity = Point<i8>;

#[derive(Clone)]
struct Map {
    width: u8,
    height: u8,

    robots: Vec<Robot>,
}

impl Map {
    fn tick(&mut self, n: u32) {
        for r in self.robots.iter_mut() {
            let mut x = (r.position.x as i32 + r.velocity.x as i32 * n as i32) % self.width as i32;
            if x < 0 {
                x += self.width as i32;
            }
            let mut y = (r.position.y as i32 + r.velocity.y as i32 * n as i32) % self.height as i32;
            if y < 0 {
                y += self.height as i32;
            }

            r.position.x = x as u8;
            r.position.y = y as u8;
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

    fn render(&self, tick: u32) {
        // create a canvas to draw on
        let mut canvas = Canvas::new(self.width as u32, self.height as u32);
        canvas.display_list.add(
            Drawing::new()
                .with_shape(draw::Shape::Rectangle {
                    width: canvas.width,
                    height: canvas.height,
                })
                .with_style(Style::filled(Color::gray(255))),
        );

        for r in &self.robots {
            let d = Drawing::new()
                .with_shape(draw::Shape::Rectangle {
                    width: 1,
                    height: 1,
                })
                .with_xy(r.position.x as f32, r.position.y as f32)
                .with_style(Style::filled(Color::black()));
            canvas.display_list.add(d);
        }

        // save the canvas as an svg
        render::save(
            &canvas,
            &format!("day14/output/{:05}.svg", tick),
            SvgRenderer::new(),
        )
        .expect("Failed to save");
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

#[tracing::instrument(skip_all)]
fn parse_map(input: &[String], width: u8, height: u8) -> Map {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").expect("Invalid regex");

    let robots = input
        .iter()
        .map(|i| {
            let m = re
                .captures(i)
                .unwrap_or_else(|| panic!("Unable to parse {}", i));
            let px = m.get(1).unwrap().as_str().parse::<u8>().unwrap();
            let py = m.get(2).unwrap().as_str().parse::<u8>().unwrap();
            let vx = m.get(3).unwrap().as_str().parse::<i8>().unwrap();
            let vy = m.get(4).unwrap().as_str().parse::<i8>().unwrap();

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
}
