use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input_as_string, tracing_init};

fn main() {
    tracing_init();

    let input = get_input_as_string("day11.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_str());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[tracing::instrument(skip_all)]
fn solve(input: &str) -> (impl Display, impl Display) {
    let p1 = 0;
    let p2 = 0;

    (p1, p2)
}

fn parse_stones(input: &str) -> Vec<u64> {
    input.split(' ').map(|i| i.parse::<u64>().unwrap()).collect()
}

fn get_result(stones: &[u64], n: i32) -> Vec<u64> {
    for _ in 0..n {

    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn test_input() -> String {
        String::from("125 17")
    }

    #[fixture]
    fn puzzle_input() -> String {
        get_input_as_string("day11.txt")
    }

    #[rstest]
    fn test_p1(test_input: String) {
        let stones = parse_stones(&test_input);
        let stones = blink(&stones, 6);

        assert_eq!(res, 1);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: String) {
        let res = 0;

        assert_eq!(res, 1);
    }

    #[rstest]
    fn test_p2(test_input: String) {
        let res = 0;

        assert_eq!(res, 1);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: String) {
        let res = 0;

        assert_eq!(res, 1);
    }
}
