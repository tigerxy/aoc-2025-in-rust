//! Day 11 — Advent of Code 2025 🎄
//!
//! Replace the puzzle logic below with your real solution.
//! The structure is: read → parse → solve part 1 & part 2.

use aoc2025::*;
use std::collections::HashMap;
use std::env;

struct Day11;

impl AoCDay for Day11 {
    type Parsed = HashMap<String, Vec<String>>;

    fn parse(input: &str) -> Self::Parsed {
        input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let (key, value) = line.split_once(":").unwrap();
                (
                    key.trim().to_string(),
                    value
                        .split_whitespace()
                        .map(|s| s.trim().to_string())
                        .collect(),
                )
            })
            .collect()
    }

    fn part1(data: &Self::Parsed) -> String {
        let required_index: HashMap<&str, usize> = HashMap::new();
        let full_mask: u64 = 0;

        let mut memo = HashMap::new();
        let result = Day11::count_paths_with_requirements(
            "you",
            "out",
            data,
            &required_index,
            0,
            full_mask,
            &mut memo,
        );

        result.to_string()
    }

    fn part2(data: &Self::Parsed) -> String {
        let mut required_index: HashMap<&str, usize> = HashMap::new();
        required_index.insert("dac", 0);
        required_index.insert("fft", 1);

        let full_mask: u64 = (1u64 << required_index.len()) - 1;

        let mut memo = HashMap::new();
        let result = Day11::count_paths_with_requirements(
            "svr",
            "out",
            data,
            &required_index,
            0,
            full_mask,
            &mut memo,
        );

        result.to_string()
    }
}

impl Day11 {
    fn count_paths_with_requirements<'a>(
        node: &'a str,
        target: &'a str,
        graph: &'a HashMap<String, Vec<String>>,
        required_index: &HashMap<&'a str, usize>,
        visited_mask: u64,
        full_mask: u64,
        memo: &mut HashMap<(&'a str, u64), u64>,
    ) -> u64 {
        let mut mask = visited_mask;
        if let Some(&idx) = required_index.get(node) {
            mask |= 1u64 << idx;
        }

        if node == target {
            return if mask & full_mask == full_mask { 1 } else { 0 };
        }

        let key = (node, mask);
        if let Some(&cached) = memo.get(&key) {
            return cached;
        }

        let children = match graph.get(node) {
            Some(cs) => cs,
            None => {
                memo.insert(key, 0);
                return 0;
            }
        };

        let total = children
            .iter()
            .map(|child| {
                Self::count_paths_with_requirements(
                    child.as_str(),
                    target,
                    graph,
                    required_index,
                    mask,
                    full_mask,
                    memo,
                )
            })
            .sum();

        memo.insert(key, total);
        total
    }
}

fn main() {
    // Allow: `cargo run --bin day11` or specify an input: `cargo run --bin day11 -- foo.txt`
    let args: Vec<String> = env::args().collect();
    let input_path = args
        .get(1)
        .map(String::as_str)
        .unwrap_or("inputs/day11.txt");

    let raw = read_input(input_path);
    let parsed = Day11::parse(&raw);

    println!("🎄 Day 11 — Advent of Code 2025");
    println!("⭐ Part 1: {}", Day11::part1(&parsed));
    println!("⭐ Part 2: {}", Day11::part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = r#"
        aaa: you hhh
        you: bbb ccc
        bbb: ddd eee
        ccc: ddd eee fff
        ddd: ggg
        eee: out
        fff: out
        ggg: out
        hhh: ccc fff iii
        iii: out
        "#;

    const SAMPLE2: &str = r#"
        svr: aaa bbb
        aaa: fft
        fft: ccc
        bbb: tty
        tty: ccc
        ccc: ddd eee
        ddd: hub
        hub: fff
        eee: dac
        dac: fff
        fff: ggg hhh
        ggg: out
        hhh: out
        "#;

    #[test]
    fn test_parse() {
        let parsed = Day11::parse(SAMPLE1);
        assert_eq!(parsed.len(), 10);
        let value = parsed.get("you").unwrap();
        assert_eq!(*value, vec!["bbb".to_string(), "ccc".to_string(),]);
    }

    #[test]
    fn test_part1() {
        let parsed = Day11::parse(SAMPLE1);
        assert_eq!(Day11::part1(&parsed), "5");
    }

    #[test]
    fn test_part2() {
        let parsed = Day11::parse(SAMPLE2);
        assert_eq!(Day11::part2(&parsed), "2");
    }
}
