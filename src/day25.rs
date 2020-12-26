const MOD: usize = 20_201_227;

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> (usize, usize) {
    let keys = input.lines().flat_map(str::parse).collect::<Vec<usize>>();
    (keys[0], keys[1])
}

#[aoc(day25, part1)]
pub fn solve_part1(input: &(usize, usize)) -> usize {
    let (card_pub, door_pub) = *input;
    let mut card_seed = 0;
    let mut key = 1;
    while key != card_pub {
        card_seed += 1;
        key = (key * 7) % MOD;
    }
    (0..card_seed).fold(1, |key, _| {
        (key * door_pub) % MOD
    })
}

#[aoc(day25, part1, in_one_go)]
pub fn solve_part1_1(input: &(usize, usize)) -> usize {
    let (card_pub, door_pub) = *input;
    let mut value = 1;
    let mut key = 1;
    while value != card_pub {
        value = (value * 7) % MOD; // This is a termination condition
        key = (key * door_pub) % MOD;
    }
    key
}
