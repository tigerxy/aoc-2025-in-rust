//! Day 02 — Advent of Code 2025 🎄
//!
//! Replace the puzzle logic below with your real solution.
//! The structure is: read → parse → solve part 1 & part 2.

use aoc2025::*;
use std::collections::HashSet;
use std::env;

struct Day02;

impl AoCDay for Day02 {
    type Parsed = Vec<(u64, u64)>;

    fn parse(input: &str) -> Self::Parsed {
        input
            .split(|c| c == ',')
            .map(|e| e.trim().split_once(|e| e == '-').unwrap())
            .map(|(from, to)| (from.parse::<u64>().unwrap(), to.parse::<u64>().unwrap()))
            .collect()
    }

    fn part1(data: &Self::Parsed) -> String {
        let sum = data.iter().fold(0, |acc, &(start, end)| {
            let ids = calc_twice(start, end);
            println!(
                "{}-{} IDs: {}",
                start,
                end,
                ids.iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            acc + ids.iter().sum::<u64>()
        });
        format!("{}", sum)
    }

    fn part2(data: &Self::Parsed) -> String {
        let sum = data.iter().fold(0, |acc, &(start, end)| {
            let ids: Vec<u64> = calc_all(start, end);
            println!(
                "{}-{} IDs: {}",
                start,
                end,
                ids.iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            acc + ids.iter().sum::<u64>()
        });
        format!("{}", sum)
    }
}

fn main() {
    // Allow: `cargo run --bin day02` or specify an input: `cargo run --bin day02 -- foo.txt`
    let args: Vec<String> = env::args().collect();
    let input_path = args
        .get(1)
        .map(String::as_str)
        .unwrap_or("inputs/day02.txt");

    let raw = read_input(input_path);
    let parsed = Day02::parse(&raw);

    println!("🎄 Day 02 — Advent of Code 2025");
    println!("⭐ Part 1: {}", Day02::part1(&parsed));
    println!("⭐ Part 2: {}", Day02::part2(&parsed));
}

fn calc_twice(start: u64, end: u64) -> Vec<u64> {
    calc(start, end, 2)
}

fn calc_all(start: u64, end: u64) -> Vec<u64> {
    let mut ids = (1..=(end.to_string().len() / 2))
        .flat_map(|n| calc(start, end, n))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<u64>>();

    ids.sort();
    ids
}

fn calc(start: u64, end: u64, divider: usize) -> Vec<u64> {
    let mut x = start.max(11);
    let mut sum = Vec::new();

    while x <= end {
        let s = x.to_string();
        let len = s.len();

        if divider == 1 {
            let first = s.chars().next().unwrap();
            let cand: u64 = std::iter::repeat(first)
                .take(len)
                .collect::<String>()
                .parse()
                .unwrap();

            if cand >= start && cand <= end {
                sum.push(cand);
            }

            let base = 10u64.pow((len.saturating_sub(1)) as u32);
            x = (x / base + 1) * base;
            continue;
        }

        if len % divider == 0 {
            let chunk_len = len / divider;
            let first_chunk = &s[..chunk_len];
            let cand: u64 = first_chunk.repeat(divider).parse().unwrap();

            if cand >= start && cand <= end {
                sum.push(cand);
            }
        }

        let step_exp = len - len / divider;
        let step = 10u64.pow(step_exp as u32);
        x = (x / step) * step + step;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
"#;
    #[test]
    fn test_parse() {
        let input = r#"
            30-31,22-25
        "#;
        let parsed: Vec<(u64, u64)> = Day02::parse(input);
        assert_eq!(parsed, vec![(30, 31), (22, 25)]);
    }

    #[test]
    fn test_part1() {
        let parsed = Day02::parse(SAMPLE);
        assert_eq!(Day02::part1(&parsed), "1227775554");
    }

    #[test]
    fn test_part2() {
        let parsed = Day02::parse(SAMPLE);
        assert_eq!(Day02::part2(&parsed), "4174379265");
    }

    #[test]
    fn test_calc() {
        assert_eq!(calc(998, 1012, 5), vec![]);
        assert_eq!(calc(998, 1012, 4), vec![]);
        assert_eq!(calc(998, 1012, 3), vec![999]);
        assert_eq!(calc(998, 1012, 2), vec![1010]);
        assert_eq!(calc(998, 1012, 1), vec![999]);
    }

    #[test]
    fn test_calc_all() {
        assert_eq!(calc_all(1, 22), vec![11, 22]);
        assert_eq!(calc_all(998, 1012), vec![999, 1010]);
    }
}
