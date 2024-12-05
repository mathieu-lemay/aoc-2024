use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input, tracing_init};
use itertools::Itertools;

fn main() {
    tracing_init();

    let input = get_input("day05.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[tracing::instrument(skip_all)]
fn solve(input: &[String]) -> (impl Display, impl Display) {
    let manual = parse_manual(input);

    let p1 = manual.get_hash_of_ordered_updates();
    let p2 = manual.get_hash_of_fixed_updates();

    (p1, p2)
}

struct PageOrdering {
    p1: u8,
    p2: u8,
}

struct PageList {
    pages: Vec<u8>,
}

impl PageList {
    fn is_ordered(&self, rules: &[PageOrdering]) -> bool {
        for rule in rules {
            let idx1 = self.pages.iter().position(|&p| p == rule.p1);
            let idx2 = self.pages.iter().position(|&p| p == rule.p2);

            if let (Some(idx1), Some(idx2)) = (idx1, idx2) {
                if idx1 > idx2 {
                    return false;
                }
            }
        }

        true
    }

    #[tracing::instrument(skip_all)]
    fn fix(&self, rules: &[PageOrdering]) -> Option<PageList> {
        if self.is_ordered(rules) {
            return None;
        }

        let mut pl = PageList {
            pages: self.pages.clone(),
        };

        let rules = rules
            .iter()
            .filter(|r| pl.pages.contains(&r.p1) || pl.pages.contains(&r.p2))
            .map(|r| PageOrdering { p1: r.p1, p2: r.p2 })
            .collect_vec();

        loop {
            for rule in &rules {
                let idx1 = pl.pages.iter().position(|&p| p == rule.p1);
                let idx2 = pl.pages.iter().position(|&p| p == rule.p2);

                if let (Some(idx1), Some(idx2)) = (idx1, idx2) {
                    if idx1 > idx2 {
                        let p1 = pl.pages[idx1];
                        let p2 = pl.pages[idx2];
                        pl.pages[idx1] = p2;
                        pl.pages[idx2] = p1;
                    }
                }
            }

            if pl.is_ordered(&rules) {
                return Some(pl);
            }
        }
    }

    fn get_middle_page(&self) -> u8 {
        self.pages[self.pages.len() / 2]
    }
}

struct SafetyManual {
    ordering_rules: Vec<PageOrdering>,
    pages_to_produce: Vec<PageList>,
}

impl SafetyManual {
    #[tracing::instrument(skip_all)]
    fn get_hash_of_ordered_updates(&self) -> u32 {
        let mut hash = 0;

        for pl in &self.pages_to_produce {
            if pl.is_ordered(&self.ordering_rules) {
                let m = pl.get_middle_page();

                hash += m as u32;
            }
        }

        hash
    }

    #[tracing::instrument(skip_all)]
    fn get_hash_of_fixed_updates(&self) -> u32 {
        let mut hash = 0;

        for pl in &self.pages_to_produce {
            if let Some(pl) = pl.fix(&self.ordering_rules) {
                let m = pl.get_middle_page();

                hash += m as u32;
            }
        }

        hash
    }
}

#[tracing::instrument(skip_all)]
fn parse_manual(input: &[String]) -> SafetyManual {
    let sep = input.iter().position(|s| s.is_empty()).unwrap();

    let mut ordering_rules = Vec::with_capacity(sep);
    let mut pages_to_produce = Vec::with_capacity(input.len() - sep - 1);

    for e in input.iter().take(sep) {
        let (p1, p2) = e
            .splitn(2, '|')
            .map(|p| p.parse::<u8>().unwrap())
            .collect_tuple()
            .unwrap();
        ordering_rules.push(PageOrdering { p1, p2 })
    }

    for e in input.iter().skip(sep + 1) {
        let pages = e.split(',').map(|p| p.parse::<u8>().unwrap()).collect_vec();
        pages_to_produce.push(PageList { pages })
    }

    SafetyManual {
        ordering_rules,
        pages_to_produce,
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
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day05.txt")
    }

    #[rstest]
    fn test_parse_manual(test_input: Vec<String>) {
        let manual = parse_manual(&test_input);

        assert_eq!(manual.ordering_rules.len(), 21);
        assert_eq!(manual.pages_to_produce.len(), 6);
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let manual = parse_manual(&test_input);
        let res = manual.get_hash_of_ordered_updates();

        assert_eq!(res, 143);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let manual = parse_manual(&puzzle_input);
        let res = manual.get_hash_of_ordered_updates();

        assert_eq!(res, 6034);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let manual = parse_manual(&test_input);
        let res = manual.get_hash_of_fixed_updates();

        assert_eq!(res, 123);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let manual = parse_manual(&puzzle_input);
        let res = manual.get_hash_of_fixed_updates();

        assert_eq!(res, 6305);
    }
}
