#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .flat_map(|s| s.parse::<u32>())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    let sum: u32 = 2020;

    for (i, i_val) in input.iter().enumerate() {
        for j_val in input[i..].iter() {
            if i_val + j_val == sum {
                let r = i_val * j_val;
                return r;
            }
        }
    }
    0
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    let sum: u32 = 2020;

    for (i, i_val) in input.iter().enumerate() {
        for (j, j_val) in input[i..].iter().enumerate() {
            if i_val + j_val < sum {
                for k_val in input[j..].iter() {
                    if i_val + j_val + k_val == sum {
                        return i_val * j_val * k_val;
                    }
                }
            }
        }
    }
    0
}
