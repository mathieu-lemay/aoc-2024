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

#[derive(Clone, Copy, Eq, PartialEq)]
enum BlockType {
    Empty,
    File { id: u32 },
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Block {
    type_: BlockType,
    size: u8,
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
                BlockType::File {
                    id: (idx / 2) as u32,
                }
            } else {
                BlockType::Empty
            };

            for _ in 0..size {
                blocks.push(b);
            }
        }

        Disk { blocks }
    }
}

impl Disk {
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

            (self.blocks[a], self.blocks[b]) = (self.blocks[b], self.blocks[a]);
        }
    }

    fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .map(|(idx, b)| {
                if let BlockType::File { id } = b {
                    idx * (*id as usize)
                } else {
                    0
                }
            })
            .sum()
    }
}

struct UnfragmentedDisk {
    blocks: Vec<Block>,
}

impl From<&str> for UnfragmentedDisk {
    fn from(value: &str) -> Self {
        let blocks = value
            .chars()
            .enumerate()
            .map(|(idx, c)| {
                let size = c.to_digit(10).unwrap() as u8;
                let type_ = if idx % 2 == 0 {
                    BlockType::File {
                        id: (idx / 2) as u32,
                    }
                } else {
                    BlockType::Empty
                };

                Block { type_, size }
            })
            .collect::<Vec<Block>>();

        Self { blocks }
    }
}

impl UnfragmentedDisk {
    fn optimized(&self) -> Self {
        let mut blocks: Vec<Block> = self.blocks.iter().rev().cloned().collect::<Vec<Block>>();

        let mut i = 0;

        while i < blocks.len() {
            let src = blocks[i];
            if src.type_ == BlockType::Empty {
                i += 1;
                continue;
            }

            for j in (i..(blocks.len() - 1)).rev() {
                let tgt = blocks[j];
                if tgt.type_ != BlockType::Empty || src.size > tgt.size {
                    continue;
                }

                blocks[i] = Block {
                    type_: BlockType::Empty,
                    size: src.size,
                };
                blocks[j] = src;

                let remainder = tgt.size - src.size;
                if remainder > 0 {
                    blocks.insert(
                        j,
                        Block {
                            type_: BlockType::Empty,
                            size: remainder,
                        },
                    )
                }

                break;
            }

            i += 1;
        }

        Self {
            blocks: blocks.iter().rev().cloned().collect(),
        }
    }

    fn checksum(&self) -> usize {
        let mut sum = 0;
        let mut idx: usize = 0;

        for b in &self.blocks {
            if let BlockType::File { id } = b.type_ {
                for i in 0..(b.size as usize) {
                    sum += (idx + i) * (id as usize);
                }
            }
            idx += b.size as usize;
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
