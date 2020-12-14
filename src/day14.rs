use std::collections::HashMap;

fn parse_mask(input: &str) -> (i64, i64) {
    let (u, d) = input
        .split(" = ")
        .nth(1)
        .unwrap()
        .chars()
        .fold((0, 0), |acc, c| match c {
            '1' => ((acc.0 << 1) | 1, acc.1 << 1),
            '0' => (acc.0 << 1, (acc.1 << 1) | 1),
            _ => (acc.0 << 1, acc.1 << 1),
        });
    (u, !d)
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &str) -> i64 {
    let mut mask: (i64, i64) = (0, 0);
    let mut mem: HashMap<usize, i64> = HashMap::new();
    for l in input.lines() {
        if l.starts_with("mask") {
            mask = parse_mask(l)
        } else {
            let (idx, num) = scan_fmt!(l, "mem[{d}] = {d}", usize, i64).unwrap();
            mem.remove(&idx);
            mem.insert(idx, (num | mask.0) & mask.1);
        }
    }
    mem.values().sum()
}

fn get_mem_vals(mask: &[char], value: usize) -> Vec<usize> {
    mask.iter().enumerate().fold(vec![0], |opts, (ix, c)| {
        let value_bit = value & (1 << ix);
        match *c {
            '0' => opts.iter().map(|x| (value_bit) | x).collect(),
            '1' => opts.iter().map(|x| (1 << ix) | x).collect(),
            'X' => (0..=1)
                .flat_map(|new_bit| opts.iter().map(move |x| (new_bit << ix) | x))
                .collect(),
            _ => panic!("Why we are here?"),
        }
    })
}

#[aoc(day14, part2)]
pub fn p2(input: &str) -> usize {
    let mut mem: HashMap<usize, usize> = HashMap::new();
    let mut mask: Vec<char> = Vec::new();
    for l in input.lines() {
        if l.starts_with("mask") {
            mask = l.split(" = ").nth(1).unwrap().chars().rev().collect();
        } else {
            let (target, value) = scan_fmt!(l, "mem[{d}] = {d}", usize, usize).unwrap();
            let ts = get_mem_vals(&mask, target);
            for t in ts {
                mem.insert(t, value);
            }
        }
    }
    mem.values().sum()
}
