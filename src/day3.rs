#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|s| s.chars().collect() )
        .collect()
}

#[aoc(day3, part1)]
pub fn solve1(input: &[Vec<char>]) -> u32 {
    let mut x = 0;
    input
        .iter()
        .filter(|line| {
            let char = line[x % line.len()];
            x += 3;
            char == '#'
        })
        .count() as u32
}

fn trees_on_the_slope(slope: &[Vec<char>], right: usize, down: usize) -> usize {
    let mut x = 0;
    let mut num = 0;
    for line in slope.iter().step_by(down.clone()) {
        let char = line[x % line.len()];
        x += right;
        if char == '#' {
            num += 1;
        }
    }
    num
}

#[aoc(day3, part2)]
pub fn solve2(input: &[Vec<char>]) -> usize {
    vec![(1, 1), (3, 1), (5,1), (7,1), (1,2)]
        .iter()
        .map(|(right, down)| trees_on_the_slope(input, *right, *down))
        .product()
}
