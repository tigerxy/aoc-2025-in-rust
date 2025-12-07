//! Day 07 — Advent of Code 2025 🎄
//!
//! Replace the puzzle logic below with your real solution.
//! The structure is: read → parse → solve part 1 & part 2.

use aoc2025::*;
use itertools::Itertools;
use std::env;

struct Day07;

impl AoCDay for Day07 {
    type Parsed = (usize, Vec<Vec<usize>>);

    fn parse(input: &str) -> Self::Parsed {
        let lines: Vec<&str> = input.lines().collect();
        let width = lines.first().map(|l| l.chars().count()).unwrap_or(0);

        let rows = lines
            .iter()
            .map(|l| {
                l.chars()
                    .enumerate()
                    .filter(|(_, c)| *c != '.')
                    .map(|(i, _)| i)
                    .collect_vec()
            })
            .collect_vec();

        (width, rows)
    }

    fn part1(data: &Self::Parsed) -> String {
        let (_, lines) = data;
        let (start, tree) = lines.split_first().unwrap();
        let (_, splits) = tree
            .iter()
            .fold((start.clone(), 0usize), |(beams, splits), row| {
                let mut new_splits = splits;

                let mut new_beams = beams
                    .into_iter()
                    .flat_map(|beam| {
                        if row.contains(&beam) {
                            new_splits += 1;
                            [beam - 1, beam + 1].into_iter().collect_vec()
                        } else {
                            [beam].into_iter().collect_vec()
                        }
                    })
                    .collect_vec();

                new_beams.sort_unstable();
                new_beams.dedup();

                (new_beams, new_splits)
            });

        splits.to_string()
    }

    fn part2(data: &Self::Parsed) -> String {
        let (width, rows) = data;
        let (start_row, tree) = rows.split_first().unwrap();

        let start_x = *start_row.first().expect("start row must contain S");

        let init_count = vec![1_u64; *width];

        let final_count = tree.iter().rev().fold(init_count, |count, row| {
            (0..*width)
                .map(|i| {
                    if row.contains(&i) {
                        let left  = count.get(i.wrapping_sub(1)).copied().unwrap_or(0);
                        let right = count.get(i + 1).copied().unwrap_or(0);
                        left + right
                    } else {
                        count[i]
                    }
                })
                .collect_vec()
        });

        final_count[start_x].to_string()
    }
}

impl Day07 {}

fn main() {
    // Allow: `cargo run --bin day07` or specify an input: `cargo run --bin day07 -- foo.txt`
    let args: Vec<String> = env::args().collect();
    let input_path = args
        .get(1)
        .map(String::as_str)
        .unwrap_or("inputs/day07.txt");

    let raw = read_input(input_path);
    let parsed = Day07::parse(&raw);

    println!("🎄 Day 07 — Advent of Code 2025");
    println!("⭐ Part 1: {}", Day07::part1(&parsed));
    println!("⭐ Part 2: {}", Day07::part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    #[test]
    fn test_parse() {
        let (width, rows) = Day07::parse(SAMPLE);
        assert_eq!(width, 15);
        assert_eq!(
            rows,
            vec![
                vec![7],
                vec![],
                vec![7],
                vec![],
                vec![6, 8],
                vec![],
                vec![5, 7, 9],
                vec![],
                vec![4, 6, 10],
                vec![],
                vec![3, 5, 9, 11],
                vec![],
                vec![2, 6, 12],
                vec![],
                vec![1, 3, 5, 7, 9, 13],
                vec![]
            ]
        );
    }

    #[test]
    fn test_part1() {
        let parsed = Day07::parse(SAMPLE);
        assert_eq!(Day07::part1(&parsed), "21");
    }

    #[test]
    fn test_part2() {
        let parsed = Day07::parse(SAMPLE);
        assert_eq!(Day07::part2(&parsed), "40");
    }
}
