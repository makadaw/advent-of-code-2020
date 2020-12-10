use std::cmp::Ordering;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.lines().flat_map(|s| s.parse::<usize>()).collect()
}

fn sum_in_preamble(number: usize, preamble: &[usize]) -> bool {
    for (i, i_val) in preamble.iter().enumerate() {
        for j_val in preamble[i..].iter() {
            if i_val + j_val == number {
                return true;
            }
        }
    }
    false
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    let size = 25;
    let mut preamble = input[..size].to_vec();
    *input[size..]
        .iter()
        .find(|n| {
            let valid = sum_in_preamble(**n, &preamble);
            if valid {
                preamble.remove(0);
                preamble.push(**n);
            }
            !valid
        })
        .unwrap()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    let number = solve_part1(input);
    let mut i = 0;
    let mut j = 1;
    let mut sum = input[i] + input[j];
    while j < input.len() {
        match sum.cmp(&number) {
            Ordering::Less => {
                j += 1;
                sum += input[j];
            }
            Ordering::Greater => {
                sum -= input[i];
                i += 1;
            }
            Ordering::Equal => break,
        }
    }
    let values = input[i..=j]
        .iter()
        .fold((input[i], 0), |acc, n| (acc.0.min(*n), acc.1.max(*n)));
    values.0 + values.1
}
