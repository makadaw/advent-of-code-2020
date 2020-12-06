use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Password {
    range: RangeInclusive<usize>,
    char: char,
    password: String,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
    input
        .lines()
        .map(|s| {
            let parts = s.split(' ').collect::<Vec<&str>>();
            // TODO String parsing
            let in_range = parts[0]
                .split('-')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            // TODO Ranges
            Password {
                range: std::ops::RangeInclusive::new(in_range[0], in_range[1]),
                char: parts[1].chars().next().unwrap(),
                password: String::from(parts[2]),
            }
        })
        .collect()
}

fn validate(pass: &Password) -> bool {
    pass.range
        .contains(&pass.password.chars().filter(|c| c == &pass.char).count())
}

#[aoc(day2, part1)]
pub fn solve1(input: &[Password]) -> u32 {
    input.iter().filter(|p| validate(p)).count() as u32
}

fn validate2(pass: &Password) -> bool {
    let mut chars = pass.password.chars();
    // TODO Range access
    let (from, to) = pass.range.clone().into_inner();
    let first = chars.nth(from - 1).unwrap();
    let second = chars.nth(to - from - 1).unwrap();
    (pass.char == first || pass.char == second) && first != second
}

#[aoc(day2, part2)]
pub fn solve2(input: &[Password]) -> u32 {
    input.iter().filter(|p| validate2(p)).count() as u32
}
