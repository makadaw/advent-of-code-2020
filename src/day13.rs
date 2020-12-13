#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().take(2).map(String::from).collect()
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &[String]) -> usize {
    let time: usize = input[0].parse::<usize>().unwrap();
    let next = input[1]
        .split(',')
        .flat_map(|s| s.parse::<usize>())
        .map(|t| (t, time - time % t + t - time))
        .fold(
            (0, std::usize::MAX),
            |acc, x| if x.1 < acc.1 { x } else { acc },
        );
    next.0 * next.1
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &[String]) -> usize {
    input[1]
        .split(',')
        .enumerate()
        .flat_map(|(i, s)| s.parse::<usize>().map(|n| (i, n)))
        .fold((0, 1), |(solution, product), (idx, bus_id)| {
            let mut m = solution;
            while (m + idx) % bus_id != 0 {
                m += product;
            }
            (m, product * bus_id)
        })
        .0
}
