use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

type Pair = (isize, isize);

impl Direction {
    fn from_pair(pair: (char, char)) -> Option<Direction> {
        match pair {
            ('s', 'e') => Some(Direction::SouthEast),
            ('s', 'w') => Some(Direction::SouthWest),
            ('n', 'e') => Some(Direction::NorthEast),
            ('n', 'w') => Some(Direction::NorthWest),
            ('e', _) => Some(Direction::East),
            ('w', _) => Some(Direction::West),
            _ => None,
        }
    }

    fn step(&self) -> Pair {
        match self {
            Direction::East => (2, 0),
            Direction::SouthEast => (1, 1),
            Direction::SouthWest => (-1, 1),
            Direction::West => (-2, 0),
            Direction::NorthWest => (-1, -1),
            Direction::NorthEast => (1, -1),
        }
    }

    fn all() -> Vec<Direction> {
        vec![
            Direction::East,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
            Direction::NorthEast,
        ]
    }
}

fn build_map(input: &[Vec<Direction>]) -> HashSet<Pair> {
    input.iter().fold(HashSet::new(), |mut acc, direct| {
        let pair = direct.iter().fold((0, 0), |acc, d| {
            let step = d.step();
            (acc.0 + step.0, acc.1 + step.1)
        });

        if acc.contains(&pair) {
            acc.remove(&pair)
        } else {
            acc.insert(pair)
        };
        acc
    })
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<Vec<Direction>> {
    input
        .lines()
        .map(|l| {
            let mut line: Vec<char> = l.chars().collect();
            line.insert(0, ' ');
            line.push(' ');
            line.windows(3)
                .filter_map(|w| match w[0] {
                    'e' | 'w' | ' ' => Some((w[1], w[2])),
                    _ => None,
                })
                .flat_map(Direction::from_pair)
                .collect()
        })
        .collect()
}

#[aoc(day24, part1)]
pub fn solve_part1(input: &[Vec<Direction>]) -> usize {
    build_map(input).len()
}

#[aoc(day24, part2)]
pub fn solve_part2(input: &[Vec<Direction>]) -> usize {
    (1..=100)
        .fold(build_map(input), |current, _| {
            let mut counter: HashMap<Pair, usize> = HashMap::new();
            // Build map of black neighbours
            for p in &current {
                Direction::all()
                    .iter()
                    .map(|to| to.step())
                    .map(|dif| (p.0 + dif.0, p.1 + dif.1))
                    .for_each(|p| {
                        counter.entry(p).and_modify(|e| *e += 1).or_insert(1);
                    })
            }
            // Flip white tiles
            let mut next: HashSet<Pair> = counter
                .iter()
                .filter(|(&p, &c)| c == 2 && !current.contains(&p))
                .map(|(&p, _)| p)
                .collect();
            // Flip blacks
            for p in &current {
                if counter.get(&p).map(|&c| c == 1 || c == 2).unwrap_or(false) {
                    next.insert(*p);
                }
            }
            next
        })
        .len()
}
