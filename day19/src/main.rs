use std::collections::VecDeque;
use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input, tracing_init};

fn main() {
    tracing_init();

    let input = get_input("day19.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[tracing::instrument(skip_all)]
fn solve(input: &[String]) -> (impl Display, impl Display) {
    let puzzle = parse_puzzle(input);

    let p1 = puzzle.count_possible_designs();
    let p2 = 0;

    (p1, p2)
}

#[tracing::instrument(skip_all)]
fn parse_puzzle(input: &[String]) -> Puzzle {
    let patterns: Vec<String> = input[0].split(", ").map(String::from).collect();
    let designs: Vec<String> = input[2..].iter().map(String::from).collect();

    Puzzle { patterns, designs }
}

struct Puzzle {
    patterns: Vec<String>,
    designs: Vec<String>,
}

impl Puzzle {
    #[tracing::instrument(skip_all)]
    fn count_possible_designs(&self) -> usize {
        self.designs.iter().filter(|d| self.can_build(d)).count()
    }

    fn can_build(&self, design: &str) -> bool {
        let mut candidates = VecDeque::new();
        candidates.push_back(String::new());

        let useful_patterns: Vec<&str> = self
            .patterns
            .iter()
            .filter_map(|p| {
                if design.contains(p) {
                    Some(p.as_str())
                } else {
                    None
                }
            })
            .collect();

        while !candidates.is_empty() {
            let c = candidates.pop_front().unwrap();

            for p in &useful_patterns {
                let d = format!("{}{}", c, p);
                if d == design {
                    return true;
                }

                if design.starts_with(&d) && !candidates.contains(&d) {
                    candidates.push_back(d);
                }
            }
        }

        false
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
            r, wr, b, g, bwu, rb, gb, br

            brwrr
            bggr
            gbbr
            rrbgbr
            ubwu
            bwurrg
            brgr
            bbrgwb
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day19.txt")
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let puzzle = parse_puzzle(&test_input);
        let res = puzzle.count_possible_designs();

        assert_eq!(res, 6);
    }

    #[rstest]
    #[ignore]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let puzzle = parse_puzzle(&puzzle_input);
        let res = puzzle.count_possible_designs();

        assert_eq!(res, 290);
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
