//! Day 05 — Advent of Code 2025 🎄
//!
//! Replace the puzzle logic below with your real solution.
//! The structure is: read → parse → solve part 1 & part 2.

use aoc2025::*;
use itertools::Itertools;
use std::env;
use std::ops::RangeInclusive;

struct Day05;

impl AoCDay for Day05 {
    type Parsed = (Vec<RangeInclusive<u64>>, Vec<u64>);

    fn parse(input: &str) -> Self::Parsed {
        let (ranges_block, ids_block) = input
            .split_once("\n\n")
            .expect("input must contain a blank line between ranges and ids");

        let ranges: Vec<RangeInclusive<u64>> = ranges_block
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|line| {
                let (start, end) = line.split_once('-').expect("range line must contain '-'");

                RangeInclusive::new(
                    start.trim().parse::<u64>().expect("invalid start number"),
                    end.trim().parse::<u64>().expect("invalid end number"),
                )
            })
            .sorted_by_key(|s| *s.start())
            .collect();

        let ids: Vec<u64> = ids_block
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().expect("invalid id"))
            .sorted()
            .collect();

        (ranges, ids)
    }

    fn part1(data: &Self::Parsed) -> String {
        let (ranges, ids) = data.clone();
        let spoiled = ids
            .iter()
            .filter(|id| ranges.iter().any(|r| r.contains(&id)))
            .count();

        spoiled.to_string()
    }

    fn part2(data: &Self::Parsed) -> String {
        let (ranges, _) = data.clone();
        let fresh_ingredients: u64 = Self::merge_all(ranges)
            .iter()
            .map(|r| r.end() - r.start() + 1)
            .sum();
        fresh_ingredients.to_string()
    }
}

impl Day05 {
    fn merge_all(ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
        let mut iter = ranges.into_iter();
        let first = iter.next().unwrap();

        iter.fold(vec![first], |mut acc, r| {
            let last = acc.last_mut().unwrap();

            if r.start() <= &(last.end() + 1) {
                let new_end = (*last.end()).max(*r.end());
                *last = *last.start()..=new_end;
            } else {
                acc.push(r);
            }

            acc
        })
    }
}

fn main() {
    // Allow: `cargo run --bin day05` or specify an input: `cargo run --bin day05 -- foo.txt`
    let args: Vec<String> = env::args().collect();
    let input_path = args
        .get(1)
        .map(String::as_str)
        .unwrap_or("inputs/day05.txt");

    let raw = read_input(input_path);
    let parsed = Day05::parse(&raw);

    println!("🎄 Day 05 — Advent of Code 2025");
    println!("⭐ Part 1: {}", Day05::part1(&parsed));
    println!("⭐ Part 2: {}", Day05::part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
        "#;

    fn r(start: u64, end: u64) -> RangeInclusive<u64> {
        start..=end
    }

    #[test]
    fn test_parse() {
        let (ranges, ids) = Day05::parse(SAMPLE);
        assert_eq!(
            ranges,
            vec![(3, 5), (10, 14), (12, 18), (16, 20)]
                .into_iter()
                .map(|(s, e)| s..=e)
                .collect::<Vec<_>>()
        );
        assert_eq!(ids, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn test_part1() {
        let parsed = Day05::parse(SAMPLE);
        assert_eq!(Day05::part1(&parsed), "3");
    }

    #[test]
    fn test_part2() {
        let parsed = Day05::parse(SAMPLE);
        assert_eq!(Day05::part2(&parsed), "14");
    }

    #[test]
    fn single_range_is_unchanged() {
        let ranges = vec![r(3, 7)];
        let merged = Day05::merge_all(ranges);
        assert_eq!(merged, vec![r(3, 7)]);
    }

    #[test]
    fn non_overlapping_ranges_remain_separate() {
        let ranges = vec![r(1, 3), r(10, 12)];
        let merged = Day05::merge_all(ranges);
        assert_eq!(merged, vec![r(1, 3), r(10, 12)]);
    }

    #[test]
    fn overlapping_ranges_are_merged() {
        let ranges = vec![r(1, 5), r(3, 10)];
        let merged = Day05::merge_all(ranges);
        assert_eq!(merged, vec![r(1, 10)]);
    }

    #[test]
    fn touching_ranges_are_merged() {
        let ranges = vec![r(1, 3), r(4, 6)];
        let merged = Day05::merge_all(ranges);
        assert_eq!(merged, vec![r(1, 6)]);
    }

    #[test]
    fn contained_range_does_not_change_outer_range() {
        let ranges = vec![r(1, 10), r(3, 7)];
        let merged = Day05::merge_all(ranges);
        assert_eq!(merged, vec![r(1, 10)]);
    }

    #[test]
    fn ranges_with_gaps_form_multiple_blocks() {
        let ranges = vec![r(1, 3), r(2, 4), r(10, 12), r(13, 15), r(20, 22)];
        let merged = Day05::merge_all(ranges);
        assert_eq!(merged, vec![r(1, 4), r(10, 15), r(20, 22)]);
    }
}
