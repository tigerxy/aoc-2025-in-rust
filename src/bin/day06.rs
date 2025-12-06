//! Day 06 — Advent of Code 2025 🎄
//!
//! Replace the puzzle logic below with your real solution.
//! The structure is: read → parse → solve part 1 & part 2.

use aoc2025::*;
use std::env;

struct Day06;

impl AoCDay for Day06 {
    type Parsed = Vec<(Vec<u64>, char)>;

    fn parse(input: &str) -> Self::Parsed {
        let mut lines: Vec<_> = input.lines().filter(|l| !l.trim().is_empty()).collect();
        let ops: Vec<char> = lines
            .pop()
            .unwrap()
            .split_whitespace()
            .map(|s| s.chars().next().unwrap())
            .collect();
        let rows: Vec<Vec<u64>> = lines
            .iter()
            .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
            .collect();

        (0..ops.len())
            .map(|c| (rows.iter().map(|r| r[c]).collect(), ops[c]))
            .collect()
    }

    fn part1(data: &Self::Parsed) -> String {
        let sum: u64 = data
            .iter()
            .map(|(nums, op)| match op {
                '+' => nums.iter().copied().sum::<u64>(),
                '*' => nums.iter().copied().product::<u64>(),
                _ => unreachable!(),
            })
            .sum::<u64>();
        sum.to_string()
    }

    fn part2(data: &Self::Parsed) -> String {
        String::new()
    }
}

impl Day06 {
    pub fn solve_cephalopod(input: &str) -> u64 {
        let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
        let h = lines.len();
        if h == 0 {
            return 0;
        }

        let w = lines.iter().map(|l| l.len()).max().unwrap_or(0);
        let op_row = h - 1;

        let mut total = 0u64;
        let mut nums: Vec<u64> = Vec::new();
        let mut op: Option<u8> = None;

        for col in 0..w {
            let mut non_space = false;
            let mut n = 0u64;
            let mut has_digit = false;
            let mut col_op: Option<u8> = None;

            for (row, line) in lines.iter().enumerate() {
                let c = line.get(col).copied().unwrap_or(b' ');
                if c == b' ' {
                    continue;
                }
                non_space = true;

                if row == op_row {
                    col_op = Some(c);
                } else if c.is_ascii_digit() {
                    n = n * 10 + (c - b'0') as u64;
                    has_digit = true;
                }
            }

            if !non_space {
                if !nums.is_empty() || op.is_some() {
                    total += Self::eval_problem_stream(&nums, op);
                    nums.clear();
                    op = None;
                }
                continue;
            }

            if has_digit {
                nums.push(n);
            }
            if let Some(o) = col_op {
                op.get_or_insert(o);
            }
        }

        if !nums.is_empty() || op.is_some() {
            total += Self::eval_problem_stream(&nums, op);
        }

        total
    }

    fn eval_problem_stream(nums: &[u64], op: Option<u8>) -> u64 {
        let op = op.expect("Problem without operator");

        match op {
            b'+' => nums.iter().copied().sum(),
            b'*' => nums.iter().copied().fold(1, |a, n| a * n),
            _ => panic!("Unknown operator"),
        }
    }
}

fn main() {
    // Allow: `cargo run --bin day06` or specify an input: `cargo run --bin day06 -- foo.txt`
    let args: Vec<String> = env::args().collect();
    let input_path = args
        .get(1)
        .map(String::as_str)
        .unwrap_or("inputs/day06.txt");

    let raw = read_input(input_path);
    let parsed = Day06::parse(&raw);

    println!("🎄 Day 06 — Advent of Code 2025");
    println!("⭐ Part 1: {}", Day06::part1(&parsed));
    println!("⭐ Part 2: {}", Day06::solve_cephalopod(&raw));
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    const SAMPLE: &str = r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "#;

    #[test]
    fn test_parse() {
        let columns = Day06::parse(SAMPLE);
        let nums: Vec<Vec<u64>> = columns.iter().map(|(nums, _)| nums.clone()).collect();

        assert_eq!(
            columns.iter().map(|(_, op)| *op).collect_vec(),
            vec!['*', '+', '*', '+']
        );
        assert_eq!(
            nums,
            vec![
                vec![123, 45, 6],
                vec![328, 64, 98],
                vec![51, 387, 215],
                vec![64, 23, 314],
            ]
        );
    }

    #[test]
    fn test_part1() {
        let parsed = Day06::parse(SAMPLE);
        assert_eq!(Day06::part1(&parsed), "4277556");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day06::solve_cephalopod(SAMPLE).to_string(), "3263827");
    }
}
