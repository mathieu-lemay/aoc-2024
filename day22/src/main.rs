use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input, tracing_init};

fn main() {
    tracing_init();

    let input = get_input("day22.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[tracing::instrument(skip_all)]
fn solve(input: &[String]) -> (impl Display, impl Display) {
    let input: Vec<u64> = input.iter().map(|i| i.parse::<u64>().unwrap()).collect();

    let p1 = get_sum_of_secret_numbers(&input, 2000);
    let p2 = 0;

    (p1, p2)
}

const MODULO: u64 = 16777215;

fn get_next_secret_number(mut s: u64, iterations: u32) -> u64 {
    for _ in 0..iterations {
        s ^= s << 6;
        s &= MODULO;
        s ^= s >> 5;
        s &= MODULO;
        s ^= s << 11;
        s &= MODULO;
    }

    s
}

#[tracing::instrument(skip_all)]
fn get_sum_of_secret_numbers(input: &[u64], iterations: u32) -> u64 {
    let mut sum = 0;

    for s in input {
        sum += get_next_secret_number(*s, iterations);
    }

    sum
}

#[cfg(test)]
mod tests {
    use aoc_common::parse_test_input;
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn test_input() -> Vec<u64> {
        parse_test_input(
            "
            1
            10
            100
            2024
        ",
        )
        .iter()
        .map(|i| i.parse::<u64>().unwrap())
        .collect()
    }

    #[fixture]
    fn puzzle_input() -> Vec<u64> {
        get_input("day22.txt")
            .iter()
            .map(|i| i.parse::<u64>().unwrap())
            .collect()
    }

    #[rstest]
    fn test_p1(test_input: Vec<u64>) {
        let res = get_sum_of_secret_numbers(&test_input, 2000);

        assert_eq!(res, 37327623);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<u64>) {
        let res = get_sum_of_secret_numbers(&puzzle_input, 2000);

        assert_eq!(res, 20071921341);
    }

    #[rstest]
    fn test_p2(test_input: Vec<u64>) {
        let res = 0;

        assert_eq!(res, 1);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<u64>) {
        let res = 0;

        assert_eq!(res, 1);
    }

    #[rstest]
    fn test_get_next_secret_number() {
        let mut n = 123;

        n = get_next_secret_number(n, 1);
        assert_eq!(n, 15887950);
        n = get_next_secret_number(n, 1);
        assert_eq!(n, 16495136);
        n = get_next_secret_number(n, 1);
        assert_eq!(n, 527345);
        n = get_next_secret_number(n, 1);
        assert_eq!(n, 704524);
        n = get_next_secret_number(n, 1);
        assert_eq!(n, 1553684);
        n = get_next_secret_number(n, 1);
        assert_eq!(n, 12683156);
        n = get_next_secret_number(n, 1);
        assert_eq!(n, 11100544);
        n = get_next_secret_number(n, 1);
        assert_eq!(n, 12249484);
        n = get_next_secret_number(n, 1);
        assert_eq!(n, 7753432);
        n = get_next_secret_number(n, 1);
        assert_eq!(n, 5908254);
    }
}
