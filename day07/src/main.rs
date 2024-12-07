use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input, tracing_init};
use itertools::Itertools;

fn main() {
    tracing_init();

    let input = get_input("day07.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[tracing::instrument(skip_all)]
fn solve(input: &[String]) -> (impl Display, impl Display) {
    let eqs = parse_equations(input);
    let p1 = get_total_calibration_result(&eqs);
    let p2 = get_total_calibration_result_with_concat(&eqs);

    (p1, p2)
}

struct Equation {
    target: u64,
    values: Vec<u64>,
}

impl Equation {
    fn is_solvable(&self) -> bool {
        let max = self.values.iter().product::<u64>();
        if max < self.target {
            return false;
        }

        // Skip 1s because they're multiplicative identity
        let min: u64 = self.values.iter().filter(|&&v| v != 1).sum();
        if min > self.target {
            return false;
        }

        let n = 2u64.pow(self.values.len() as u32 - 1);
        for ops in 0..n {
            let total =
                self.values
                    .iter()
                    .skip(1)
                    .enumerate()
                    .fold(self.values[0], |acc, (idx, v)| {
                        if ops & (1 << idx) != 0 {
                            acc * v
                        } else {
                            acc + v
                        }
                    });
            if total == self.target {
                return true;
            }
        }

        false
    }

    fn is_solvable_with_concat(&self) -> bool {
        let n = 3u64.pow(self.values.len() as u32 - 1);
        for ops in 0..n {
            let total =
                self.values
                    .iter()
                    .skip(1)
                    .enumerate()
                    .fold(self.values[0], |acc, (idx, v)| {
                        match (ops / 3u64.pow(idx as u32)) % 3 {
                            0 => acc + v,
                            1 => acc * v,
                            2 => {
                                let p = (*v as f32).log10().floor() as u32;
                                acc * 10u64.pow(p + 1) + v
                            }
                            _ => panic!("panik"),
                        }
                    });
            if total == self.target {
                return true;
            }
        }

        false
    }
}

impl TryFrom<&String> for Equation {
    type Error = String;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let (target, values) = value.split(':').collect_tuple().unwrap();

        let target = match target.parse::<u64>() {
            Ok(t) => t,
            Err(e) => return Err(format!("unable to parse target: {}: {:?}", target, e)),
        };

        let values = match values
            .split(' ')
            .filter_map(|v| {
                let v = v.trim();
                if v.is_empty() {
                    return None;
                }
                Some(v.parse::<u64>())
            })
            .collect()
        {
            Ok(v) => v,
            Err(e) => return Err(format!("unable to parse values: {}: {:?}", values, e)),
        };

        Ok(Equation { target, values })
    }
}

#[tracing::instrument(skip_all)]
fn parse_equations(input: &[String]) -> Vec<Equation> {
    input
        .iter()
        .map(|i| Equation::try_from(i).unwrap())
        .collect()
}

#[tracing::instrument(skip_all)]
fn get_total_calibration_result(eqs: &[Equation]) -> u64 {
    eqs.iter()
        .filter(|e| e.is_solvable())
        .map(|e| e.target)
        .sum()
}

#[tracing::instrument(skip_all)]
fn get_total_calibration_result_with_concat(eqs: &[Equation]) -> u64 {
    eqs.iter()
        .filter(|e| e.is_solvable_with_concat())
        .map(|e| e.target)
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
            190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day07.txt")
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let eqs = parse_equations(&test_input);
        let res = get_total_calibration_result(&eqs);

        assert_eq!(res, 3749);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let eqs = parse_equations(&puzzle_input);
        let res = get_total_calibration_result(&eqs);

        assert_eq!(res, 12940396350192);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let eqs = parse_equations(&test_input);
        let res = get_total_calibration_result_with_concat(&eqs);

        assert_eq!(res, 11387);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let eqs = parse_equations(&puzzle_input);
        let res = get_total_calibration_result_with_concat(&eqs);

        assert_eq!(res, 106016735664498);
    }

    #[rstest]
    fn test_eq_is_solvable() {
        let eq = Equation {
            target: 292,
            values: vec![11, 6, 16, 20],
        };

        assert!(eq.is_solvable());
    }

    #[rstest]
    fn test_eq_is_solvable_with_concat() {
        let eq = Equation {
            target: 7290,
            values: vec![6, 8, 6, 15],
        };

        assert!(eq.is_solvable_with_concat());
    }
}
