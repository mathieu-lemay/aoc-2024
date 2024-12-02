use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input, tracing_init};
use itertools::Itertools;

fn main() {
    tracing_init();

    let input = get_input("day02.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[tracing::instrument(skip_all)]
fn solve(input: &[String]) -> (impl Display, impl Display) {
    let reports = parse_reports(input);
    let p1 = get_safe_reports(&reports);
    let p2 = get_safe_reports_with_tolerance(&reports);

    (p1, p2)
}

type Report = Vec<i32>;

#[tracing::instrument(skip_all)]
fn parse_reports(input: &[String]) -> Vec<Report> {
    input
        .iter()
        .map(|i| {
            i.split(' ')
                .map(|v| v.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

#[tracing::instrument(skip_all)]
fn get_safe_reports(reports: &[Report]) -> usize {
    reports.iter().filter(|&r| is_safe(r)).count()
}

#[tracing::instrument(skip_all)]
fn get_safe_reports_with_tolerance(reports: &[Report]) -> usize {
    let mut c = 0;

    for report in reports {
        if is_safe(report) {
            c += 1;
            continue;
        }

        for i in 0..report.len() {
            let mut r = report.clone();
            r.remove(i);

            if is_safe(&r) {
                c += 1;
                break;
            }
        }
    }

    c
}

#[inline]
fn get_deltas(r: &Report) -> Vec<i32> {
    r.windows(2).map(|w| w[1] - w[0]).collect_vec()
}

#[inline]
#[allow(clippy::manual_range_contains)]
fn is_safe(report: &Vec<i32>) -> bool {
    let deltas = get_deltas(report);

    deltas.iter().all(|&v| v > 0 && v <= 3) || deltas.iter().all(|&v| v < 0 && v >= -3)
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
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day02.txt")
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let reports = parse_reports(&test_input);
        let res = get_safe_reports(&reports);

        assert_eq!(res, 2);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let reports = parse_reports(&puzzle_input);
        let res = get_safe_reports(&reports);

        assert_eq!(res, 332);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let reports = parse_reports(&test_input);
        let res = get_safe_reports_with_tolerance(&reports);

        assert_eq!(res, 4);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let reports = parse_reports(&puzzle_input);
        let res = get_safe_reports_with_tolerance(&reports);

        assert_eq!(res, 398);
    }
}
