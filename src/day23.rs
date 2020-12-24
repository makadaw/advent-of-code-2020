use itertools::Itertools;
use std::collections::HashMap;

struct Deck {
    next_map: HashMap<usize, usize>,
    current: usize,
    max: usize,
}

impl Deck {
    pub fn step(&mut self) {
        let a = self.next_map[&self.current];
        let b = self.next_map[&a];
        let c = self.next_map[&b];
        let new_current = self.next_map[&c];
        let mut destination = self.current - 1;
        if destination == 0 {
            destination = self.max
        };
        while [a, b, c].contains(&destination) {
            destination -= 1;
            if destination == 0 {
                destination = self.max
            };
        }
        let next = self.next_map[&destination];
        self.next_map.insert(self.current, new_current);
        self.next_map.insert(destination, a);
        self.next_map.insert(c, next);
        self.current = new_current;
    }

    pub fn make(seed: &[usize], max: usize) -> Self {
        let i: Vec<usize> = (1..=max)
            .map(|i| if i < seed.len() + 1 { seed[i - 1] } else { i })
            .collect();
        let mut next_map: HashMap<usize, usize> = i.iter().copied().tuple_windows().collect();
        next_map.insert(i[i.len() - 1], i[0]);

        Deck {
            next_map,
            current: i[0],
            max,
        }
    }

    pub fn run(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();
        }
    }

    pub fn iterate_from(&self, from: usize) -> DeckIter {
        DeckIter {
            g: self,
            cursor: from,
            start: from,
        }
    }
}

struct DeckIter<'a> {
    g: &'a Deck,
    cursor: usize,
    start: usize,
}

impl<'a> Iterator for DeckIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor = self.g.next_map[&self.cursor];
        if self.cursor == self.start {
            None
        } else {
            Some(self.cursor)
        }
    }
}

#[aoc_generator(day23)]
pub fn gen(input: &str) -> Vec<usize> {
    input
        .chars()
        .flat_map(|c| c.to_digit(10).map(|n| n as usize))
        .collect()
}

#[aoc(day23, part1)]
pub fn p1(input: &[usize]) -> String {
    let mut g = Deck::make(input, 9);
    g.run(100);
    g.iterate_from(1).map(|x| x.to_string()).join("")
}

#[aoc(day23, part2)]
pub fn p2(input: &[usize]) -> usize {
    let mut g = Deck::make(input, 1_000_000);
    g.run(10_000_000);
    g.iterate_from(1).take(2).product()
}
