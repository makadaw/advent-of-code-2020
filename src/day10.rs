#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.lines().flat_map(|s| s.parse::<usize>()).collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    let mut input = input.to_vec();
    input.sort();
    let mut one_count = 1;
    let mut third_count = 1;
    for (i, val) in input[..input.len() - 1].iter().enumerate() {
        match input[i + 1] - val {
            1 => one_count += 1,
            3 => third_count += 1,
            _ => {}
        }
    }
    one_count * third_count
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    let mut input = input.to_vec();
    input.sort();
    input.insert(0, 0); // Add start value

    let mut cache = vec![0; input.len()];
    cache[0] = 1;

    for (i, val) in input.iter().enumerate() {
        for (j, output) in input.iter().enumerate().skip(i + 1) {
            if output - val > 3 {
                break;
            }
            cache[j] += cache[i]
        }
    }

    cache.last().copied().unwrap_or(0)
}
