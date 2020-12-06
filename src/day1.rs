#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.lines().flat_map(|s| s.parse::<usize>()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    let sum: usize = 2020;

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

#[aoc(day1, part1, sorted)]
pub fn solve_part1_sorted(input: &[usize]) -> usize {
    let mut nums: Vec<usize> = input.to_vec();
    nums.sort();

    let mut a = 0;
    let mut b = nums.len() - 1;
    while nums[a] + nums[b] != 2020 && a < b {
        if nums[a] + nums[b] > 2020 {
            b -= 1;
        } else {
            a += 1;
        }
    }
    nums[a] * nums[b]
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    let sum: usize = 2020;

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
