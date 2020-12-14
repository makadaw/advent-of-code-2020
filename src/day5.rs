trait BinaryMove {
    fn binary_move(&mut self, up: bool);
}

#[derive(Debug)]
struct Range {
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Ticket {
    row: usize,
    seat: usize,
    seat_id: usize,
}

impl Ticket {
    fn from(decoded: &str) -> Ticket {
        let mut row = Range { from: 0, to: 127 };
        let mut seat = Range { from: 0, to: 7 };
        decoded.chars().enumerate().for_each(|(idx, c)| {
            if idx < 7 {
                row.binary_move(c == 'B')
            } else {
                seat.binary_move(c == 'R')
            }
        });
        Ticket {
            row: row.from,
            seat: seat.from,
            seat_id: row.from * 8 + seat.from,
        }
    }
}

impl BinaryMove for Range {
    fn binary_move(&mut self, upper: bool) {
        let half = (((self.to - self.from) as f64) / 2f64).ceil() as usize;
        if upper {
            self.from += half;
        } else {
            self.to -= half;
        }
    }
}

#[aoc(day5, part1)]
pub fn solve1(input: &str) -> usize {
    input
        .lines()
        .map(Ticket::from)
        .map(|t| t.seat_id)
        .max()
        .unwrap_or(0)
}

#[aoc(day5, part1, binary)]
pub fn solve1_2(input: &str) -> usize {
    input
        .lines()
        .map(|s| {
            s.replace('F', "0")
                .replace('B', "1")
                .replace('L', "0")
                .replace('R', "1")
        })
        .flat_map(|s| usize::from_str_radix(s.as_str(), 2))
        .max()
        .unwrap_or(0)
}

#[aoc(day5, part2)]
pub fn solve2(input: &str) -> usize {
    let mut tickets = input.lines().map(Ticket::from).collect::<Vec<Ticket>>();
    tickets.sort_by(|a, b| b.seat_id.cmp(&a.seat_id));
    tickets
        .iter()
        .zip(tickets[1..].iter())
        .find(|(a, b)| a.seat_id - b.seat_id != 1)
        .map(|(a, _)| a.seat_id - 1)
        .unwrap_or(0)
}
