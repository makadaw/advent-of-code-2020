use std::collections::HashSet;

type Instruction = (String, isize);

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .flat_map(|l| scan_fmt!(l, "{} {d}", String, isize))
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Instruction]) -> isize {
    let mut idx = 0;
    let mut acc = 0;
    let mut last: HashSet<usize> = HashSet::new();
    while last.get(&idx) == None {
        last.insert(idx);
        let instruction = &input[idx];
        match instruction.0.as_str() {
            "acc" => acc += instruction.1,
            "jmp" => idx = (idx as isize + instruction.1 - 1) as usize,
            _ => {}
        };
        idx += 1
    }
    acc
}

fn evaluate(input: &[Instruction]) -> Option<isize> {
    let mut idx = 0;
    let mut acc = 0;
    let mut last: HashSet<usize> = HashSet::new();
    while last.get(&idx) == None {
        last.insert(idx);
        match input.get(idx) {
            Some((instruction, i)) => match instruction.as_str() {
                "acc" => acc += i,
                "jmp" => idx = (idx as isize + i - 1) as usize,
                "nop" => {}
                _ => panic!("Not supported op!"),
            },
            // We are out of instruction
            None => return Some(acc),
        }
        idx += 1
    }
    // If we are here program was in the loop
    None
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[Instruction]) -> isize {
    let mut copy = input.to_vec();
    (0..input.len())
        .find_map(|i| {
            if input[i].0 != "acc" {
                let was = copy.remove(i);
                copy.insert(
                    i,
                    (
                        (if was.0 == "nop" { "jmp" } else { "nop" }).to_string(),
                        was.1,
                    ),
                );
                match evaluate(&copy) {
                    Some(a) => return Some(a),
                    None => {
                        copy.remove(i);
                        copy.insert(i, was);
                    }
                }
            };
            None
        })
        .unwrap_or(0)
}
