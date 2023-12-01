use itertools::Itertools;
use lazy_static::lazy_static;
use std::{collections::HashMap, io};

fn iter(map: &HashMap<(i64, i64, i64), bool>) -> HashMap<(i64, i64, i64), bool> {
    let mut new = map.clone();

    let mut minx = 0;
    let mut maxx = 0;
    let mut miny = 0;
    let mut maxy = 0;
    let mut minz = 0;
    let mut maxz = 0;

    for (&coord, _) in map {
        minx = minx.min(coord.0);
        maxx = maxx.max(coord.0);
        miny = miny.min(coord.1);
        maxy = maxy.max(coord.1);
        minz = minz.min(coord.2);
        maxz = maxz.max(coord.2);
    }

    for x in minx - 1..=maxx + 1 {
        for y in miny - 1..=maxy + 1 {
            for z in minz - 1..=maxz + 1 {
                let coord = (x, y, z);
                let active = map.get(&coord).unwrap_or(&false);
                let neigh = adjacent(coord)
                    .into_iter()
                    .filter_map(|c| map.get(&c))
                    .filter(|c| **c)
                    .count();

                let new_act = match active {
                    true if neigh == 2 || neigh == 3 => true,
                    true => false,
                    false if neigh == 3 => true,
                    false => false,
                };

                new.insert(coord, new_act);
            }
        }
    }

    new
}

fn iter4(map: &HashMap<(i64, i64, i64, i64), bool>) -> HashMap<(i64, i64, i64, i64), bool> {
    let mut new = map.clone();

    let mut minx = 0;
    let mut maxx = 0;
    let mut miny = 0;
    let mut maxy = 0;
    let mut minz = 0;
    let mut maxz = 0;
    let mut minw = 0;
    let mut maxw = 0;

    for (&coord, _) in map {
        minx = minx.min(coord.0);
        maxx = maxx.max(coord.0);
        miny = miny.min(coord.1);
        maxy = maxy.max(coord.1);
        minz = minz.min(coord.2);
        maxz = maxz.max(coord.2);
        minw = minw.min(coord.3);
        maxw = maxw.max(coord.3);
    }

    for x in minx - 1..=maxx + 1 {
        for y in miny - 1..=maxy + 1 {
            for z in minz - 1..=maxz + 1 {
                for w in minw - 1..=maxw + 1 {
                    let coord = (x, y, z, w);
                    let active = map.get(&coord).unwrap_or(&false);
                    let neigh = adjacent4(coord)
                        .into_iter()
                        .filter_map(|c| map.get(&c))
                        .filter(|c| **c)
                        .count();

                    let new_act = match active {
                        true if neigh == 2 || neigh == 3 => true,
                        true => false,
                        false if neigh == 3 => true,
                        false => false,
                    };

                    new.insert(coord, new_act);
                }
            }
        }
    }

    new
}

fn part1(map: &HashMap<(i64, i64, i64), bool>) -> i64 {
    let mut map = map.clone();
    for _ in 0..6 {
        map = iter(&map);
    }

    map.into_iter().filter(|(_, x)| *x).count() as i64
}

fn part2(map: &HashMap<(i64, i64, i64), bool>) -> i64 {
    let mut map: HashMap<(i64, i64, i64, i64), bool> = map
        .into_iter()
        .map(|((x, y, z), v)| ((*x, *y, *z, 0), *v))
        .collect();

    for _ in 0..6 {
        map = iter4(&map);
        //print0(&map);
    }

    map.into_iter().filter(|(_, x)| *x).count() as i64
}

fn adjacent((x, y, z): (i64, i64, i64)) -> Vec<(i64, i64, i64)> {
    lazy_static! {
        static ref DIRS: [(i64, i64, i64); 26] = [
            (0, 0, 1),
            (0, 0, -1),
            (0, 1, 0),
            (0, 1, 1),
            (0, 1, -1),
            (0, -1, 0),
            (0, -1, 1),
            (0, -1, -1),
            (1, 0, 0),
            (1, 0, 1),
            (1, 0, -1),
            (1, 1, 0),
            (1, 1, 1),
            (1, 1, -1),
            (1, -1, 0),
            (1, -1, 1),
            (1, -1, -1),
            (-1, 0, 0),
            (-1, 0, 1),
            (-1, 0, -1),
            (-1, 1, 0),
            (-1, 1, 1),
            (-1, 1, -1),
            (-1, -1, 0),
            (-1, -1, 1),
            (-1, -1, -1),
        ];
    }

    DIRS.iter()
        .map(|(dx, dy, dz)| (x + dx, y + dy, z + dz))
        .collect_vec()
}

