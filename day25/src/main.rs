use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input, tracing_init};
use itertools::Itertools;

fn main() {
    tracing_init();

    let input = get_input("day25.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[tracing::instrument(skip_all)]
fn solve(input: &[String]) -> (impl Display, impl Display) {
    let schematics = Schematics::from(input);
    let p1 = schematics.get_arrangements();
    let p2 = 0;

    (p1, p2)
}

#[derive(Debug)]
struct Schematics {
    locks: Vec<Vec<u8>>,
    keys: Vec<Vec<u8>>,
}

impl Schematics {
    #[tracing::instrument(skip_all)]
    fn get_arrangements(&self) -> usize {
        self.keys
            .iter()
            .cartesian_product(&self.locks)
            .filter(|(k, l)| k.iter().zip(l.iter()).all(|(a, b)| a + b <= 5))
            .count()
    }
}

impl From<&[String]> for Schematics {
    #[tracing::instrument(skip_all)]
    fn from(values: &[String]) -> Self {
        let mut locks = Vec::new();
        let mut keys = Vec::new();

        for chunk in values.split(|s| s.is_empty()) {
            let heights: Vec<u8> = (0usize..5)
                .map(|i| {
                    (chunk
                        .iter()
                        .filter_map(|s| {
                            if let Some("#") = s.get(i..=i) {
                                Some(())
                            } else {
                                None
                            }
                        })
                        .count()
                        - 1) as u8
                })
                .collect();

            if chunk[0] == "#####" {
                locks.push(heights);
            } else {
                keys.push(heights);
            }
        }

        Self { locks, keys }
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
            #####
            .####
            .####
            .####
            .#.#.
            .#...
            .....

            #####
            ##.##
            .#.##
            ...##
            ...#.
            ...#.
            .....

            .....
            #....
            #....
            #...#
            #.#.#
            #.###
            #####

            .....
            .....
            #.#..
            ###..
            ###.#
            ###.#
            #####

            .....
            .....
            .....
            #....
            #.#..
            #.#.#
            #####
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day25.txt")
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let schematics = Schematics::from(test_input.as_slice());
        let res = schematics.get_arrangements();

        assert_eq!(res, 3);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let schematics = Schematics::from(puzzle_input.as_slice());
        let res = schematics.get_arrangements();

        assert_eq!(res, 3307);
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
