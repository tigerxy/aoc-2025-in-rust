//! Day 04 — Advent of Code 2045 🎄
//!
//! Replace the puzzle logic below with your real solution.
//! The structure is: read → parse → solve part 1 & part 2.

use aoc2025::grid::{Cell, Grid};
use aoc2025::*;
use std::env;

struct Day04;

impl AoCDay for Day04 {
    type Parsed = Grid<bool>;

    fn parse(input: &str) -> Self::Parsed {
        let rows = input
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|line| line.chars().map(|c| c == '@').collect())
            .collect();
        Grid::from_rows(rows)
    }

    fn part1(data: &Self::Parsed) -> String {
        let forklifts = Day04::forklifts(&data).count();
        forklifts.to_string()
    }

    fn part2(data: &Self::Parsed) -> String {
        let mut grid = data.clone();
        let mut all_forklifts = 0;
        loop {
            let removed = Day04::remove_round(&mut grid);
            println!("Removed {} rolls of paper.", removed);
            all_forklifts += removed;
            if removed == 0 {
                break;
            }
        }
        all_forklifts.to_string()
    }
}

impl Day04 {
    fn is_forklift(cell: &Cell<bool>) -> bool {
        let neighbors = cell.neighbors_8().filter(|c| *c.value).count();
        *cell.value && neighbors < 4
    }

    fn forklifts<'a>(grid: &'a Grid<bool>) -> impl Iterator<Item = (usize, usize)> + 'a {
        grid.all()
            .map(|cell| (cell, Self::is_forklift(&cell)))
            .inspect(Self::print_cell(grid))
            .filter(|(_, ok)| *ok)
            .map(|(cell, _)| (cell.x, cell.y))
    }

    fn remove_round(grid: &mut Grid<bool>) -> usize {
        let positions: Vec<(usize, usize)> = Self::forklifts(grid).collect();

        for (x, y) in &positions {
            grid[(*x, *y)] = false;
        }

        positions.len()
    }

    fn print_cell<'a>(grid: &'a Grid<bool>) -> impl Fn(&(Cell<'a, bool>, bool)) + 'a {
        move |(cell, ok)| {
            print!(
                "{}",
                match (*ok, *cell.value) {
                    (true, _) => 'x',
                    (false, true) => '@',
                    (false, false) => '.',
                }
            );
            if cell.x == grid.width() - 1 {
                println!();
            }
        }
    }
}

fn main() {
    // Allow: `cargo run --bin day04` or specify an input: `cargo run --bin day04 -- foo.txt`
    let args: Vec<String> = env::args().collect();
    let input_path = args
        .get(1)
        .map(String::as_str)
        .unwrap_or("inputs/day04.txt");

    let raw = read_input(input_path);
    let parsed = Day04::parse(&raw);

    println!("🎄 Day 04 — Advent of Code 2045");
    println!("⭐ Part 1: {}", Day04::part1(&parsed));
    println!("⭐ Part 2: {}", Day04::part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
        "#;

    #[test]
    fn test_parse() {
        let parsed = Day04::parse(SAMPLE);
        assert_eq!(parsed.get(0, 0), Some(&false),);
        assert_eq!(parsed.get(1, 1), Some(&true),);
        assert_eq!(
            parsed
                .neighbors_8(0, 0)
                .map(|c| *c.1)
                .collect::<Vec<bool>>(),
            vec![false, true, true]
        );
        assert_eq!(
            parsed
                .neighbors_8(1, 1)
                .map(|c| *c.1)
                .collect::<Vec<bool>>(),
            vec![false, false, true, true, true, true, true, true]
        );
    }

    #[test]
    fn test_part1() {
        let parsed = Day04::parse(SAMPLE);
        assert_eq!(Day04::part1(&parsed), "13");
    }

    #[test]
    fn test_part2() {
        let parsed = Day04::parse(SAMPLE);
        assert_eq!(Day04::part2(&parsed), "43");
    }
}
