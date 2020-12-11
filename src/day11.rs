use std::collections::HashMap;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Field {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let field: Field = map.iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().flat_map(move |(j, seat)| Seat::from(*seat).map(|s| (Point {x: i as isize, y: j as isize}, s)) ))
        .collect();
    field
}

#[derive(Debug, Clone, PartialEq)]
pub enum Seat {
    Empty,
    Occupied
}

impl Seat {
    fn from(str: char) -> Option<Seat> {
        match str {
            'L' => Some(Seat::Empty),
            '#' => Some(Seat::Occupied),
            _ => None
        }
    }
}
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, PartialOrd, Ord)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

type Field = HashMap<Point<isize>, Seat>;

#[aoc(day11, part1)]
pub fn solve_part1(input: &Field) -> usize {
    let mut map: Field = input.clone();
    let adj = vec![(-1,-1),(-1, 0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];

    let mut changed = true;
    while changed {
        let next: Field = map
            .iter()
            .map(|(k, val)| {
                let mut around = adj.iter()
                    .map(|p| Point{x: p.0 + k.x, y: p.1 + k.y})
                    .flat_map(|p| map.get(&p))
                    ;
                let new = match val {
                    Seat::Empty => if around.any(|s| s == &Seat::Occupied) { Seat::Empty } else { Seat::Occupied },
                    Seat::Occupied => if around.filter(|s| *s == &Seat::Occupied).count() >= 4 { Seat::Empty } else { Seat::Occupied }, 
                };
                (*k, new)
            })
            .collect();
        if next != map {
            map = next;
        } else {
            changed = false;
        }
    }
    map
        .iter()
        .filter(|(_, val)| *val == &Seat::Occupied)
        .count()
}

fn visible_occupied(from: &Point<isize>, map: &Field, vector: &[(isize, isize)]) -> Vec<Seat> {
    vector
        .iter()
        .flat_map(|v| {
            for m in 1..=9 {
                let point = Point{x: from.x + m * v.0, y: from.y + m * v.1};
                if let Some(seat) = map.get(&point) {
                    return Some(seat.clone());
                }
            };
            None
        })
        .collect()
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Field) -> usize {
    let mut map: Field = input.clone();
    let adj = vec![(-1,-1),(-1, 0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];

    let mut changed = true;
    while changed {
        let next: Field = map
            .iter()
            .map(|(k, val)| {
                let around = visible_occupied(k, &map, &adj);
                let new = match val {
                    Seat::Empty => if around.iter().any(|s| s == &Seat::Occupied) { Seat::Empty } else { Seat::Occupied },
                    Seat::Occupied => if around.iter().filter(|s| *s == &Seat::Occupied).count() >= 5 { Seat::Empty } else { Seat::Occupied }, 
                };
                (*k, new)
            })
            .collect();
        if next != map {
            map = next;
        } else {
            changed = false;
        }
    }
    map
        .iter()
        .filter(|(_, val)| *val == &Seat::Occupied)
        .count()
}

