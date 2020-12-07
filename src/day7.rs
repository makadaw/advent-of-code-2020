use std::collections::{HashMap, HashSet};

pub fn scan_bag(input: &str) -> Option<Vec<(usize, String)>> {
    match input {
        "no other bags." => None,
        _ => {
            let vec = input
                .split(", ")
                .flat_map(|s| scan_fmt!(s, "{d} {[ a-z]}", usize, String))
                .map(|(s, name)| (s, name.replace("bags", "bag")))
                .collect();
            Some(vec)
        }
    }
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> usize {
    let bags: HashMap<String, HashSet<String>> = input.lines().fold(HashMap::new(), |mut acc, l| {
        let mut split = l.split(" contain ");
        let bag = split.next().map(|s| s.replace("bags", "bag")).unwrap();
        if let Some(bags) = split.next().and_then(|s| scan_bag(s)) {
            bags.iter().for_each(|(_, name)| {
                acc.entry(name.to_string()).or_insert_with(HashSet::new).insert(bag.to_string());
            })
        }
        acc
    });

    let mut answer: HashSet<String> = HashSet::new();
    let mut stack = vec!["shiny gold bag"];
    while let Some(head) = stack.pop() {
        if let Some(inside) = bags.get(head) {
            inside.iter().for_each(|i| {
                if !answer.contains(i.as_str()) {
                    answer.insert(i.to_string());
                    stack.push(i);
                }
            });
        }
    }
    answer.len()
}

type Bags = HashMap<String, HashSet<(usize, String)>>;

fn calc(amount: usize, of: String, bags: &Bags) -> usize {
    match bags.get(&of) {
        Some(vec) => {
            let content: usize = vec.iter().map(|(n, inside)| calc(*n, inside.to_string(), &bags)).sum();
            content * amount + amount
        },
        None => amount
    }
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &str) -> usize {
    let bags: Bags = input
        .lines()
        .flat_map(|l| {
            let mut split = l.split(" contain ");
            let bag = split.next().map(|s| s.replace("bags", "bag")).unwrap();
            split.next().and_then(|s| scan_bag(s)).map(|s| s.into_iter().collect::<HashSet<(usize, String)>>()).map(|s| (bag, s))
        })
        .collect();
    calc(1, "shiny gold bag".to_string(), &bags) - 1
}
