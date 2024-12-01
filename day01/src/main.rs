use std::collections::HashMap;
use std::fmt::Display;
use std::iter::zip;
use std::time::Instant;

use aoc_common::{format_duration, get_input, tracing_init};
use itertools::{sorted, Itertools};

fn main() {
    tracing_init();

    let input = get_input("day01.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[tracing::instrument(skip_all)]
fn solve(input: &[String]) -> (impl Display, impl Display) {
    let (v1, v2) = parse_list_values(input);
    let p1 = get_sum_distances(&v1, &v2);
    let p2 = get_similarity_score(&v1, &v2);

    (p1, p2)
}

#[tracing::instrument(skip_all)]
fn parse_list_values(input: &[String]) -> (Vec<i32>, Vec<i32>) {
    let mut values_a = Vec::new();
    let mut values_b = Vec::new();

    for (v1, v2) in input.iter().map(|e| {
        e.split(' ')
            .filter(|v| !v.is_empty())
            .map(|v| v.parse::<i32>().unwrap())
            .collect_tuple::<(i32, i32)>()
            .unwrap()
    }) {
        values_a.push(v1);
        values_b.push(v2);
    }

    (values_a, values_b)
}

#[tracing::instrument(skip_all)]
fn get_sum_distances(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    let a = sorted(a).collect_vec();
    let b = sorted(b).collect_vec();

    zip(a, b).map(|(v1, v2)| (v1 - v2).abs()).sum()
}

#[tracing::instrument(skip_all)]
fn get_similarity_score(a: &[i32], b: &[i32]) -> usize {
    let mut counts: HashMap<i32, usize> = HashMap::new();

    for v in b {
        *counts.entry(*v).or_default() += 1;
    }

    a.iter()
        .map(|v| (*v as usize) * counts.get(v).unwrap_or(&0))
        .sum()
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
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day01.txt")
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let (v1, v2) = parse_list_values(&test_input);
        let res = get_sum_distances(&v1, &v2);

        assert_eq!(res, 11);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let (v1, v2) = parse_list_values(&puzzle_input);
        let res = get_sum_distances(&v1, &v2);

        assert_eq!(res, 2192892);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let (v1, v2) = parse_list_values(&test_input);
        let res = get_similarity_score(&v1, &v2);

        assert_eq!(res, 31);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let (v1, v2) = parse_list_values(&puzzle_input);
        let res = get_similarity_score(&v1, &v2);

        assert_eq!(res, 22962826);
    }
}
