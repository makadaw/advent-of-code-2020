use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Point {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Point {
        Point { x, y, z, w: 0 }
    }
    fn neighbors_in_dimensions(&self, dim: usize) -> Vec<Point> {
        (0..dim)
            .map(|_| &[-1, 0, 1])
            .multi_cartesian_product()
            .map(|v| (v[0], v[1], v[2], if v.len() > 3 { v[3] } else { &0 }))
            .map(|(dx, dy, dz, dw)| Point {
                x: self.x + dx,
                y: self.y + dy,
                z: self.z + dz,
                w: self.w + dw,
            })
            .collect()
    }
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> HashSet<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Point::new(x as isize, y as isize, 0))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn solve_part(input: &HashSet<Point>, dimensions: usize) -> usize {
    let map = (0..6).fold(input.clone(), |last, _| {
        last.iter()
            // Count active cubes around each active cube
            .fold(HashMap::new(), |mut count: HashMap<Point, usize>, p| {
                // Each neighbor get +1 for surround active cubes
                p.neighbors_in_dimensions(dimensions)
                    .iter()
                    .for_each(|n| *count.entry(n.clone()).or_default() += 1);
                count
            })
            .into_iter()
            // Filter all cubes that do not have enough neighbours
            // if cube is active we also count it in the fold, so we need to check with 3, not 2
            .filter(|(k, c)| *c == 3 || *c == 4 && last.contains(k))
            // Map to points and build a next set
            .map(|(k, _)| k)
            .collect()
    });
    map.len()
}

#[aoc(day17, part1)]
#[allow(clippy::implicit_hasher)]
pub fn solve_part1(input: &HashSet<Point>) -> usize {
    solve_part(input, 3)
}

#[aoc(day17, part2)]
#[allow(clippy::implicit_hasher)]
pub fn solve_part2(input: &HashSet<Point>) -> usize {
    solve_part(input, 4)
}
