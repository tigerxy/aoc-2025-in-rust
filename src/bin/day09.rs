//! Day 09 — Advent of Code 2025 🎄
//!
//! Replace the puzzle logic below with your real solution.
//! The structure is: read → parse → solve part 1 & part 2.

use aoc2025::grid::Grid;
use aoc2025::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;

struct Day09;

impl AoCDay for Day09 {
    type Parsed = Vec<(u64, u64)>;

    fn parse(input: &str) -> Self::Parsed {
        input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let (l, r) = line.trim().split_once(',').unwrap();
                (l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap())
            })
            .collect()
    }

    fn part1(data: &Self::Parsed) -> String {
        let mx = data
            .iter()
            .tuple_combinations()
            .map(|(&(xi, yi), &(xj, yj))| {
                let dx = xi.abs_diff(xj) + 1;
                let dy = yi.abs_diff(yj) + 1;
                dx * dy
            })
            .max()
            .unwrap();
        mx.to_string()
    }

    fn part2(data: &Self::Parsed) -> String {
        if data.len() < 2 {
            return "0".to_string();
        }

        let (xs, ys) = Day09::compressed_axes(data);
        let x_lookup = Day09::build_lookup(&xs);
        let y_lookup = Day09::build_lookup(&ys);

        let w = xs.len() - 1;
        let h = ys.len() - 1;

        let widths: Vec<u64> = xs.windows(2).map(|p| p[1] - p[0]).collect();
        let heights: Vec<u64> = ys.windows(2).map(|p| p[1] - p[0]).collect();

        let mut state = Grid::new(w, h, vec![0u8; w * h]);
        Day09::mark_border(data, &x_lookup, &y_lookup, &mut state);
        Day09::flood_outside(&mut state);

        let pref = Day09::build_prefix(w, h, &widths, &heights, &state);

        let max_area = data
            .iter()
            .tuple_combinations()
            .map(|(&(x1, y1), &(x2, y2))| {
                let lx = x1.min(x2);
                let rx = x1.max(x2) + 1;
                let ty = y1.min(y2);
                let by = y1.max(y2) + 1;

                let xi0 = x_lookup[&lx];
                let xi1 = x_lookup[&rx];
                let yi0 = y_lookup[&ty];
                let yi1 = y_lookup[&by];

                let area = (rx - lx) as u128 * (by - ty) as u128;
                let allowed = Day09::rect_sum(&pref, h, xi0, xi1, yi0, yi1);

                if allowed == area { area } else { 0 }
            })
            .max()
            .unwrap_or(0);

        max_area.to_string()
    }
}

impl Day09 {
    const BORDER: u8 = 0b01;
    const OUTSIDE: u8 = 0b10;

    fn compressed_axes(reds: &[(u64, u64)]) -> (Vec<u64>, Vec<u64>) {
        let build_axis = |selector: fn(&(u64, u64)) -> u64| -> Vec<u64> {
            let (min, max) = reds.iter().map(selector).minmax().into_option().unwrap();

            let mut values: HashSet<u64> = HashSet::new();
            values.extend([min.saturating_sub(1), max.saturating_add(2)]);

            reds.iter().map(selector).for_each(|v| {
                values.insert(v);
                values.insert(v + 1);
            });

            let mut axis: Vec<u64> = values.into_iter().collect();
            axis.sort_unstable();
            axis
        };

        (build_axis(|&(x, _)| x), build_axis(|&(_, y)| y))
    }

    fn build_lookup(axis: &[u64]) -> HashMap<u64, usize> {
        axis.iter()
            .copied()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect()
    }

    fn mark_border(
        reds: &[(u64, u64)],
        xs: &HashMap<u64, usize>,
        ys: &HashMap<u64, usize>,
        state: &mut Grid<u8>,
    ) {
        let segments = reds
            .iter()
            .copied()
            .zip(reds.iter().copied().cycle().skip(1))
            .take(reds.len());

        segments.for_each(|((x1, y1), (x2, y2))| match (x1 == x2, y1 == y2) {
            (true, false) => {
                let xi = xs[&x1];
                let (lo, hi) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
                let y_range = Self::lookup_range(ys, lo, hi + 1);

                Self::paint_range(state, y_range, |yi| (xi, yi));
            }
            (false, true) => {
                let yi = ys[&y1];
                let (lo, hi) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
                let x_range = Self::lookup_range(xs, lo, hi + 1);

                Self::paint_range(state, x_range, |xi| (xi, yi));
            }
            _ => panic!("Non-orthogonal segment: ({x1},{y1}) -> ({x2},{y2})"),
        });
    }

