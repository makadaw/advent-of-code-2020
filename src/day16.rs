use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Field {
    name: String,
    ranges: Vec<RangeInclusive<usize>>,
}

impl Field {
    fn from(string: &str) -> Field {
        let (name, s1, e1, s2, e2) = scan_fmt!(
            string,
            "{[a-z ]}: {d}-{d} or {d}-{d}",
            String,
            usize,
            usize,
            usize,
            usize
        )
        .unwrap();
        Field {
            name,
            ranges: vec![RangeInclusive::new(s1, e1), RangeInclusive::new(s2, e2)],
        }
    }

    fn contains(&self, value: usize) -> bool {
        self.ranges.iter().any(|r| r.contains(&value))
    }
}

type Input = (Vec<Field>, Vec<usize>, Vec<Vec<usize>>);

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Input {
    let mut fields = vec![];
    let mut my_ticket = vec![];
    let mut tickets = vec![];
    let mut part = 0;
    for line in input.lines() {
        if line.is_empty() {
            part += 1;
        } else {
            match part {
                0 => fields.push(Field::from(&line)),
                1 | 3 => part += 1,
                2 => my_ticket = line.split(',').flat_map(|c| c.parse::<usize>()).collect(),
                _ => tickets.push(line.split(',').flat_map(|c| c.parse::<usize>()).collect()),
            }
        }
    }
    (fields, my_ticket, tickets)
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let (fields, _, tickets) = input;
    tickets
        .iter()
        .flat_map(|t| -> Vec<usize> {
            t.iter()
                .flat_map(|v| {
                    if fields.iter().any(|f| f.contains(*v)) {
                        None
                    } else {
                        Some(*v)
                    }
                })
                .collect()
        })
        .sum()
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let (fields, my_ticket, tickets) = input;
    let fields_map: Vec<usize> = tickets
        .iter()
        // Filter valid ticket
        .filter(|ticket| {
            ticket
                .iter()
                .all(|value| fields.iter().any(|field| field.contains(*value)))
        })
        .map(|ticket| {
            // Map tickets field into mask
            ticket
                .iter()
                .map(|value| {
                    fields.iter().fold(0, |acc, field| {
                        if field.contains(*value) {
                            (acc << 1) | 1
                        } else {
                            acc << 1
                        }
                    })
                })
                .collect()
        })
        // Create a field occurrence mask across all tickets
        .fold(
            vec![std::usize::MAX; fields.len()],
            |acc, ticket: Vec<usize>| acc.iter().zip(ticket.iter()).map(|(a, t)| a & t).collect(),
        );
    // Create a vec of tuple with field index and mask
    let mut fields_map: Vec<(usize, &usize)> = fields_map.iter().enumerate().collect();
    let len = fields.len();
    // Sort by amount of high bits in the field mask
    fields_map.sort_by(|a, b| a.1.count_ones().cmp(&b.1.count_ones()));

    fields_map
        .iter()
        // Go from more specific field to general, found a possible index
        .fold((0, 1), |(mask, mut product), (idx, &field)| {
            let intersection = mask ^ field;
            // Find index of the first high bit
            let field_id = len
                - 1
                - (0..intersection)
                    .find(|i| intersection >> i & 1 == 1)
                    .unwrap_or(0);
            if fields[field_id].name.starts_with("departure") {
                product *= my_ticket[*idx];
            }
            (mask | field, product)
        })
        .1
}
