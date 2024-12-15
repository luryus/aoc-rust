use aoclib::coord2::Coord2;
use itertools::Itertools;
use ndarray::{array, s, Array2};
use std::io;

fn move_thing(
    map: &mut Array2<char>,
    pos: Coord2<usize>,
    dir: Coord2<isize>,
) -> Option<Coord2<usize>> {
    let new_pos = pos.checked_add_with_upper(dir, map.dim())?;
    let thing = map[pos];

    if map[new_pos] == '.' || map[new_pos] == 'O' && move_thing(map, new_pos, dir).is_some() {
        map[new_pos] = thing;
        map[pos] = '.';
        Some(new_pos)
    } else {
        debug_assert!(map[new_pos] == '#' || map[new_pos] == 'O');
        None
    }
}

fn buddy_pos(dim: (usize, usize), thing: char, pos: Coord2<usize>) -> Option<Coord2<usize>> {
    match thing {
        '[' => Some(pos.checked_add_with_upper(Coord2::RIGHT, dim).unwrap()),
        ']' => Some(pos.checked_add_with_upper(Coord2::LEFT, dim).unwrap()),
        _ => None,
    }
}

fn move_thing2(
    map: &mut Array2<char>,
    pos: Coord2<usize>,
    dir: Coord2<isize>,
) -> Option<Coord2<usize>> {
    let new_pos = pos.checked_add_with_upper(dir, map.dim())?;
    let thing = map[pos];

    if let Some(bp) = buddy_pos(map.dim(), thing, pos) {
        let nbp = buddy_pos(map.dim(), thing, new_pos)?;
        let buddy_thing = map[bp];
        if map[nbp] == '.' || (map[nbp] != '#' && move_thing2(map, nbp, dir).is_some()) {
            map[bp] = '.';
            map[nbp] = buddy_thing;
        } else {
            return None;
        }
    }

    if map[new_pos] == '.' || (map[new_pos] != '#' && move_thing2(map, new_pos, dir).is_some()) {
        map[new_pos] = thing;
        map[pos] = '.';
        Some(new_pos)
    } else {
        None
    }
}

fn part1(map: &Array2<char>, dirs: &Vec<Coord2<isize>>) -> usize {
    let mut robot_pos: Coord2<usize> = map
        .indexed_iter()
        .find(|&(_, c)| *c == '@')
        .map(|(c, _)| c)
        .unwrap()
        .into();
    let mut map = map.clone();

    for d in dirs {
        if let Some(new_pos) = move_thing(&mut map, robot_pos, *d) {
            robot_pos = new_pos;
        }
    }

    map.indexed_iter()
        .filter_map(|((y, x), b)| match b {
            'O' => Some(y * 100 + x),
            _ => None,
        })
        .sum()
}

fn part2(small_map: &Array2<char>, dirs: &Vec<Coord2<isize>>) -> usize {
    let dim = (small_map.dim().0, small_map.dim().1 * 2);
    let mut map = Array2::from_elem(dim, '.');

    for ((y, x), c) in small_map.indexed_iter() {
        let mut slot = map.slice_mut(s![y, (x * 2)..(x * 2) + 2]);
        debug_assert_eq!(slot.dim(), 2);
        slot.assign(&match c {
            '#' => array!['#', '#'],
            'O' => array!['[', ']'],
            '.' => array!['.', '.'],
            '@' => array!['@', '.'],
            _ => unreachable!(),
        });
    }

    let mut robot_pos: Coord2<usize> = map
        .indexed_iter()
        .find(|&(_, c)| *c == '@')
        .map(|(c, _)| c)
        .unwrap()
        .into();

    for d in dirs {
        let mut new_map = map.clone();
        if let Some(new_pos) = move_thing2(&mut new_map, robot_pos, *d) {
            robot_pos = new_pos;
            map = new_map;
        }
    }

    map.indexed_iter()
        .filter_map(|((y, x), b)| match b {
            '[' => Some(y * 100 + x),
            _ => None,
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_string()?;
    let (map, dirs) = parse_input(input);

    let p1 = part1(&map, &dirs);
    println!("Part 1: {}", p1);

    let p2 = part2(&map, &dirs);
    println!("Part 2: {}", p2);

    Ok(())
}

fn parse_input(input: String) -> (Array2<char>, Vec<Coord2<isize>>) {
    let (map, dirs) = aoclib::split_to_tuple2(&input, "\n\n").unwrap();
    let map = aoclib::read_string_char_matrix(map).unwrap();
    let dirs = dirs
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Coord2::UP),
            'v' => Some(Coord2::DOWN),
            '<' => Some(Coord2::LEFT),
            '>' => Some(Coord2::RIGHT),
            _ => None,
        })
        .collect_vec();

    (map, dirs)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = std::fs::read_to_string(aoclib::get_test_input_file!(15)).unwrap();
        let (map, dirs) = parse_input(input);

        let p1 = part1(&map, &dirs);
        assert_eq!(p1, 1526018);

        let p2 = part2(&map, &dirs);
        assert_eq!(p2, 1550677);
    }
}
