use std::collections::HashSet;

type Deck = Vec<usize>;

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> (Deck, Deck) {
    let decks: Vec<Deck> = input
        .split("\n\n")
        .map(|d| d.lines().skip(1).flat_map(str::parse::<usize>).collect())
        .collect();
    (decks[0].to_vec(), decks[1].to_vec())
}

fn game(deck1: &mut Vec<usize>, deck2: &mut Vec<usize>, with_recursive: bool) -> usize {
    let mut infinite = false;
    let mut mem: HashSet<Vec<usize>> = HashSet::new();
    while !deck1.is_empty() && !deck2.is_empty() {
        if with_recursive && mem.contains(&deck1.to_vec()) || mem.contains(&deck2.to_vec()) {
            infinite = true;
            break;
        }
        if with_recursive {
            mem.insert(deck1.to_vec());
            mem.insert(deck2.to_vec());
        }

        match (deck1.remove(0), deck2.remove(0)) {
            (a, b) if with_recursive && deck1.len() >= a && deck2.len() >= b => {
                // Rec game
                if game(
                    &mut deck1[..a].to_vec(),
                    &mut deck2[..b].to_vec(),
                    with_recursive,
                ) == 1
                {
                    deck1.push(a);
                    deck1.push(b);
                } else {
                    deck2.push(b);
                    deck2.push(a);
                }
            }

            (a, b) if a > b => {
                deck1.push(a);
                deck1.push(b);
            }
            (a, b) if a < b => {
                deck2.push(b);
                deck2.push(a);
            }
            _ => {}
        }
    }
    if infinite || deck2.is_empty() {
        1
    } else {
        2
    }
}

#[aoc(day22, part1)]
pub fn solve_part1(input: &(Deck, Deck)) -> usize {
    let (mut deck1, mut deck2) = input.clone();
    let winner = if game(&mut deck1, &mut deck2, false) == 1 {
        deck1
    } else {
        deck2
    };
    println!("{:?}", winner);
    winner
        .iter()
        .enumerate()
        .map(|(i, v)| v * (winner.len() - i))
        .sum()
}

#[aoc(day22, part2)]
pub fn solve_part2(input: &(Deck, Deck)) -> usize {
    let (mut deck1, mut deck2) = input.clone();
    let winner = if game(&mut deck1, &mut deck2, true) == 1 {
        deck1
    } else {
        deck2
    };
    println!("{:?}", winner);
    winner
        .iter()
        .enumerate()
        .map(|(i, v)| v * (winner.len() - i))
        .sum()
}