fn adjacent4((x, y, z, w): (i64, i64, i64, i64)) -> Vec<(i64, i64, i64, i64)> {
    lazy_static! {
        static ref DIRS: [(i64, i64, i64, i64); 80] = [
            (0, 0, 1, 0),
            (0, 0, -1, 0),
            (0, 1, 0, 0),
            (0, 1, 1, 0),
            (0, 1, -1, 0),
            (0, -1, 0, 0),
            (0, -1, 1, 0),
            (0, -1, -1, 0),
            (1, 0, 0, 0),
            (1, 0, 1, 0),
            (1, 0, -1, 0),
            (1, 1, 0, 0),
            (1, 1, 1, 0),
            (1, 1, -1, 0),
            (1, -1, 0, 0),
            (1, -1, 1, 0),
            (1, -1, -1, 0),
            (-1, 0, 0, 0),
            (-1, 0, 1, 0),
            (-1, 0, -1, 0),
            (-1, 1, 0, 0),
            (-1, 1, 1, 0),
            (-1, 1, -1, 0),
            (-1, -1, 0, 0),
            (-1, -1, 1, 0),
            (-1, -1, -1, 0),
            (0, 0, 0, 1),
            (0, 0, 1, 1),
            (0, 0, -1, 1),
            (0, 1, 0, 1),
            (0, 1, 1, 1),
            (0, 1, -1, 1),
            (0, -1, 0, 1),
            (0, -1, 1, 1),
            (0, -1, -1, 1),
            (1, 0, 0, 1),
            (1, 0, 1, 1),
            (1, 0, -1, 1),
            (1, 1, 0, 1),
            (1, 1, 1, 1),
            (1, 1, -1, 1),
            (1, -1, 0, 1),
            (1, -1, 1, 1),
            (1, -1, -1, 1),
            (-1, 0, 0, 1),
            (-1, 0, 1, 1),
            (-1, 0, -1, 1),
            (-1, 1, 0, 1),
            (-1, 1, 1, 1),
            (-1, 1, -1, 1),
            (-1, -1, 0, 1),
            (-1, -1, 1, 1),
            (-1, -1, -1, 1),
            (0, 0, 0, -1),
            (0, 0, 1, -1),
            (0, 0, -1, -1),
            (0, 1, 0, -1),
            (0, 1, 1, -1),
            (0, 1, -1, -1),
            (0, -1, 0, -1),
            (0, -1, 1, -1),
            (0, -1, -1, -1),
            (1, 0, 0, -1),
            (1, 0, 1, -1),
            (1, 0, -1, -1),
            (1, 1, 0, -1),
            (1, 1, 1, -1),
            (1, 1, -1, -1),
            (1, -1, 0, -1),
            (1, -1, 1, -1),
            (1, -1, -1, -1),
            (-1, 0, 0, -1),
            (-1, 0, 1, -1),
            (-1, 0, -1, -1),
            (-1, 1, 0, -1),
            (-1, 1, 1, -1),
            (-1, 1, -1, -1),
            (-1, -1, 0, -1),
            (-1, -1, 1, -1),
            (-1, -1, -1, -1),
        ];
    }

    DIRS.iter()
        .map(|(dx, dy, dz, dw)| (x + dx, y + dy, z + dz, w + dw))
        .collect_vec()
}

fn parse(inp: &Vec<String>) -> HashMap<(i64, i64, i64), bool> {
    let z = 0i64;

    inp.iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i64, y as i64, z), c == '#'))
        })
        .collect()
}

fn main() -> io::Result<()> {
    let input = aoc2020::read_stdin_lines()?;
    let input = parse(&input);

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
