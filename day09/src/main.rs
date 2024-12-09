use std::collections::VecDeque;
use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input_as_string, tracing_init};

fn main() {
    tracing_init();

    let input = get_input_as_string("day09.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_str());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[tracing::instrument(skip_all)]
fn solve(input: &str) -> (impl Display, impl Display) {
    let mut disk = Disk::from(input);
    disk.optimize();

    let p1 = disk.checksum();

    let disk = UnfragmentedDisk::from(input);
    let disk = disk.optimized();
    let p2 = disk.checksum();

    (p1, p2)
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum BlockType {
    Empty,
    File(u32),
}

struct Disk {
    blocks: Vec<BlockType>,
}

impl From<&str> for Disk {
    fn from(value: &str) -> Self {
        let mut blocks = Vec::with_capacity(value.len());

        for (idx, c) in value.chars().enumerate() {
            let size = c.to_digit(10).unwrap();

            let b = if idx % 2 == 0 {
                BlockType::File((idx / 2) as u32)
            } else {
                BlockType::Empty
            };

            for _ in 0..size {
                blocks.push(b.clone());
            }
        }

        Disk { blocks }
    }
}

impl Disk {
    #[tracing::instrument(skip_all)]
    fn optimize(&mut self) {
        let mut a = 0;
        let mut b = self.blocks.len() - 1;

        while a < b {
            while self.blocks[a] != BlockType::Empty {
                a += 1;
            }

            while self.blocks[b] == BlockType::Empty {
                b -= 1;
            }

            if a >= b {
                break;
            }

            self.blocks.swap(a, b);
        }
    }

    fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .map(|(idx, b)| {
                if let BlockType::File(id) = b {
                    idx * (*id as usize)
                } else {
                    0
                }
            })
            .sum()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Block {
    type_: BlockType,
    size: u8,
    pos: usize,
}

struct UnfragmentedDisk {
    blocks: Vec<Block>,
}

impl From<&str> for UnfragmentedDisk {
    fn from(value: &str) -> Self {
        let mut pos = 0;

        let blocks = value
            .chars()
            .enumerate()
            .map(|(idx, c)| {
                let size = c.to_digit(10).unwrap() as u8;
                let type_ = if idx % 2 == 0 {
                    BlockType::File((idx / 2) as u32)
                } else {
                    BlockType::Empty
                };

                let b = Block { type_, size, pos };
                pos += size as usize;

                b
            })
            .collect::<Vec<Block>>();

        Self { blocks }
    }
}

impl UnfragmentedDisk {
    #[tracing::instrument(skip_all)]
    fn optimized(&self) -> Self {
        let mut files = self
            .blocks
            .iter()
            .filter(|b| matches!(b.type_, BlockType::File(_)))
            .collect::<VecDeque<&Block>>();
        let mut empties = self
            .blocks
            .iter()
            .filter(|b| b.type_ == BlockType::Empty)
            .collect::<VecDeque<&Block>>();

        let mut blocks = Vec::new();

        while !(empties.is_empty()) {
            let e = empties.pop_front().unwrap();
            let mut free = e.size;
            let mut pos = e.pos;

            if let Some(f) = files.front() {
                if f.pos < pos {
                    let f = files.pop_front().unwrap();
                    blocks.push(f.clone());
                }
            }

            while free > 0 {
                if let Some((idx, &b)) = files
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, b)| b.pos > pos && b.size <= free)
                {
                    let mut b = b.clone();
                    b.pos = pos;
                    pos += b.size as usize;
                    free -= b.size;
                    blocks.push(b);

                    files.remove(idx);
                } else {
                    break;
                }
            }
        }

        for f in files {
            blocks.push(f.clone());
        }

        Self { blocks }
    }

    fn checksum(&self) -> usize {
        let mut sum = 0;

        for b in &self.blocks {
            if let BlockType::File(id) = b.type_ {
                for i in 0..(b.size as usize) {
                    sum += (b.pos + i) * (id as usize);
                }
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn test_input() -> String {
        String::from("2333133121414131402")
    }

    #[fixture]
    fn puzzle_input() -> String {
        get_input_as_string("day09.txt")
    }

    #[rstest]
    fn test_p1(test_input: String) {
        let mut disk = Disk::from(test_input.as_str());
        disk.optimize();
        let res = disk.checksum();

        assert_eq!(res, 1928);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: String) {
        let mut disk = Disk::from(puzzle_input.as_str());
        disk.optimize();
        let res = disk.checksum();

        assert_eq!(res, 6241633730082);
    }

    #[rstest]
    fn test_p2(test_input: String) {
        let disk = UnfragmentedDisk::from(test_input.as_str());
        let disk = disk.optimized();
        let res = disk.checksum();

        assert_eq!(res, 2858);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: String) {
        let disk = UnfragmentedDisk::from(puzzle_input.as_str());
        let disk = disk.optimized();
        let res = disk.checksum();

        assert_eq!(res, 6265268809555);
    }
}
