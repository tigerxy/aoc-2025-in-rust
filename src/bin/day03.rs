//! Day 03 — Advent of Code 2035 🎄
//!
//! Replace the puzzle logic below with your real solution.
//! The structure is: read → parse → solve part 1 & part 2.

use aoc2025::*;
use std::ops::Not;
use std::{env, usize};

struct Day03;

impl AoCDay for Day03 {
    type Parsed = Vec<Vec<u8>>;

    fn parse(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .filter(|line| line.is_empty().not())
            .collect()
    }

    fn part1(data: &Self::Parsed) -> String {
        let joltage = data
            .iter()
            .map(|line| {
                let batteries = calc_joltage(line, 2);
                println!("{}", batteries);
                batteries
            })
            .sum::<u64>();
        format!("{}", joltage)
    }

    fn part2(data: &Self::Parsed) -> String {
        let joltage = data
            .iter()
            .map(|line| {
                let batteries = calc_joltage(line, 12);
                println!("{}", batteries);
                batteries
            })
            .sum::<u64>();
        format!("{}", joltage)
    }
}

fn main() {
    // Allow: `cargo run --bin day03` or specify an input: `cargo run --bin day03 -- foo.txt`
    let args: Vec<String> = env::args().collect();
    let input_path = args
        .get(1)
        .map(String::as_str)
        .unwrap_or("inputs/day03.txt");

    let raw = read_input(input_path);
    let parsed = Day03::parse(&raw);

    println!("🎄 Day 03 — Advent of Code 2035");
    println!("⭐ Part 1: {}", Day03::part1(&parsed));
    println!("⭐ Part 2: {}", Day03::part2(&parsed));
}

fn calc_joltage(batteries: &[u8], length: u8) -> u64 {
    (0..length)
        .rev()
        .fold((0usize, 0u64), |(start_idx, sum), i| {
            batteries[start_idx..batteries.len() - i as usize]
                .iter()
                .enumerate()
                .reduce(|acc, right| if right.1 > acc.1 { right } else { acc })
                .map(|(idx, &first)| {
                    (
                        start_idx + idx + 1,
                        sum + 10_u64.pow(i as u32) * u64::from(first),
                    )
                })
                .unwrap()
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
        987654321111111
        811111111111119
        234234234234278
        818181911112111
        "#;

    #[test]
    fn test_parse() {
        let parsed = Day03::parse(SAMPLE);
        assert_eq!(
            parsed,
            vec![
                [9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
                [8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
                [2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
                [8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
            ]
        );
    }

    #[test]
    fn test_part1() {
        let parsed = Day03::parse(SAMPLE);
        assert_eq!(Day03::part1(&parsed), "357");
    }

    #[test]
    fn test_part2() {
        let parsed = Day03::parse(SAMPLE);
        assert_eq!(Day03::part2(&parsed), "3121910778619");
    }

    #[test]
    fn test_calc_joltage2() {
        assert_eq!(calc_joltage(&vec![1, 1, 9], 2), 19);
        assert_eq!(calc_joltage(&vec![1, 1, 1], 2), 11);
        assert_eq!(calc_joltage(&vec![1, 9, 1], 2), 91);
        assert_eq!(calc_joltage(&vec![6, 6, 1], 2), 66);
        assert_eq!(calc_joltage(&vec![8, 7, 6, 7, 5, 4, 3, 2, 1], 2), 87);
        assert_eq!(calc_joltage(&vec![8, 7, 6, 7, 5, 4, 3, 2, 1], 3), 877);
        assert_eq!(
            calc_joltage(&vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 12),
            987654321111
        );
        assert_eq!(
            calc_joltage(&vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 12),
            811111111119
        );
        assert_eq!(
            calc_joltage(&vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 12),
            434234234278
        );
        assert_eq!(
            calc_joltage(&vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 12),
            888911112111
        );
    }
}
