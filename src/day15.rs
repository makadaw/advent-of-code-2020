use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.split(',').flat_map(|n| n.parse::<usize>()).collect()
}

fn result_for_turn(numbers: &[usize], turn: usize) -> usize {
    let mut game: HashMap<usize, Vec<usize>> = numbers
        .iter()
        .enumerate()
        .map(|(i, n)| (*n, vec![i + 1]))
        .collect();
    (numbers.len() + 1..=turn).fold(*numbers.last().unwrap(), |last, turn| {
        let entry = &game.entry(last).or_insert_with(|| vec![]);
        let result = if entry.len() >= 2 {
            entry[entry.len() - 1] - entry[entry.len() - 2]
        } else {
            0
        };
        game.entry(result).or_insert_with(|| vec![]).push(turn);
        result
    })
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    result_for_turn(input, 2020)
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    result_for_turn(input, 30_000_000)
}
