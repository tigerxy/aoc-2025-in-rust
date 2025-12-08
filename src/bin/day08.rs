//! Day 08 — Advent of Code 2025 🎄
//!
//! Replace the puzzle logic below with your real solution.
//! The structure is: read → parse → solve part 1 & part 2.

use aoc2025::*;
use itertools::Itertools;
use std::env;

struct Day08;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point3 {
    pub fn distance2(&self, other: &Self) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz // squared distance (faster)
    }
}

impl AoCDay for Day08 {
    type Parsed = Vec<Point3>;

    fn parse(input: &str) -> Self::Parsed {
        input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let mut it = line.split(',');
                let x = it.next().unwrap().trim().parse::<i64>().unwrap();
                let y = it.next().unwrap().trim().parse::<i64>().unwrap();
                let z = it.next().unwrap().trim().parse::<i64>().unwrap();
                Point3 { x, y, z }
            })
            .collect_vec()
    }

    fn part1(data: &Self::Parsed) -> String {
        let n = data.len();

        let edges = Day08::all_distances(data)
            .iter()
            .sorted_by_key(|(length, _, _)| length)
            .take(Day08::NUM_SHORTEST_CONNECTIONS)
            .cloned()
            .collect_vec();

        let adj = edges.iter().fold(vec![vec![]; n], |mut acc, &(_d, i, j)| {
            acc[i].push(j);
            acc[j].push(i);
            acc
        });

        let sizes: Vec<usize> = (0..n)
            .scan(&mut vec![false; n], |visited, start| {
                if visited[start] {
                    return Some(None);
                }

                let mut stack = vec![start];
                let mut size = 0;

                while let Some(v) = stack.pop() {
                    if !visited[v] {
                        visited[v] = true;
                        size += 1;

                        adj[v]
                            .iter()
                            .filter(|&&n| !visited[n])
                            .for_each(|&n| stack.push(n));
                    }
                }

                Some(Some(size))
            })
            .flatten()
            .sorted()
            .rev()
            .collect();

        let result: usize = sizes.iter().take(3).product();
        result.to_string()
    }

    fn part2(data: &Self::Parsed) -> String {
        let n = data.len();
        if n == 0 {
            return "0".to_string();
        }

        let (i, j) = Day08::all_distances(data)
            .into_iter()
            .sorted_by_key(|(d, _, _)| *d)
            .scan(
                ((0..n).collect::<Vec<_>>(), vec![1usize; n], n),
                |(parent, size, components), (_d, i, j)| {
                    let mut ra = Day08::find(parent, i);
                    let mut rb = Day08::find(parent, j);

                    if ra == rb {
                        return Some(None);
                    }

                    if size[ra] < size[rb] {
                        std::mem::swap(&mut ra, &mut rb);
                    }
                    parent[rb] = ra;
                    size[ra] += size[rb];
                    *components -= 1;

                    if *components == 1 {
                        Some(Some((i, j)))
                    } else {
                        Some(None)
                    }
                },
            )
            .flatten()
            .next()
            .unwrap();

        (data[i].x * data[j].x).to_string()
    }
}

impl Day08 {
    #[cfg(not(test))]
    pub const NUM_SHORTEST_CONNECTIONS: usize = 1000;
    #[cfg(test)]
    pub const NUM_SHORTEST_CONNECTIONS: usize = 10;

    pub fn all_distances(points: &[Point3]) -> Vec<(i64, usize, usize)> {
        points
            .iter()
            .enumerate()
            .flat_map(|(i, a)| {
                points[i + 1..]
                    .iter()
                    .enumerate()
                    .map(move |(j, b)| (a.distance2(b), i, i + 1 + j))
            })
            .collect_vec()
    }

    fn find(parent: &mut [usize], mut x: usize) -> usize {
        while parent[x] != x {
            let p = parent[x];
            parent[x] = parent[p];
            x = p;
        }
        x
    }
}

fn main() {
    // Allow: `cargo run --bin day08` or specify an input: `cargo run --bin day08 -- foo.txt`
    let args: Vec<String> = env::args().collect();
    let input_path = args
        .get(1)
        .map(String::as_str)
        .unwrap_or("inputs/day08.txt");

    let raw = read_input(input_path);
    let parsed = Day08::parse(&raw);

    println!("🎄 Day 08 — Advent of Code 2025");
    println!("⭐ Part 1: {}", Day08::part1(&parsed));
    println!("⭐ Part 2: {}", Day08::part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

    #[test]
    fn test_parse() {
        let jboxes = Day08::parse(SAMPLE);
        assert_eq!(jboxes.len(), 20);
        assert_eq!(
            *jboxes.first().unwrap(),
            Point3 {
                x: 162,
                y: 817,
                z: 812
            }
        );
    }

    #[test]
    fn test_part1() {
        let parsed = Day08::parse(SAMPLE);
        assert_eq!(Day08::part1(&parsed), "40");
    }

    #[test]
    fn test_part2() {
        let parsed = Day08::parse(SAMPLE);
        assert_eq!(Day08::part2(&parsed), "25272");
    }
}
