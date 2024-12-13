use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input, tracing_init, Point};
use mathru::algebra::linear::matrix::{General, Solve};
use mathru::algebra::linear::vector::Vector;
use mathru::vector;
use regex::Regex;

fn main() {
    tracing_init();

    let input = get_input("day13.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[tracing::instrument(skip_all)]
fn solve(input: &[String]) -> (impl Display, impl Display) {
    let machines = parse_machines(input);

    let p1 = get_fewest_tokens(&machines, 0);
    let p2 = get_fewest_tokens(&machines, 10000000000000);

    (p1, p2)
}

#[tracing::instrument(skip_all)]
fn parse_machines(input: &[String]) -> Vec<Machine> {
    let mut machines = Vec::new();

    let regex = Regex::new(r"[A-Za-z ]+: X[=\+](\d+), Y[=\+](\d+)").expect("Invalid regex");

    for blocks in input.split(|s| s.is_empty()) {
        let btn_a_m = regex.captures(&blocks[0]).expect("invalid btn_a");
        let btn_b_m = regex.captures(&blocks[1]).expect("invalid btn_b");
        let prize_m = regex.captures(&blocks[2]).expect("invalid prize");

        let m = Machine {
            btn_a: Button::new(
                btn_a_m.get(1).unwrap().as_str().parse::<u64>().unwrap(),
                btn_a_m.get(2).unwrap().as_str().parse::<u64>().unwrap(),
            ),
            btn_b: Button::new(
                btn_b_m.get(1).unwrap().as_str().parse::<u64>().unwrap(),
                btn_b_m.get(2).unwrap().as_str().parse::<u64>().unwrap(),
            ),
            prize: Prize::new(
                prize_m.get(1).unwrap().as_str().parse::<u64>().unwrap(),
                prize_m.get(2).unwrap().as_str().parse::<u64>().unwrap(),
            ),
        };

        machines.push(m);
    }

    machines
}

#[tracing::instrument(skip(machines))]
fn get_fewest_tokens(machines: &[Machine], offset: u64) -> u64 {
    machines
        .iter()
        .filter_map(|m| m.get_presses_to_prize(offset).map(|p| p.cost()))
        .sum()
}

type Button = Point<u64>;
type Prize = Point<u64>;

#[derive(Debug, PartialEq, Eq)]
struct Presses {
    btn_a: u64,
    btn_b: u64,
}

impl Presses {
    fn cost(&self) -> u64 {
        self.btn_a * 3 + self.btn_b
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Machine {
    btn_a: Button,
    btn_b: Button,
    prize: Prize,
}

impl Machine {
    fn get_presses_to_prize(&self, offset: u64) -> Option<Presses> {
        let a: General<f64> = General::new(
            2,
            2,
            vec![
                self.btn_a.x as f64,
                self.btn_a.y as f64,
                self.btn_b.x as f64,
                self.btn_b.y as f64,
            ],
        );
        let b: Vector<f64> =
            vector![(self.prize.x + offset) as f64; (self.prize.y + offset) as f64];

        let x = a.solve(&b).unwrap();

        let btn_a = x[0];
        if (btn_a * 1000.0).round() / 1000.0 != btn_a.round() {
            return None;
        }

        let btn_b = x[1];
        if (btn_b * 1000.0).round() / 1000.0 != btn_b.round() {
            return None;
        }

        Some(Presses {
            btn_a: btn_a.round() as u64,
            btn_b: btn_b.round() as u64,
        })
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
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400

            Button A: X+26, Y+66
            Button B: X+67, Y+21
            Prize: X=12748, Y=12176

            Button A: X+17, Y+86
            Button B: X+84, Y+37
            Prize: X=7870, Y=6450

            Button A: X+69, Y+23
            Button B: X+27, Y+71
            Prize: X=18641, Y=10279
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day13.txt")
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let machines = parse_machines(&test_input);
        let res = get_fewest_tokens(&machines, 0);

        assert_eq!(res, 480);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let machines = parse_machines(&puzzle_input);
        let res = get_fewest_tokens(&machines, 0);

        assert_eq!(res, 38839);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let machines = parse_machines(&test_input);
        let res = get_fewest_tokens(&machines, 10000000000000);

        assert_eq!(res, 875318608908);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let machines = parse_machines(&puzzle_input);
        let res = get_fewest_tokens(&machines, 10000000000000);

        assert_eq!(res, 75200131617108);
    }

    #[rstest]
    #[case(
        Machine{
            btn_a: Button::new(94, 34),
            btn_b: Button::new(22, 67),
            prize: Prize::new(8400, 5400),
        },
        Some(Presses{btn_a: 80, btn_b: 40}),
        Some(280)
    )]
    #[case(
        Machine{
            btn_a: Button::new(26, 66),
            btn_b: Button::new(67, 21),
            prize: Prize::new(12748, 12176),
        },
        None,
        None,
    )]
    #[case(
        Machine{
            btn_a: Button::new(17, 86),
            btn_b: Button::new(84, 37),
            prize: Prize::new(7870, 6450),
        },
        Some(Presses{btn_a: 38, btn_b: 86}),
        Some(200),
    )]
    #[case(
        Machine{
            btn_a: Button::new(69, 23),
            btn_b: Button::new(27, 71),
            prize: Prize::new(18641, 10279),
        },
        None,
        None,
    )]
    fn test_get_presses_to_prize(
        #[case] machine: Machine,
        #[case] expected: Option<Presses>,
        #[case] expected_cost: Option<u64>,
    ) {
        let res = machine.get_presses_to_prize(0);

        assert_eq!(res, expected);
        assert_eq!(res.map(|r| r.cost()), expected_cost);
    }

    #[rstest]
    #[case(
        Machine{
            btn_a: Button::new(94, 34),
            btn_b: Button::new(22, 67),
            prize: Prize::new(8400, 5400),
        },
        None,
        None,
    )]
    #[case(
        Machine{
            btn_a: Button::new(26, 66),
            btn_b: Button::new(67, 21),
            prize: Prize::new(12748, 12176),
        },
        Some(Presses{ btn_a: 118679050709, btn_b: 103199174542 }),
        Some(459236326669)
    )]
    #[case(
        Machine{
            btn_a: Button::new(17, 86),
            btn_b: Button::new(84, 37),
            prize: Prize::new(7870, 6450),
        },
        None,
        None,
    )]
    #[case(
        Machine{
            btn_a: Button::new(69, 23),
            btn_b: Button::new(27, 71),
            prize: Prize::new(18641, 10279),
        },
        Some(Presses{btn_a: 102851800151, btn_b: 107526881786}),
        Some(416082282239),
    )]
    fn test_get_presses_to_prize_with_offset(
        #[case] machine: Machine,
        #[case] expected: Option<Presses>,
        #[case] expected_cost: Option<u64>,
    ) {
        let res = machine.get_presses_to_prize(10000000000000);

        assert_eq!(res, expected);
        assert_eq!(res.map(|r| r.cost()), expected_cost);
    }
}
