use std::{collections::HashMap, fmt::Display, str::FromStr};

use itertools::iproduct;

const IMAGE_TILES: usize = 12;
const IMAGE_SIZE: usize = IMAGE_TILES * 8;

const MONSTER: [&str; 3] = [
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   ",
];
const MONSTER_WAVES: usize = 15;
const MONSTER_HEIGHT: usize = MONSTER.len();
const MONSTER_WIDTH: usize = MONSTER[0].len();

#[derive(Debug, Default)]
struct Tile {
    id: u16,
    data: [u16; 10],
    borders: [u16; 8],
    neighbors: [Option<(usize, u16)>; 8],
    transform: usize,
}

fn flip_borders<T: Default + Copy>(data: &mut [T; 8]) {
    let mut new_data = [
        data[4], data[5], data[6], data[7], data[0], data[1], data[2], data[3],
    ];
    std::mem::swap(data, &mut new_data);
}

fn rotate_borders<T: Default + Copy>(data: &mut [T; 8]) {
    let mut new_data = [
        data[3], data[0], data[1], data[2], data[7], data[4], data[5], data[6],
    ];
    std::mem::swap(data, &mut new_data);
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .data
            .iter()
            .map(|row| {
                (0..10)
                    .map(|i| match ((row >> (9 - i)) & 1_u16) as u16 {
                        0 => '.',
                        1 => '#',
                        _ => unreachable!(),
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", s)
    }
}

impl Tile {
    // swap X and Y axis
    fn flip(&mut self) {
        let mut new_data = [0u16; 10];
        for y in 0..10 {
            for x in 0..10 {
                new_data[x] |= ((self.data[y] >> x) & 1) << y;
            }
        }
        std::mem::swap(&mut new_data, &mut self.data);
        flip_borders(&mut self.borders);
        flip_borders(&mut self.neighbors);
        self.transform ^= 4;
    }
    // rotate
    fn rotate(&mut self, steps: usize) {
        for _ in 0..(steps % 4) {
            let mut new_data = [0u16; 10];
            for y in 0..10 {
                for x in 0..10 {
                    new_data[x] |= ((self.data[y] >> x) & 1) << (9 - y);
                }
            }
            std::mem::swap(&mut new_data, &mut self.data);
            rotate_borders(&mut self.borders);
            rotate_borders(&mut self.neighbors);
        }
        self.transform = (self.transform & !3) | (self.transform + steps) & 3;
    }
    fn image_data(&self) -> impl Iterator<Item = u8> + '_ {
        self.data
            .iter()
            .skip(1)
            .take(8)
            .map(|row| ((row >> 1) & 255_u16) as u8)
    }
}

fn vertical_border(data: &[u16; 10], col: usize) -> u16 {
    data.iter()
        .fold(0u16, |border, line| border << 1 | ((line >> col) & 1))
}

fn rev_border(border: u16) -> u16 {
    (0..10).fold(0u16, |r, i| r << 1 | ((border >> i) & 1))
}

impl FromStr for Tile {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let id = lines.next().unwrap()[5..]
            .split(':')
            .next()
            .unwrap()
            .parse::<u16>()
            .unwrap();
        let data = lines
            .map(|line| {
                line.chars().fold(0u16, |data, c| {
                    data << 1
                        | match c {
                            '#' => 1,
                            _ => 0,
                        }
                })
            })
            .enumerate()
            .fold([0u16; 10], |mut arr, (i, data)| {
                arr[i] = data;
                arr
            });
        let borders = [
            // normal borders
            data[0],
            rev_border(vertical_border(&data, 9)),
            rev_border(data[9]),
            vertical_border(&data, 0),
            // flipped borders
            rev_border(vertical_border(&data, 0)),
            data[9],
            vertical_border(&data, 9),
            rev_border(data[0]),
        ];
        let neighbors = [None; 8];
        Ok(Self {
            id,
            data,
            borders,
            neighbors,
            transform: 0,
        })
    }
}

fn assemble_image(input: &str) -> Image {
    let mut tiles: HashMap<u16, Tile> = input
        .split("\n\n")
        .map(|block| block.parse::<Tile>().unwrap())
        .map(|tile| (tile.id, tile))
        .collect();
    let mut border_map: HashMap<u16, Vec<(u16, usize)>> = HashMap::new();
    for tile in tiles.values() {
        for (direction, border) in tile.borders.iter().enumerate() {
            border_map
                .entry(*border)
                .or_default()
                .push((tile.id, direction));
        }
    }
    for tiles_w_directions in border_map.values() {
        assert!(tiles_w_directions.len() <= 2);
        if tiles_w_directions.len() != 2 {
            continue;
        }
        let (tile0_id, dir0) = tiles_w_directions[0];
        let (tile1_id, dir1) = tiles_w_directions[1];
        let mut tile0 = tiles.get_mut(&tile0_id).unwrap();
        tile0.neighbors[dir0] = Some((dir1, tile1_id));
        let mut tile1 = tiles.get_mut(&tile1_id).unwrap();
        tile1.neighbors[dir1] = Some((dir0, tile0_id));
    }
    let mut map = [[0u16; IMAGE_TILES]; IMAGE_TILES];
    {
        let corner = tiles
            .values_mut()
            .find(|tile| tile.neighbors.iter().filter(|n| n.is_some()).count() == 4)
            .unwrap();
        // rotate first corner so that it has neighbors on the right (E)
        // and bottom (S).
        while corner.neighbors[1].is_none() || corner.neighbors[2].is_none() {
            corner.rotate(1);
        }
        map[0][0] = corner.id;
    }
    // fill first row
    for x in 0..IMAGE_TILES - 1 {
        let tile = &tiles[&map[0][x]];
        let (neighbor_transform, neighbor_id) = tile.neighbors[1].unwrap();
        let neighbor = tiles.get_mut(&neighbor_id).unwrap();
        map[0][x + 1] = neighbor_id;
        if neighbor_transform & 4 == 0 {
            neighbor.flip();
            neighbor.rotate(neighbor_transform % 4);
        } else {
            neighbor.rotate(neighbor_transform % 4);
        }
    }
    // fill other rows
    for y in 0..IMAGE_TILES - 1 {
        for x in 0..IMAGE_TILES {
            let tile = &tiles[&map[y][x]];
            let (neighbor_transform, neighbor_id) = tile.neighbors[2].unwrap();
            let neighbor = tiles.get_mut(&neighbor_id).unwrap();
            map[y + 1][x] = neighbor_id;
            if neighbor_transform & 4 == 0 {
                neighbor.flip();
                neighbor.rotate(neighbor_transform % 4 + 1);
            } else {
                neighbor.rotate(neighbor_transform % 4 + 1);
            }
        }
    }

    Image::from_tiles(&tiles, &map)
}

struct Image([u128; IMAGE_SIZE]);

impl Image {
    fn new() -> Self {
        Self([0u128; IMAGE_SIZE])
    }
    fn from_tiles(tiles: &HashMap<u16, Tile>, map: &[[u16; IMAGE_TILES]; IMAGE_TILES]) -> Self {
        let mut image = Self::new();

        for (y, y_val) in map.iter().enumerate() {
            for x in (0..IMAGE_TILES).rev() {
                let tile = tiles.get(&y_val[x]).unwrap();
                for (i, data) in tile.image_data().enumerate() {
                    image.0[y * 8 + i] <<= 8;
                    image.0[y * 8 + i] |= data as u128;
                }
            }
        }
        image
    }
    fn rotate(&self) -> Self {
        let mut new_image = Self::new();
        for y in 0..IMAGE_SIZE {
            for x in 0..IMAGE_SIZE {
                new_image.0[x] |= ((self.0[y] >> x) & 1) << (IMAGE_SIZE - y - 1);
            }
        }
        new_image
    }
    fn count_monsters(&self, monster: &[u128; MONSTER_HEIGHT]) -> usize {
        iproduct!(
            0..=(IMAGE_SIZE - MONSTER_HEIGHT),
            0..=(IMAGE_SIZE - MONSTER_WIDTH)
        )
        .filter(|(y, x)| {
            (0..MONSTER_HEIGHT).all(|row| (self.0[y + row] >> x) & monster[row] == monster[row])
        })
        .count()
    }
    fn count_waves(&self) -> usize {
        self.0
            .iter()
            .map(|row| (0..IMAGE_SIZE).filter(|i| (row >> i) & 1 == 1).count())
            .sum::<usize>()
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .0
            .iter()
            .map(|row| {
                (0..IMAGE_SIZE)
                    .map(|i| match (row >> (IMAGE_SIZE - i - 1)) & 1 {
                        0 => '.',
                        1 => '#',
                        _ => unreachable!(),
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", s)
    }
}

fn prepare_monster() -> [u128; MONSTER_HEIGHT] {
    let mut monster = [0u128; MONSTER_HEIGHT];
    for y in 0..MONSTER_HEIGHT {
        let row = &mut monster[y];
        for x in 0..MONSTER[y].len() {
            *row <<= 1;
            *row |= match MONSTER[y].chars().nth(x).unwrap() {
                '#' => 1,
                _ => 0,
            }
        }
    }
    monster
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &str) -> usize {
    let tiles: Vec<Tile> = input
        .split("\n\n")
        .map(|block| block.parse::<Tile>().unwrap())
        .collect();
    let mut borders: HashMap<u16, Vec<u16>> = HashMap::new();
    for tile in tiles {
        for border in tile.borders.iter() {
            borders
                .entry(*border)
                .and_modify(|t| t.push(tile.id))
                .or_insert_with(|| vec![tile.id]);
        }
    }
    let mut found: HashMap<u16, usize> = HashMap::new();
    for (_, tile_ids) in borders.iter() {
        if tile_ids.len() == 2 {
            for tile_id in tile_ids {
                found
                    .entry(*tile_id)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
    }
    found
        .iter()
        .filter(|(_, count)| **count == 4)
        .map(|(tile_id, _)| *tile_id as usize)
        .product::<usize>()
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut image = assemble_image(input);
    // println!("{}", image);
    let monster = prepare_monster();
    let monster_rev = [monster[2], monster[1], monster[0]];
    let mut monster_count = 0;
    for _ in 0..4 {
        monster_count += image.count_monsters(&monster);
        monster_count += image.count_monsters(&monster_rev);
        image = image.rotate();
    }
    let image_waves = image.count_waves();
    image_waves - monster_count * MONSTER_WAVES
}
