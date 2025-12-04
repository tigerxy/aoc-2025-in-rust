//! Shared helpers for Advent of Code 2025 🎄
//!
//! This module provides common utilities such as reading puzzle inputs,
//! trimming lines, and structuring a typical AoC workflow (parse → solve).

use std::fs;

pub mod grid;

/// Reads an input file into a trimmed string.
pub fn read_input(path: &str) -> String {
    fs::read_to_string(path)
        .expect("Failed to read input file")
        .trim()
        .to_string()
}

/// Split the input into non-empty lines.
pub fn lines(input: &str) -> impl Iterator<Item = &str> {
    input.lines().map(str::trim).filter(|l| !l.is_empty())
}

/// A template for all puzzle days:
/// 1. parse the input
/// 2. compute part 1
/// 3. compute part 2
///
/// Not required to use, but a handy pattern.
pub trait AoCDay {
    type Parsed;

    fn parse(input: &str) -> Self::Parsed;

    fn part1(data: &Self::Parsed) -> String;
    fn part2(data: &Self::Parsed) -> String;
}
