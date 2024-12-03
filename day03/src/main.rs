use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input_as_string, tracing_init};
use regex::Regex;

fn main() {
    tracing_init();

    let input = get_input_as_string("day03.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[tracing::instrument(skip_all)]
fn solve(input: &str) -> (impl Display, impl Display) {
    add_all_multiplications(input)
}

#[tracing::instrument(skip_all)]
fn add_all_multiplications(input: &str) -> (u32, u32) {
    let mul_re =
        Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don't\(\)").expect("Invalid regex");

    let mut sum = 0;
    let mut partial_sum = 0;

    let mut enabled = true;

    for m in mul_re.captures_iter(input) {
        match m.get(0).unwrap().as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                let a = m.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let b = m.get(2).unwrap().as_str().parse::<u32>().unwrap();

                let v = a * b;

                sum += v;
                if enabled {
                    partial_sum += v
                }
            }
        }
    }

    (sum, partial_sum)
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn puzzle_input() -> String {
        get_input_as_string("day03.txt")
    }

    #[rstest]
    fn test_p1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let (res, _) = add_all_multiplications(input);

        assert_eq!(res, 161);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: String) {
        let (res, _) = add_all_multiplications(&puzzle_input);

        assert_eq!(res, 174103751);
    }

    #[rstest]
    fn test_p2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let (_, res) = add_all_multiplications(input);

        assert_eq!(res, 48);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: String) {
        let (_, res) = add_all_multiplications(&puzzle_input);

        assert_eq!(res, 100411201);
    }
}
