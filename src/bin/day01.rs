//! Day 01 — Advent of Code 2025 🎄
//!
//! Replace the puzzle logic below with your real solution.
//! The structure is: read → parse → solve part 1 & part 2.

use aoc2025::*;
use std::env;

struct Day01;

impl AoCDay for Day01 {
    type Parsed = Vec<i32>;

    fn parse(input: &str) -> Self::Parsed {
        lines(input)
            .map(|line| {
                let (dir, num) = line.split_at(1);
                let n = num.parse::<i32>().unwrap();
                match dir {
                    "L" => -n,
                    "R" => n,
                    _ => panic!("Unknown direction: {}", dir),
                }
            })
            .collect()
    }

    fn part1(data: &Self::Parsed) -> String {
        let sum = data
            .iter()
            .fold((0, 50), |c, m| {
                let (sum, pos) = c;
                let v = (pos + m) % 100;
                match v {
                    0 => (sum + 1, v),
                    _ => (sum, v),
                }
            })
            .0;
        format!("{}", sum)
    }

    fn part2(data: &Self::Parsed) -> String {
        let sum = data
            .iter()
            .fold((0, 50), |(sum, pos), &m| {
                let (hits, new_pos) = eval_move(pos, m);
                (sum + hits, new_pos)
            })
            .0;

        format!("{}", sum)
    }
}

fn eval_move(pos: i32, delta: i32) -> (i64, i32) {
    if delta == 0 {
        return (0, pos);
    }

    let steps = delta.abs() as i64;
    let p = pos.rem_euclid(100) as i64;

    let first = {
        let f = if delta > 0 {
            (100 - p).rem_euclid(100)
        } else {
            p.rem_euclid(100)
        };

        if f == 0 { 100 } else { f }
    };

    let hits = if first > steps {
        0
    } else {
        1 + (steps - first) / 100
    };

    let new_pos = (pos + delta).rem_euclid(100);

    (hits, new_pos)
}

fn main() {
    // Allow: `cargo run --bin day01` or specify an input: `cargo run --bin day01 -- foo.txt`
    let args: Vec<String> = env::args().collect();
    let input_path = args
        .get(1)
        .map(String::as_str)
        .unwrap_or("inputs/day01.txt");

    let raw = read_input(input_path);
    let parsed = Day01::parse(&raw);

    println!("🎄 Day 01 — Advent of Code 2025");
    println!("⭐ Part 1: {}", Day01::part1(&parsed));
    println!("⭐ Part 2: {}", Day01::part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;

    #[test]
    fn test_part1() {
        let parsed = Day01::parse(SAMPLE);
        assert_eq!(Day01::part1(&parsed), "3");
    }

    #[test]
    fn test_part2() {
        let parsed = Day01::parse(SAMPLE);
        assert_eq!(Day01::part2(&parsed), "6");
    }
}
