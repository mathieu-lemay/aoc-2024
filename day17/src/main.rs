use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input, tracing_init};
use itertools::Itertools;
use tracing::debug;

fn main() {
    tracing_init();

    let input = get_input("day17.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[tracing::instrument(skip_all)]
fn solve(input: &[String]) -> (impl Display, impl Display) {
    let mut m = parse_machine(input);

    let p1 = m.get_output();
    let p2 = 0;

    (p1, p2)
}

#[tracing::instrument(skip_all)]
fn parse_machine(input: &[String]) -> Machine {
    let a = input[0].split(": ").last().unwrap().parse::<u64>().unwrap();
    let b = input[1].split(": ").last().unwrap().parse::<u64>().unwrap();
    let c = input[2].split(": ").last().unwrap().parse::<u64>().unwrap();

    let program = input[4]
        .split(": ")
        .last()
        .unwrap()
        .split(',')
        .map(|i| i.parse::<u8>().unwrap())
        .collect();

    Machine {
        a,
        b,
        c,
        program,
        pc: 0,
        output: Vec::new(),
    }
}

#[derive(Default, Debug)]
struct Machine {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u8>,
    pc: usize,

    output: Vec<u8>,
}

impl Machine {
    fn run(&mut self) {
        loop {
            if self.pc >= self.program.len() {
                break;
            }

            let instr = self.program[self.pc];
            let op = self.program[self.pc + 1];

            self.pc += 2;

            match instr {
                // ADV
                0 => {
                    let d = 2u64.pow(self.get_combo_op_value(op));

                    debug!(
                        "a = a / 2.pow({}) (a = {} / {})",
                        self.get_combo_op_as_str(op),
                        self.a,
                        d
                    );
                    self.a /= d;
                }
                // BXL
                1 => {
                    debug!("b = b ^ {} (b = {} ^ 3)", op, self.b);
                    self.b ^= op as u64;
                }
                // BST
                2 => {
                    debug!(
                        "b = {} % 8 (b = {} % 8)",
                        self.get_combo_op_as_str(op),
                        self.get_combo_op_value(op)
                    );
                    self.b = (self.get_combo_op_value(op) % 8) as u64;
                }
                // JNZ
                3 => {
                    debug!("jnz (a = {})", self.a);
                    if self.a != 0 {
                        self.pc = op as usize;
                    }
                }
                // BXC
                4 => {
                    debug!("b = b ^ c (b = {} ^ {})", self.b, self.c);
                    self.b ^= self.c;
                }
                // OUT
                5 => {
                    let v = match op {
                        0..=3 => format!("{}", op),
                        4 => "a".to_owned(),
                        5 => "b".to_owned(),
                        6 => "c".to_owned(),
                        _ => panic!(""),
                    };
                    debug!("print {} ({})", v, self.get_combo_op_value(op) % 8);
                    self.output.push((self.get_combo_op_value(op) % 8) as u8);
                }
                // BDV
                6 => {
                    let d = 2u64.pow(self.get_combo_op_value(op));

                    debug!(
                        "b = a / 2.pow({}) (a = {} / {})",
                        self.get_combo_op_as_str(op),
                        self.a,
                        d
                    );
                    self.b = self.a / d;
                }
                // CDV
                7 => {
                    let d = 2u64.pow(self.get_combo_op_value(op));

                    debug!(
                        "c = a / 2.pow({}) (a = {} / {})",
                        self.get_combo_op_as_str(op),
                        self.a,
                        d
                    );
                    self.c = self.a / d;
                }
                i => panic!("Invalid instruction: {}", i),
            }
            debug!("a={}, b={}, c={}", self.a, self.b, self.c);
        }
    }

    #[tracing::instrument(skip_all)]
    fn get_output(&mut self) -> String {
        self.run();

        self.output.iter().map(|i| i.to_string()).join(",")
    }

    fn get_combo_op_value(&self, op: u8) -> u32 {
        match op {
            0..=3 => op as u32,
            4 => self.a as u32,
            5 => self.b as u32,
            6 => self.c as u32,
            c => panic!("Invalid combo operand: {}", c),
        }
    }

    fn get_combo_op_as_str(&self, op: u8) -> String {
        match op {
            0..=3 => format!("{}", op),
            4 => "a".to_owned(),
            5 => "b".to_owned(),
            6 => "c".to_owned(),
            _ => panic!(""),
        }
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
            Register A: 729
            Register B: 0
            Register C: 0

            Program: 0,1,5,4,3,0
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day17.txt")
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let mut m = parse_machine(&test_input);
        let res = m.get_output();

        assert_eq!(res, "4,6,3,5,6,3,5,2,1,0");
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let mut m = parse_machine(&puzzle_input);
        let res = m.get_output();

        assert_eq!(res, "6,0,6,3,0,2,3,1,6");
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

    #[test]
    fn test_instructions_1() {
        let mut m = Machine {
            c: 9,
            program: vec![2, 6],
            ..Default::default()
        };

        m.run();

        assert_eq!(m.b, 1);
    }

    #[test]
    fn test_instructions_2() {
        let mut m = Machine {
            a: 10,
            program: vec![5, 0, 5, 1, 5, 4],
            ..Default::default()
        };

        m.run();

        assert_eq!(m.output, vec![0, 1, 2]);
    }

    #[test]
    fn test_instructions_3() {
        let mut m = Machine {
            a: 2024,
            program: vec![0, 1, 5, 4, 3, 0],
            ..Default::default()
        };

        m.run();

        assert_eq!(m.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(m.a, 0);
    }

    #[test]
    fn test_instructions_4() {
        let mut m = Machine {
            b: 29,
            program: vec![1, 7],
            ..Default::default()
        };

        m.run();

        assert_eq!(m.b, 26);
    }

    #[test]
    fn test_instructions_5() {
        let mut m = Machine {
            b: 2024,
            c: 43690,
            program: vec![4, 0],
            ..Default::default()
        };

        m.run();

        assert_eq!(m.b, 44354);
    }
}
