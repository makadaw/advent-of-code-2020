use std::collections::HashMap;

enum Instruction {
    Mask(i64, i64),
    Assign(usize, i64),
}

impl Instruction {
    fn from(string: &str) -> Option<Instruction> {
        if string.starts_with("mask") {
            let (u, d) =
                string
                    .split(" = ")
                    .nth(1)
                    .unwrap()
                    .chars()
                    .fold((0, 0), |acc, c| match c {
                        '1' => ((acc.0 << 1) | 1, acc.1 << 1),
                        '0' => (acc.0 << 1, (acc.1 << 1) | 1),
                        _ => (acc.0 << 1, acc.1 << 1),
                    });
            Some(Instruction::Mask(u, !d))
        } else {
            scan_fmt!(string, "mem[{d}] = {d}", usize, i64)
                .map(|(idx, num)| Some(Instruction::Assign(idx, num)))
                .unwrap_or(None)
        }
    }
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &str) -> i64 {
    let mem: HashMap<usize, i64> = HashMap::new();
    let mem = input.lines().flat_map(|l| Instruction::from(l)).fold(
        (mem, (0, 0)),
        |(mut mem, mask), ist| match ist {
            Instruction::Mask(u, l) => (mem, (u, l)),
            Instruction::Assign(idx, num) => {
                mem.remove(&idx);
                mem.insert(idx, (num | mask.0) & mask.1);
                (mem, mask)
            }
        },
    );
    mem.0.values().sum()
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
pub fn solve_part2(input: &str) -> usize {
    let mem: HashMap<usize, usize> = HashMap::new();
    let mem = input.lines().fold((mem, vec![]), |(mut mem, mask), line| {
        if line.starts_with("mask") {
            let mask = line.split(" = ").nth(1).unwrap().chars().rev().collect();
            (mem, mask)
        } else {
            let (target, value) = scan_fmt!(line, "mem[{d}] = {d}", usize, usize).unwrap();
            let ts = get_mem_vals(&mask, target);
            for t in ts {
                mem.insert(t, value);
            }
            (mem, mask)
        }
    });
    mem.0.values().sum()
}
