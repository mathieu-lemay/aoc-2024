use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input, tracing_init};
use itertools::{izip, Itertools};

fn main() {
    tracing_init();

    let input = get_input("day04.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[tracing::instrument(skip_all)]
fn solve(input: &[String]) -> (impl Display, impl Display) {
    let chars = to_char_vec(input);
    let p1 = count_xmases(&chars);
    let p2 = count_x_mases(&chars);

    (p1, p2)
}

#[tracing::instrument(skip_all)]
fn to_char_vec(input: &[String]) -> Vec<Vec<char>> {
    input.iter().map(|i| i.chars().collect_vec()).collect_vec()
}

#[tracing::instrument(skip_all)]
fn count_xmases(input: &[Vec<char>]) -> u32 {
    let mut count = 0;

    // Horizontal
    for line in input {
        for w in line.windows(4) {
            match *w {
                ['X', 'M', 'A', 'S'] => count += 1,
                ['S', 'A', 'M', 'X'] => count += 1,
                _ => {}
            }
        }
    }

    for w in input.windows(4) {
        // Vertical
        for x in izip!(&w[0], &w[1], &w[2], &w[3]) {
            match x {
                ('X', 'M', 'A', 'S') => count += 1,
                ('S', 'A', 'M', 'X') => count += 1,
                _ => {}
            }
        }

        // Diagonals
        for x in izip!(&w[0], &w[1][1..], &w[2][2..], &w[3][3..]) {
            match x {
                ('X', 'M', 'A', 'S') => count += 1,
                ('S', 'A', 'M', 'X') => count += 1,
                _ => {}
            }
        }
        for x in izip!(&w[0][3..], &w[1][2..], &w[2][1..], &w[3]) {
            match x {
                ('X', 'M', 'A', 'S') => count += 1,
                ('S', 'A', 'M', 'X') => count += 1,
                _ => {}
            }
        }
    }

    count
}

#[tracing::instrument(skip_all)]
fn count_x_mases(input: &[Vec<char>]) -> u32 {
    let mut count = 0;

    for w in input.windows(3) {
        for (l1, l2, l3) in izip!(w[0].windows(3), w[1].windows(3), w[2].windows(3)) {
            match (l1, l2, l3) {
                (&['M', _, 'S'], &[_, 'A', _], &['M', _, 'S']) => count += 1,
                (&['S', _, 'M'], &[_, 'A', _], &['S', _, 'M']) => count += 1,
                (&['M', _, 'M'], &[_, 'A', _], &['S', _, 'S']) => count += 1,
                (&['S', _, 'S'], &[_, 'A', _], &['M', _, 'M']) => count += 1,
                _ => {}
            }
        }
    }

    count
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
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day04.txt")
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let chars = to_char_vec(&test_input);
        let res = count_xmases(&chars);

        assert_eq!(res, 18);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let chars = to_char_vec(&puzzle_input);
        let res = count_xmases(&chars);

        assert_eq!(res, 2500);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let chars = to_char_vec(&test_input);
        let res = count_x_mases(&chars);

        assert_eq!(res, 9);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let chars = to_char_vec(&puzzle_input);
        let res = count_x_mases(&chars);

        assert_eq!(res, 1933);
    }
}
