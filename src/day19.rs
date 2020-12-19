use pathfinding::prelude::dfs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Check {
    Or(Box<Check>, Box<Check>), // Or rule
    Refs(Vec<usize>),           // Refs on other rules
    Char(char),                 // One char rule
}

type Input = (Vec<Check>, Vec<Vec<char>>);

fn parse_rule(line: &str) -> (usize, Check) {
    let rule: Vec<&str> = line.split(": ").collect();
    let idx: usize = rule[0].parse::<usize>().unwrap();
    let rule = rule[1];
    let result: Check = match rule {
        "\"a\"" => Check::Char('a'),
        "\"b\"" => Check::Char('b'),
        _ => {
            let or: Vec<Check> = rule
                .split(" | ")
                .map(|p| Check::Refs(p.split(' ').flat_map(|i| i.parse::<usize>()).collect()))
                .collect();
            if or.len() > 1 {
                Check::Or(Box::new(or[0].clone()), Box::new(or[1].clone()))
            } else {
                or[0].clone()
            }
        }
    };
    (idx, result)
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Input {
    let (mut rules, candidates) =
        input
            .lines()
            .fold((vec![], vec![]), |(mut rules, mut lines), l| {
                if !l.is_empty() {
                    match l.chars().next().unwrap() {
                        '0'..='9' => rules.push(parse_rule(l)),
                        _ => lines.push(l.chars().collect()),
                    }
                }
                (rules, lines)
            });
    rules.sort_by_key(|x| x.0);
    (rules.iter().map(|x| x.1.clone()).collect(), candidates)
}

fn unbox<T>(value: Box<T>) -> T {
    *value
}

// Ugly dfs
fn validate(line: &[char], rules: &[Check]) -> bool {
    dfs(
        (vec![rules[0].clone()], 0),
        |(pending_rules, index)| {
            if let Some((head, tail)) = pending_rules.split_first() {
                match head {
                    Check::Char(c) if line.get(*index) == Some(c) => {
                        vec![(tail.to_vec(), index + 1)]
                    }
                    Check::Char(_) => vec![],
                    Check::Refs(rs) => {
                        let mut new: Vec<Check> = rs
                            .iter()
                            .flat_map(|i| rules.get(*i))
                            .map(|r| r.clone())
                            .collect();
                        new.extend(tail.to_vec());
                        vec![(new, *index)]
                    }
                    Check::Or(a, b) => {
                        let mut a_vec = vec![unbox(a.clone())];
                        a_vec.extend(tail.to_vec());
                        let mut b_vec = vec![unbox(b.clone())];
                        b_vec.extend(tail.to_vec());
                        vec![(a_vec, *index), (b_vec, *index)]
                    }
                }
            } else {
                Vec::new()
            }
        },
        |(rules, index)| rules.is_empty() && *index == line.len(),
    )
    .is_some()
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &Input) -> usize {
    input
        .1
        .iter()
        .filter(|l| validate(l, &input.0))
        .collect::<Vec<_>>()
        .len()
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let mut rules = input.0.clone();
    rules[8] = parse_rule("8: 42 | 42 8").1;
    rules[11] = parse_rule("11: 42 31 | 42 11 31").1;
    input
        .1
        .iter()
        .filter(|l| validate(l, &rules))
        .collect::<Vec<_>>()
        .len()
}
