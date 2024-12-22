use aoclib::coord2::Coord2;
use std::{collections::HashMap, io};

const NUMPAD_FORBIDDEN: Coord2 = Coord2 { y: 3, x: 0 };
const DPAD_FORBIDDEN: Coord2 = Coord2 { y: 0, x: 0 };

fn numpad_pos(c: char) -> Coord2 {
    match c {
        '0' => (3, 1),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        'A' => (3, 2),
        _ => unreachable!(),
    }
    .into()
}

fn dpad_pos(c: char) -> Coord2 {
    match c {
        '<' => (1, 0),
        '>' => (1, 2),
        'v' => (1, 1),
        '^' => (0, 1),
        'A' => (0, 2),
        _ => unreachable!(),
    }
    .into()
}

fn keypad_move<const IS_NUMPAD: bool>(from: char, to: char) -> Vec<Vec<char>> {
    let (from, to) = if IS_NUMPAD {
        (numpad_pos(from), numpad_pos(to))
    } else {
        (dpad_pos(from), dpad_pos(to))
    };
    let forbidden = const {
        if IS_NUMPAD {
            NUMPAD_FORBIDDEN
        } else {
            DPAD_FORBIDDEN
        }
    };

    let dy = (to.y as isize) - (from.y as isize);
    let dx = (to.x as isize) - (from.x as isize);

    if dy == 0 && dx == 0 {
        return vec![vec![]];
    }

    let mut routes = vec![];
    if dy != 0
        && from
            .y
            .checked_add_signed(dy)
            .is_some_and(|py| forbidden != (py, from.x).into())
    {
        let mut route = vec![];
        route.extend(steps_y(dy));
        route.extend(steps_x(dx));
        routes.push(route);
    }
    if dx != 0
        && from
            .x
            .checked_add_signed(dx)
            .is_some_and(|px| forbidden != (from.y, px).into())
    {
        let mut route = vec![];
        route.extend(steps_x(dx));
        route.extend(steps_y(dy));
        routes.push(route);
    }

    return routes;

    fn steps_y(dy: isize) -> impl Iterator<Item = char> {
        if dy > 0 {
            std::iter::repeat('v').take(dy as usize)
        } else {
            std::iter::repeat('^').take(-dy as usize)
        }
    }
    fn steps_x(dx: isize) -> impl Iterator<Item = char> {
        if dx > 0 {
            std::iter::repeat('>').take(dx as usize)
        } else {
            std::iter::repeat('<').take(-dx as usize)
        }
    }
}

fn keypad_seq_len<const IS_NUMPAD: bool>(
    l: &Vec<char>,
    level: usize,
    cache: &mut HashMap<Vec<char>, HashMap<usize, usize>>,
) -> usize {
    if let Some(Some(c)) = (!IS_NUMPAD).then(|| cache.get(l).and_then(|cd| cd.get(&level))) {
        return *c;
    }
    let mut pos = 'A';
    let mut sum = 0;
    for &c in l {
        let mut min_cost = usize::MAX;
        for mut m in keypad_move::<IS_NUMPAD>(pos, c) {
            m.push('A');
            let cost = if level > 0 {
                keypad_seq_len::<false>(&m, level - 1, cache)
            } else {
                m.len()
            };
            min_cost = min_cost.min(cost);
        }
        sum += min_cost;
        pos = c;
    }
    if !IS_NUMPAD {
        let inner_cache = cache.entry(l.clone()).or_default();
        inner_cache.insert(level, sum);
    }

    sum
}


fn part1(input: &[String]) -> usize {
    let mut cache = Default::default();
    input
        .iter()
        .map(|l| {
            let cs = l.chars().collect();
            let len = keypad_seq_len::<true>(&cs, 2, &mut cache);
            let num: usize = aoclib::read_ints_from_string(l, false)[0];
            len * num
        })
        .sum()
}

fn part2(input: &[String]) -> usize {
    let mut cache = Default::default();
    input
        .iter()
        .map(|l| {
            let cs = l.chars().collect();
            let len = keypad_seq_len::<true>(&cs, 25, &mut cache);
            let num: usize = aoclib::read_ints_from_string(l, false)[0];
            len * num
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;

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
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(21)).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, 169390);

        let p2 = part2(&input);
        assert_eq!(p2, 210686850124870);
    }
}
