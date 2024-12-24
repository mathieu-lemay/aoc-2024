use std::collections::HashMap;
use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input, tracing_init};
use itertools::Itertools;
use tracing::debug;

fn main() {
    tracing_init();

    let input = get_input("day24.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[tracing::instrument(skip_all)]
fn solve(input: &[String]) -> (impl Display, impl Display) {
    let mut sys = parse_system(input);
    sys.solve();

    let p1 = sys.get_z_wires_value();
    let p2 = 0;

    (p1, p2)
}

#[tracing::instrument(skip_all)]
fn parse_system(input: &[String]) -> System {
    let (values, equations) = input.split(|s| s.is_empty()).collect_tuple().unwrap();

    let variables: HashMap<String, u8> = values
        .iter()
        .map(|v| {
            let (name, value) = v.split(": ").collect_tuple().unwrap();
            (name.to_owned(), value.parse::<u8>().unwrap())
        })
        .collect();

    let wires = equations
        .iter()
        .map(|v| {
            let (a, op, b, out) = v
                .replace("-> ", "")
                .split(" ")
                .map(|v| v.to_string())
                .collect_tuple()
                .unwrap();
            Wire { a, b, op, out }
        })
        .collect();

    System { variables, wires }
}

struct System {
    variables: HashMap<String, u8>,
    wires: Vec<Wire>,
}

#[derive(Debug)]
struct Wire {
    a: String,
    b: String,
    op: String,
    out: String,
}

impl System {
    #[tracing::instrument(skip_all)]
    fn solve(&mut self) {
        debug!("Variables: {:?}", self.variables);
        debug!("Wires: {:?}", self.wires);

        while !self.wires.is_empty() {
            let mut idx: Option<usize> = None;

            for (i, w) in self.wires.iter().enumerate() {
                if self.variables.contains_key(&w.a) && self.variables.contains_key(&w.b) {
                    let a = self.variables.get(&w.a).unwrap();
                    let b = self.variables.get(&w.b).unwrap();

                    debug!("Solving: {} {} {} -> {}", w.a, w.b, w.op, w.out);

                    let val = match w.op.as_str() {
                        "AND" => a & b,
                        "OR" => a | b,
                        "XOR" => a ^ b,
                        _ => panic!("Invalid op: {}", w.op),
                    };

                    self.variables.insert(w.out.clone(), val);
                    debug!("Variables: {:?}", self.variables);

                    idx = Some(i);
                    break;
                }
            }

            if let Some(i) = idx {
                self.wires.remove(i);
            } else {
                panic!("Found nothing to solve.");
            }
        }
    }

    #[tracing::instrument(skip_all)]
    fn get_z_wires_value(&self) -> u64 {
        self.variables
            .iter()
            .filter(|(k, _)| k.starts_with("z"))
            .sorted_by(|a, b| Ord::cmp(a.0, b.0).reverse())
            .fold(0u64, |a, v| (a << 1) | (*v.1 as u64))
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
            x00: 1
            x01: 0
            x02: 1
            x03: 1
            x04: 0
            y00: 1
            y01: 1
            y02: 1
            y03: 1
            y04: 1

            ntg XOR fgs -> mjb
            y02 OR x01 -> tnw
            kwq OR kpj -> z05
            x00 OR x03 -> fst
            tgd XOR rvg -> z01
            vdt OR tnw -> bfw
            bfw AND frj -> z10
            ffh OR nrd -> bqk
            y00 AND y03 -> djm
            y03 OR y00 -> psh
            bqk OR frj -> z08
            tnw OR fst -> frj
            gnj AND tgd -> z11
            bfw XOR mjb -> z00
            x03 OR x00 -> vdt
            gnj AND wpb -> z02
            x04 AND y00 -> kjc
            djm OR pbm -> qhw
            nrd AND vdt -> hwm
            kjc AND fst -> rvg
            y04 OR y02 -> fgs
            y01 AND x02 -> pbm
            ntg OR kjc -> kwq
            psh XOR fgs -> tgd
            qhw XOR tgd -> z09
            pbm OR djm -> kpj
            x03 XOR y03 -> ffh
            x00 XOR y04 -> ntg
            bfw OR bqk -> z06
            nrd XOR fgs -> wpb
            frj XOR qhw -> z04
            bqk OR frj -> z07
            y03 OR x01 -> nrd
            hwm AND bqk -> z03
            tgd XOR rvg -> z12
            tnw OR pbm -> gnj
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day24.txt")
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let mut sys = parse_system(&test_input);
        sys.solve();

        let res = sys.get_z_wires_value();

        assert_eq!(res, 2024);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let mut sys = parse_system(&puzzle_input);
        sys.solve();

        let res = sys.get_z_wires_value();

        assert_eq!(res, 57344080719736);
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
