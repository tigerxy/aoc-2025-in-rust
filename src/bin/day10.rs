//! Day 10 — Advent of Code 2025 🎄
//!
//! Replace the puzzle logic below with your real solution.
//! The structure is: read → parse → solve part 1 & part 2.

use aoc2025::*;
use good_lp::{highs, variable, variables, Expression, Solution, SolverModel};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::env;

struct Day10;

impl AoCDay for Day10 {
    type Parsed = Vec<(Vec<usize>, Vec<Vec<usize>>, Vec<usize>)>;

    fn parse(input: &str) -> Self::Parsed {
        input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let tokens = line.split_whitespace().collect_vec();

                let (diagram, rest) = tokens.split_first().unwrap();
                let (requirements, wiring_tokens) = rest.split_last().unwrap();

                let diagram_indices = diagram
                    .chars()
                    .dropping(1)
                    .dropping_back(1)
                    .enumerate()
                    .filter_map(|(i, c)| (c == '#').then_some(i))
                    .collect_vec();

                let wiring = wiring_tokens
                    .iter()
                    .map(|s| {
                        s[1..s.len() - 1]
                            .split(',')
                            .map(|n| n.parse::<usize>().unwrap())
                            .collect_vec()
                    })
                    .collect_vec();

                let requirements_vec = requirements[1..requirements.len() - 1]
                    .split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect_vec();

                (diagram_indices, wiring, requirements_vec)
            })
            .collect_vec()
    }

    fn part1(data: &Self::Parsed) -> String {
        let sum = data
            .iter()
            .map(|(indices, wiring, _)| Day10::calc_min_clicks(indices, wiring))
            .sum::<u64>();
        sum.to_string()
    }

    fn part2(data: &Self::Parsed) -> String {
        let sum = data
            .iter()
            .map(|(_, wiring, requirements)| Day10::calc_min_joltage(requirements, wiring))
            .sum::<u64>();
        sum.to_string()
    }
}

impl Day10 {
    fn calc_min_clicks(diagram: &[usize], wiring: &[Vec<usize>]) -> u64 {
        if diagram.is_empty() {
            return 0;
        }

        let target = diagram.iter().fold(0u64, |mask, &i| mask | (1u64 << i));

        let button_masks: Vec<u64> = wiring
            .iter()
            .map(|btn| btn.iter().fold(0u64, |mask, &i| mask ^ (1u64 << i)))
            .collect();

        let start: u64 = 0;
        if start == target {
            return 0;
        }

        let mut visited: HashSet<u64> = HashSet::new();
        let mut queue: VecDeque<(u64, u64)> = VecDeque::new();

        visited.insert(start);
        queue.push_back((start, 0));

        while let Some((state, dist)) = queue.pop_front() {
            for &bmask in &button_masks {
                let next = state ^ bmask;
                if !visited.insert(next) {
                    continue;
                }
                let next_dist = dist + 1;
                if next == target {
                    return next_dist;
                }
                queue.push_back((next, next_dist));
            }
        }

        panic!("No solution found for given machine configuration");
    }

    fn calc_min_joltage(requirements: &[usize], wiring: &[Vec<usize>]) -> u64 {
        let m = requirements.len();
        let n_buttons = wiring.len();

        if m == 0 || n_buttons == 0 {
            return 0;
        }

        let mut vars = variables!();
        let x_vars: Vec<_> = (0..n_buttons)
            .map(|_| vars.add(variable().min(0).integer()))
            .collect();

        let objective = x_vars
            .iter()
            .fold(Expression::from_other_affine(0.0), |acc, &xj| acc + xj);

        let mut model = vars.minimise(objective).using(highs);

        model = requirements
            .iter()
            .enumerate()
            .fold(model, |model, (i, &req)| {
                let expr = wiring
                    .iter()
                    .enumerate()
                    .filter(|(_, btn)| btn.contains(&i))
                    .fold(Expression::from_other_affine(0.0), |mut acc, (j, _)| {
                        acc.add_mul(1.0, x_vars[j]);
                        acc
                    });

                model.with(expr.eq(req as f64))
            });

        let solution = model
            .solve()
            .expect("ILP-Solver could not solve the problem");

        x_vars
            .iter()
            .map(|&xj| solution.value(xj).round() as u64)
            .sum()
    }
}

fn main() {
    // Allow: `cargo run --bin day10` or specify an input: `cargo run --bin day10 -- foo.txt`
    let args: Vec<String> = env::args().collect();
    let input_path = args
        .get(1)
        .map(String::as_str)
        .unwrap_or("inputs/day10.txt");

    let raw = read_input(input_path);
    let parsed = Day10::parse(&raw);

    println!("🎄 Day 10 — Advent of Code 2025");
    println!("⭐ Part 1: {}", Day10::part1(&parsed));
    println!("⭐ Part 2: {}", Day10::part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
        [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        "#;

    #[test]
    fn test_parse() {
        let parsed = Day10::parse(SAMPLE);
        assert_eq!(parsed.len(), 3);
        assert_eq!(
            *parsed.first().unwrap(),
            (
                vec![1, 2],
                vec![
                    vec![3],
                    vec![1, 3],
                    vec![2],
                    vec![2, 3],
                    vec![0, 2],
                    vec![0, 1]
                ],
                vec![3, 5, 4, 7]
            )
        );
    }

    #[test]
    fn test_part1() {
        let parsed = Day10::parse(SAMPLE);
        assert_eq!(Day10::part1(&parsed), "7");
    }

    #[test]
    fn test_part2() {
        let parsed = Day10::parse(SAMPLE);
        assert_eq!(Day10::part2(&parsed), "33");
    }
}
