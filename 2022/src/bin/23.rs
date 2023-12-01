use itertools::Itertools;
use ndarray::Array2;
use std::{collections::HashSet, io};

type Coord = (i32, i32);

fn try_move_north(c: Coord, elves: &HashSet<Coord>) -> Option<Coord> {
    let n = (c.0 - 1, c.1);
    let nw = (c.0 - 1, c.1 - 1);
    let ne = (c.0 - 1, c.1 + 1);
    if !elves.contains(&n) && !elves.contains(&nw) && !elves.contains(&ne) {
        Some(n)
    } else {
        None
    }
}

fn try_move_south(c: Coord, elves: &HashSet<Coord>) -> Option<Coord> {
    let s = (c.0 + 1, c.1);
    let sw = (c.0 + 1, c.1 - 1);
    let se = (c.0 + 1, c.1 + 1);
    if !elves.contains(&s) && !elves.contains(&sw) && !elves.contains(&se) {
        Some(s)
    } else {
        None
    }
}

fn try_move_west(c: Coord, elves: &HashSet<Coord>) -> Option<Coord> {
    let w = (c.0, c.1 - 1);
    let nw = (c.0 - 1, c.1 - 1);
    let sw = (c.0 + 1, c.1 - 1);
    if !elves.contains(&w) && !elves.contains(&nw) && !elves.contains(&sw) {
        Some(w)
    } else {
        None
    }
}

fn try_move_east(c: Coord, elves: &HashSet<Coord>) -> Option<Coord> {
    let e = (c.0, c.1 + 1);
    let ne = (c.0 - 1, c.1 + 1);
    let se = (c.0 + 1, c.1 + 1);
    if !elves.contains(&e) && !elves.contains(&ne) && !elves.contains(&se) {
        Some(e)
    } else {
        None
    }
}

fn any_around(c: Coord, elves: &HashSet<Coord>) -> bool {
    (c.0 - 1..=c.0 + 1)
        .cartesian_product(c.1 - 1..=c.1 + 1)
        .filter(|cc| cc != &c)
        .any(|cc| elves.contains(&cc))
}

fn part1(input: &[Coord]) -> i32 {
    let mut elves: HashSet<Coord> = input.iter().copied().collect();

    let mfs = [try_move_north, try_move_south, try_move_west, try_move_east];

    for i in 0..10 {
        let mut proposals: Vec<(Coord, Coord)> = Vec::with_capacity(elves.len());
        let mut skips = vec![];

        for e in &elves {
            if !any_around(*e, &elves) {
                continue;
            }

            for mf in mfs.iter().cycle().skip(i % 4).take(4) {
                if let Some(m) = mf(*e, &elves) {
                    if proposals.iter().any(|p| p.1 == m) {
                        skips.push(m);
                    } else {
                        proposals.push((*e, m));
                    }
                    break;
                }
            }
        }

        for p in proposals.into_iter().filter(|p| !skips.contains(&p.1)) {
            elves.remove(&p.0);
            elves.insert(p.1);
        }
    }

    let (miny, maxy) = elves.iter().map(|e| e.0).minmax().into_option().unwrap();
    let (minx, maxx) = elves.iter().map(|e| e.1).minmax().into_option().unwrap();

    (maxy - miny + 1) * (maxx - minx + 1) - elves.len() as i32
}

fn part2(input: &[Coord]) -> usize {
    let mut elves: HashSet<Coord> = input.iter().copied().collect();
    let mut proposals: Vec<(Coord, Coord)> = Vec::with_capacity(elves.len());
    let mut targets: HashSet<Coord> = HashSet::with_capacity(elves.len());
    let mut skips: HashSet<Coord> = HashSet::new();

    let mfs = [try_move_north, try_move_south, try_move_west, try_move_east];

    for i in 0.. {
        proposals.clear();
        targets.clear();
        skips.clear();

        for e in &elves {
            if !any_around(*e, &elves) {
                continue;
            }

            for mf in mfs.iter().cycle().skip(i % 4).take(4) {
                if let Some(m) = mf(*e, &elves) {
                    if targets.contains(&m) {
                        skips.insert(m);
                    } else {
                        proposals.push((*e, m));
                        targets.insert(m);
                    }
                    break;
                }
            }
        }

        if targets.difference(&skips).count() == 0 {
            return i + 1;
        }

        for p in proposals.iter().filter(|p| !skips.contains(&p.1)) {
            elves.remove(&p.0);
            elves.insert(p.1);
        }
    }

    unreachable!()
}

fn parse_input(input: Array2<char>) -> Vec<Coord> {
    input
        .indexed_iter()
        .filter(|(_, c)| **c == '#')
        .map(|(c, _)| (c.0.try_into().unwrap(), c.1.try_into().unwrap()))
        .collect()
}

fn main() -> io::Result<()> {
    let input = parse_input(aoclib::read_input_char_matrix()?);

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_char_matrix(aoclib::get_test_input_file!(23)).unwrap();
        let input = parse_input(input);

        let p1 = part1(&input);
        assert_eq!(p1, 4091);

        let p2 = part2(&input);
        assert_eq!(p2, 1036);
    }
}
