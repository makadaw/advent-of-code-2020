use std::collections::{BTreeMap, HashSet};

struct Meal<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}

pub fn solve(input: &str) -> (usize, Vec<String>) {
    let mut meals: Vec<Meal> = input
        .lines()
        .map(|s| -> Meal {
            let i = s.find('(').unwrap();
            Meal {
                ingredients: s[..(i - 1)].split(' ').collect(),
                allergens: s[(i + 10)..(s.len() - 1)].split(", ").collect(),
            }
        })
        .collect();

    let mut unknown_allergens: HashSet<&str> = meals
        .iter()
        .map(|f| f.allergens.iter())
        .flatten()
        .cloned()
        .collect();
    let mut known_allergens: BTreeMap<&str, &str> = BTreeMap::new();

    while !unknown_allergens.is_empty() {
        let j = unknown_allergens.len();
        unknown_allergens = unknown_allergens
            .into_iter()
            .filter(|u| {
                let foods_with_this_allergen: Vec<&Meal> =
                    meals.iter().filter(|f| f.allergens.contains(u)).collect();
                let candidate_ingredients: HashSet<&str> = foods_with_this_allergen
                    .iter()
                    .skip(1)
                    .fold(foods_with_this_allergen[0].ingredients.clone(), |i, f| {
                        i.intersection(&f.ingredients).cloned().collect()
                    });

                if candidate_ingredients.len() == 1 {
                    let ingredient = candidate_ingredients.into_iter().next().unwrap();
                    for f in meals.iter_mut() {
                        f.ingredients.remove(ingredient);
                    }
                    known_allergens.insert(u, ingredient);
                    false
                } else {
                    true
                }
            })
            .collect();
        assert!(j != unknown_allergens.len());
    }

    let p1 = meals
        .iter()
        .map(|food| food.ingredients.len())
        .sum::<usize>();
    let p2 = known_allergens.values().map(|s| String::from(*s)).collect();

    (p1, p2)
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &str) -> usize {
    let solution = solve(input);
    solution.0
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &str) -> usize {
    println!("{}", solve(input).1.join(","));
    0
}