    fn paint_range<F>(state: &mut Grid<u8>, range: std::ops::Range<usize>, mut coord_of: F)
    where
        F: FnMut(usize) -> (usize, usize),
    {
        range.for_each(|v| state[coord_of(v)] |= Self::BORDER);
    }

    fn lookup_range(map: &HashMap<u64, usize>, a: u64, b: u64) -> std::ops::Range<usize> {
        let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
        map[&lo]..map[&hi]
    }

    fn flood_outside(state: &mut Grid<u8>) {
        let mut q = VecDeque::new();
        let w = state.width();
        let h = state.height();

        let perimeter = (0..w)
            .flat_map(|x| [0, h - 1].into_iter().map(move |y| (x, y)))
            .chain((0..h).flat_map(|y| [0, w - 1].into_iter().map(move |x| (x, y))));

        perimeter.for_each(|(x, y)| {
            if state[(x, y)] == 0 {
                state[(x, y)] = Self::OUTSIDE;
                q.push_back((x, y));
            }
        });

        while let Some((x, y)) = q.pop_front() {
            let neighbors: Vec<(usize, usize)> = state
                .neighbors_4(x, y)
                .map(|((nx, ny), _)| (nx, ny))
                .collect();

            neighbors
                .into_iter()
                .for_each(|(nx, ny)| {
                    let v = state[(nx, ny)];
                    if v & (Self::OUTSIDE | Self::BORDER) == 0 {
                        state[(nx, ny)] = v | Self::OUTSIDE;
                        q.push_back((nx, ny));
                    }
                });
        }
    }

    fn build_prefix(
        w: usize,
        h: usize,
        widths: &[u64],
        heights: &[u64],
        state: &Grid<u8>,
    ) -> Vec<u128> {
        let mut pref = vec![0u128; (w + 1) * (h + 1)];
        let stride = h + 1;

        (0..w)
            .flat_map(|x| (0..h).map(move |y| (x, y)))
            .for_each(|(x, y)| {
                let allowed = state[(x, y)] & Self::OUTSIDE == 0;
                let cell_area = if allowed {
                    widths[x] as u128 * heights[y] as u128
                } else {
                    0
                };

                let p_idx = (x + 1) * stride + (y + 1);
                pref[p_idx] = cell_area + pref[x * stride + (y + 1)] + pref[(x + 1) * stride + y]
                    - pref[x * stride + y];
            });

        pref
    }

    #[inline]
    fn rect_sum(pref: &[u128], h: usize, x0: usize, x1: usize, y0: usize, y1: usize) -> u128 {
        let stride = h + 1;
        let a = pref[x1 * stride + y1];
        let b = pref[x0 * stride + y1];
        let c = pref[x1 * stride + y0];
        let d = pref[x0 * stride + y0];
        a + d - b - c
    }
}

fn main() {
    // Allow: `cargo run --bin day09` or specify an input: `cargo run --bin day09 -- foo.txt`
    let args: Vec<String> = env::args().collect();
    let input_path = args
        .get(1)
        .map(String::as_str)
        .unwrap_or("inputs/day09.txt");

    let raw = read_input(input_path);
    let parsed = Day09::parse(&raw);

    println!("🎄 Day 09 — Advent of Code 2025");
    println!("⭐ Part 1: {}", Day09::part1(&parsed));
    println!("⭐ Part 2: {}", Day09::part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

    #[test]
    fn test_parse() {
        let parsed = Day09::parse(SAMPLE);
        assert_eq!(
            parsed,
            vec![
                (7, 1),
                (11, 1),
                (11, 7),
                (9, 7),
                (9, 5),
                (2, 5),
                (2, 3),
                (7, 3),
            ]
        );
    }

    #[test]
    fn test_part1() {
        let parsed = Day09::parse(SAMPLE);
        assert_eq!(Day09::part1(&parsed), "50");
    }

    #[test]
    fn test_part2() {
        let parsed = Day09::parse(SAMPLE);
        assert_eq!(Day09::part2(&parsed), "24");
    }
}
