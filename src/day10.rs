#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<usize> {
    let mut output: Vec<usize> = input.lines().flat_map(|s| s.parse::<usize>()).collect();
    output.sort();
    output
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
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

#[aoc(day10, part1, functional)]
pub fn solve_part1_fun(input: &[usize]) -> usize {
    let (_, a, b) = input
        .iter()
        .fold((0, 0, 0), |(last, a, b), &x| match x - last {
            1 => (x, a + 1, b),
            3 => (x, a, b + 1),
            _ => (x, a, b),
        });
    a * (b + 1)
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    let mut input = input.to_vec();
    input.insert(0, 0); // Add start value

    let mut cache = vec![0; input.len()];
    cache[0] = 1;

    for (i, val) in input.iter().enumerate() {
        for (j, output) in input.iter().enumerate().skip(i + 1) {
            if output - val > 3 {
                break;
            }
            cache[j] += cache[i];
        }
    }

    cache.last().copied().unwrap_or(0)
}

#[aoc(day10, part2, functional)]
pub fn solve_part2_func(input: &[usize]) -> usize {
    let input = [vec![0], input.to_vec()].concat();
    let mut acc = vec![0; input.len()];
    acc[0] = 1;

    input
        .iter()
        .enumerate()
        .fold(acc, |acc, (i, v)| {
            input[i + 1..(i + 4).min(input.len())]
                .iter()
                .enumerate()
                .filter_map(|(j, x)| if x - v > 3 { None } else { Some(j + i + 1) })
                .fold(acc, |mut acc, j| {
                    acc[j] += acc[i];
                    acc
                })
        })
        .last()
        .copied()
        .unwrap_or(0)
}
