use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<String>> {
    input
        .split("\n\n")
        .map(|g| g.lines().map(String::from).collect())
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Vec<String>]) -> usize {
    input
        .iter()
        .map(|g| g.iter().flat_map(|p| p.chars().collect::<Vec<char>>()).collect::<HashSet<char>>())
        .map(|m| m.len())
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Vec<String>]) -> usize {
    input
        .iter()
        .flat_map(|g| {
            let mut items = g.iter().map(|p| p.chars().collect::<HashSet<char>>());
            items.next().map(|set| items.fold(set, |set1, set2| set1.intersection(&set2).cloned().collect()))
        })
        .map(|m| m.len())
        .sum()
}
