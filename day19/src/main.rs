use std::collections::VecDeque;
use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input, tracing_init};
use tracing::debug;

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

    puzzle.count_possible_builds()
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
    fn count_possible_builds(&self) -> (usize, usize) {
        self.designs
            .iter()
            .enumerate()
            .filter_map(|(idx, d)| {
                debug!(
                    "Checking design {} ({} of {})",
                    d,
                    idx + 1,
                    self.designs.len()
                );

                let n = self.count_builds_for_design(d);
                if n > 0 {
                    Some(n)
                } else {
                    None
                }
            })
            .fold((0, 0), |(n, t), c| (n + 1, t + c))
    }

    fn count_builds_for_design(&self, design: &str) -> usize {
        let mut count = 0;
        debug!("Testing for design: {}", design);

        let mut candidates = VecDeque::new();
        candidates.push_back((String::new(), 1));

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
            let (c, n) = candidates.pop_front().unwrap();

            for p in &useful_patterns {
                let d = format!("{}{}", c, p);
                if d == design {
                    count += n;
                } else if design.starts_with(&d) {
                    if let Some(idx) = candidates.iter().position(|(c, _)| c == &d) {
                        let (d, n2) = candidates.remove(idx).unwrap();
                        candidates.push_back((d, n2 + n))
                    } else {
                        candidates.push_back((d, n));
                    }
                }
            }
        }

        if count > 0 {
            debug!("{} can be made in {} ways\n", design, count);
        } else {
            debug!("{} can't be made\n", design);
        }

        count
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
        let (res, _) = puzzle.count_possible_builds();

        assert_eq!(res, 6);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let puzzle = parse_puzzle(&test_input);
        let (_, res) = puzzle.count_possible_builds();

        assert_eq!(res, 16);
    }

    #[rstest]
    #[ignore]
    fn test_full_input(puzzle_input: Vec<String>) {
        let puzzle = parse_puzzle(&puzzle_input);
        let (p1, p2) = puzzle.count_possible_builds();

        assert_eq!(p1, 290);
        assert_eq!(p2, 712058625427487);
    }

    #[rstest]
    #[case("brwrr", 2)]
    #[case("bggr", 1)]
    #[case("gbbr", 4)]
    #[case("rrbgbr", 6)]
    #[case("bwurrg", 1)]
    #[case("brgr", 2)]
    #[case("ubwu", 0)]
    #[case("bbrgwb", 0)]
    fn test_count_possibilities(#[case] design: &'static str, #[case] count: usize) {
        let puzzle = Puzzle {
            patterns: vec![
                "r".to_string(),
                "wr".to_string(),
                "b".to_string(),
                "g".to_string(),
                "bwu".to_string(),
                "rb".to_string(),
                "gb".to_string(),
                "br".to_string(),
            ],
            designs: Vec::new(),
        };

        assert_eq!(puzzle.count_builds_for_design(design), count);
    }
}
