use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input_as_string, tracing_init};
use cached::proc_macro::cached;

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
    let stones = parse_stones(input);
    let p1 = get_number_of_stones(stones.clone(), 25);
    let p2 = get_number_of_stones(stones, 75);

    (p1, p2)
}

#[tracing::instrument(skip_all)]
fn parse_stones(input: &str) -> Vec<u64> {
    input
        .split(' ')
        .map(|i| i.parse::<u64>().unwrap())
        .collect()
}

#[tracing::instrument(skip_all)]
fn get_number_of_stones(stones: Vec<u64>, n: i32) -> usize {
    let mut count = 0;

    for s in stones {
        count += blink(s, n);
    }

    count
}

#[cached]
fn blink(s: u64, n: i32) -> usize {
    if n == 0 {
        return 1;
    }

    if s == 0 {
        return blink(1, n - 1);
    }

    let nb = (s as f64).log10().floor() as u32 + 1;
    if nb % 2 == 0 {
        let e = 10u64.pow(nb / 2);
        let a = s / e;
        let b = s % e;
        return blink(a, n - 1) + blink(b, n - 1);
    }

    return blink(s * 2024, n - 1);
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
        let res = get_number_of_stones(stones, 25);

        assert_eq!(res, 55312);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: String) {
        let stones = parse_stones(&puzzle_input);
        let res = get_number_of_stones(stones, 25);

        assert_eq!(res, 213625);
    }

    #[rstest]
    fn test_p2(test_input: String) {
        let stones = parse_stones(&test_input);
        let res = get_number_of_stones(stones, 75);

        assert_eq!(res, 65601038650482);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: String) {
        let stones = parse_stones(&puzzle_input);
        let res = get_number_of_stones(stones, 75);

        assert_eq!(res, 252442982856820);
    }
}
