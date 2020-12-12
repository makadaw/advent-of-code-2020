#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Ship {
    heading: isize,
    x: isize,
    y: isize,
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &str) -> isize {
    let ship = input
        .lines()
        .flat_map(|c| c[1..].parse::<isize>().map(|n| (&c[..1], n)))
        .fold(
            Ship {
                heading: 90,
                x: 0,
                y: 0,
            },
            |mut pos, (cmd, num)| {
                match cmd {
                    "N" => pos.y += num,
                    "E" => pos.x += num,
                    "S" => pos.y -= num,
                    "W" => pos.x -= num,
                    "R" => pos.heading = (pos.heading + num) % 360,
                    "L" => pos.heading = (pos.heading - num) % 360,
                    "F" => match pos.heading {
                        0 => pos.y += num,
                        90 | -270 => pos.x += num,
                        180 | -180 => pos.y -= num,
                        270 | -90 => pos.x -= num,
                        _ => {}
                    },
                    _ => panic!("Invalid command"),
                }
                pos
            },
        );
    ship.x.abs() + ship.y.abs()
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &str) -> isize {
    let position = input
        .lines()
        .flat_map(|c| c[1..].parse::<isize>().map(|n| (&c[..1], n)))
        .fold(
            (
                Ship {
                    heading: 90,
                    x: 0,
                    y: 0,
                },
                Ship {
                    heading: 0,
                    x: 1,
                    y: 10,
                },
            ),
            |mut pos, (cmd, num)| {
                match cmd {
                    "N" => pos.1.x += num,
                    "E" => pos.1.y += num,
                    "S" => pos.1.x -= num,
                    "W" => pos.1.y -= num,
                    "L" => {
                        let mut x = pos.1.x;
                        let mut y = pos.1.y;
                        (0..(num / 90)).for_each(|_| {
                            std::mem::swap(&mut x, &mut y);
                            y *= -1;
                        });
                        pos.1.x = x;
                        pos.1.y = y;
                    }
                    "R" => {
                        let mut x = pos.1.x;
                        let mut y = pos.1.y;
                        (0..(num / 90)).for_each(|_| {
                            std::mem::swap(&mut x, &mut y);
                            x *= -1;
                        });
                        pos.1.x = x;
                        pos.1.y = y;
                    }
                    "F" => {
                        pos.0.x += pos.1.x * num;
                        pos.0.y += pos.1.y * num
                    }
                    _ => panic!("Invalid command"),
                }
                pos
            },
        );
    position.0.x.abs() + position.0.y.abs()
}
