use std::collections::HashMap;
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
    let p2 = get_most_bananas(&input, 2000);

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

#[tracing::instrument(skip_all)]
fn get_most_bananas(input: &[u64], iterations: u32) -> u64 {
    let price_lists = get_price_lists(input, iterations);
    let sequences = get_sequences(&price_lists);

    let mut totals: HashMap<&Vec<i8>, u64> = HashMap::new();

    for seq in &sequences {
        for (s, v) in seq {
            *totals.entry(s).or_insert(0) += *v as u64;
        }
    }

    *totals.values().max().unwrap()
}

fn get_price_lists(input: &[u64], iterations: u32) -> Vec<Vec<u8>> {
    let mut price_lists = Vec::with_capacity(input.len());

    for val in input {
        let mut list = Vec::with_capacity(iterations as usize + 1);

        let mut s = *val;

        list.push((s % 10) as u8);

        for _ in 0..iterations {
            s = get_next_secret_number(s, 1);
            list.push((s % 10) as u8);
        }

        price_lists.push(list)
    }

    price_lists
}

fn get_sequences(price_lists: &Vec<Vec<u8>>) -> Vec<HashMap<Vec<i8>, u8>> {
    let mut sequences = Vec::with_capacity(price_lists.len());

    for pl in price_lists {
        let mut seq_for_prices: HashMap<Vec<i8>, u8> = HashMap::new();

        for w in pl.windows(5) {
            let s1 = w[1] as i8 - w[0] as i8;
            let s2 = w[2] as i8 - w[1] as i8;
            let s3 = w[3] as i8 - w[2] as i8;
            let s4 = w[4] as i8 - w[3] as i8;

            let seq = vec![s1, s2, s3, s4];
            let price = w[4];

            if let Some(&p) = seq_for_prices.get(&seq) {
                if price > p {
                    seq_for_prices.insert(seq, p);
                }
            } else {
                seq_for_prices.insert(seq, price);
            }
        }

        sequences.push(seq_for_prices)
    }

    sequences
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn puzzle_input() -> Vec<u64> {
        get_input("day22.txt")
            .iter()
            .map(|i| i.parse::<u64>().unwrap())
            .collect()
    }

    #[rstest]
    fn test_p1() {
        let input = vec![1, 10, 100, 2024];
        let res = get_sum_of_secret_numbers(&input, 2000);

        assert_eq!(res, 37327623);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<u64>) {
        let res = get_sum_of_secret_numbers(&puzzle_input, 2000);

        assert_eq!(res, 20071921341);
    }

    #[rstest]
    fn test_p2() {
        let input = vec![1, 2, 3, 2024];
        let res = get_most_bananas(&input, 2000);

        assert_eq!(res, 23);
    }

    #[rstest]
    #[ignore]
    fn test_p2_full_input(puzzle_input: Vec<u64>) {
        let res = get_most_bananas(&puzzle_input, 2000);

        assert_eq!(res, 2242);
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
