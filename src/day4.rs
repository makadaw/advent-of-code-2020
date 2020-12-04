use std::collections::HashSet;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Vec<String>> {
    input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.replace('\n', " "))
        .map(|s| s.split(' ').map(String::from).collect())
        .collect()
}

fn is_passport(pass: &[String]) -> bool {
    pass.len() == 8 || (pass.len() == 7 && !pass.iter().any(|s| s.contains("cid")))
}

#[aoc(day4, part1)]
pub fn solve1(input: &[Vec<String>]) -> usize {
    input
        .iter()
        .filter(|p| is_passport(p))
        .count()
}

fn is_valid_height(height: &str) -> bool {
    match &height[height.len()-2..] {
        "cm" => (150..=193).contains(&height[0..height.len()-2].parse::<usize>().unwrap_or(0)),
        "in" => (59..=76).contains(&height[0..height.len()-2].parse::<usize>().unwrap_or(0)),
        _ => false
    }
}

fn is_valid_passport(pass: &[String]) -> bool {
    // Now we need to validate fields
    let colors: HashSet<&'static str> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().cloned().collect();
    pass
        .iter()
        .all(|f| {
            let split: Vec<&str> = f.split(':').collect();
            let val = split[1];
            match split[0] {
                "byr" => (1920..=2002).contains(&val.parse::<usize>().unwrap_or(0)),
                "iyr" => (2010..=2020).contains(&val.parse::<usize>().unwrap_or(0)),
                "eyr" => (2020..=2030).contains(&val.parse::<usize>().unwrap_or(0)),
                "hgt" => is_valid_height(&val),
                "hcl" => val.len() == 7 && val.chars().nth(0).unwrap_or('q') == '#' && val[1..].chars().into_iter().all(|s| ('0'..='f').contains(&s)),
                "ecl" => colors.contains(val),
                "pid" => val.len() == 9 && val.chars().all(|s| ('0'..='9').contains(&s)),
                "cid" => true,
                _ => false
            }
        })
}

#[aoc(day4, part2)]
pub fn solve2(input: &[Vec<String>]) -> usize {
    input
        .iter()
        .filter(|p| is_passport(p) && is_valid_passport(p))
        .count()
}
