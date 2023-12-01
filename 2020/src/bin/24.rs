use std::{
    collections::{HashMap, HashSet},
    io,
};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct HexCoord {
    x: i32,
    y: i32,
    z: i32,
}

struct HexCoordNeighbors<'a> {
    orig: &'a HexCoord,
    state: u8,
}

impl Iterator for HexCoordNeighbors<'_> {
    type Item = HexCoord;

    fn next(&mut self) -> Option<Self::Item> {
        let c = match self.state {
            0 => self.orig.east(),
            1 => self.orig.southeast(),
            2 => self.orig.southwest(),
            3 => self.orig.west(),
            4 => self.orig.northwest(),
            5 => self.orig.northeast(),
            _ => return None,
        };
        self.state += 1;
        Some(c)
    }
}

impl HexCoord {
    fn east(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y - 1,
            z: self.z,
        }
    }

    fn west(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
            z: self.z,
        }
    }

    fn northeast(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
            z: self.z - 1,
        }
    }

    fn southwest(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
            z: self.z + 1,
        }
    }

    fn northwest(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
            z: self.z - 1,
        }
    }

    fn southeast(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
            z: self.z + 1,
        }
    }

    fn neighbors(&self) -> HexCoordNeighbors {
        HexCoordNeighbors {
            orig: self,
            state: 0,
        }
    }
}

fn resolve(l: &str) -> HexCoord {
    let mut chars = l.trim().chars();
    let mut coord = HexCoord { x: 0, y: 0, z: 0 };
    while let Some(c) = chars.next() {
        coord = match c {
            'n' => match chars.next().unwrap() {
                'e' => coord.northeast(),
                'w' => coord.northwest(),
                _ => unreachable!(),
            },
            's' => match chars.next().unwrap() {
                'e' => coord.southeast(),
                'w' => coord.southwest(),
                _ => unreachable!(),
            },
            'e' => coord.east(),
            'w' => coord.west(),
            _ => unreachable!(),
        }
    }

    return coord;
}

fn part1(input: &Vec<String>) -> (usize, HashSet<HexCoord>) {
    let mut tiles = HashMap::new();
    for l in input {
        let c = resolve(l);
        let tile = tiles.entry(c).or_insert(false);
        *tile = !*tile;
    }

    (
        tiles.values().filter(|x| **x).count(),
        tiles
            .into_iter()
            .filter(|(_, x)| *x)
            .map(|(c, _)| c)
            .collect(),
    )
}

fn part2(mut tiles: HashSet<HexCoord>) -> usize {
    for _ in 0..100 {
        let mut new_tiles = HashSet::new();

        let coords: HashSet<HexCoord> = tiles
            .iter()
            .flat_map(|t| t.neighbors())
            .collect();

        for c in coords.union(&tiles) {
            let neighbour_count: u8 = c
                .neighbors()
                .map(|c| tiles.contains(&c) as u8)
                .sum();

            let tile = match tiles.contains(&c) {
                true if neighbour_count == 0 || neighbour_count > 2 => false,
                false if neighbour_count == 2 => true,
                t => t,
            };

            if tile {
                new_tiles.insert(c.clone());
            }
        }

        tiles = new_tiles;
    }

    tiles.len()
}

fn main() -> io::Result<()> {
    let input = aoc2020::read_stdin_lines()?;

    let (p1, tiles) = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(tiles);
    println!("Part 2: {}", p2);

    Ok(())
}
