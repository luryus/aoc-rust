use ndarray::Array2;
use std::{
    collections::{HashSet, VecDeque},
    io,
};

type Coord = (usize, usize);

fn adjacents(x: usize, y: usize, w: usize, h: usize) -> impl Iterator<Item = Coord> {
    [
        if x > 0 { Some((x - 1, y)) } else { None },
        if x < w - 1 { Some((x + 1, y)) } else { None },
        if y > 0 { Some((x, y - 1)) } else { None },
        if y < h - 1 { Some((x, y + 1)) } else { None },
    ]
    .into_iter()
    .flatten()
}


fn run(input: &Array2<u8>, start: Coord, target: Coord) -> Option<usize> {
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    let (h, w) = input.dim();

    q.push_back((start, 0));

    while let Some(((y, x), d)) = q.pop_front() {
        if visited.insert((y, x)) {
            if target == (y, x) {
                return Some(d);
            }

            for (ax, ay) in adjacents(x, y, w, h) {
                if !visited.contains(&(ay, ax)) && input[(y, x)] + 1 >= input[(ay, ax)] {
                    q.push_back(((ay, ax), d + 1));
                }
            }
        }
    }

    None
}

fn part1(input: &Array2<u8>, start: Coord, target: Coord) -> usize {
    run(input, start, target).expect("No route found")
}

fn part2(input: &Array2<u8>, target: Coord) -> usize {
    input.indexed_iter().filter(|(_c, h)| h == &&b'a')
        .filter_map(|(start, _)| run(input, start, target))
        .min()
        .expect("No routes found?")
}

fn main() -> io::Result<()> {
    let mut input = aoc2022::read_input_byte_matrix()?;
    let (start, sh) = input.indexed_iter_mut().find(|(_c, el)| el == &&b'S').unwrap();
    *sh = b'a';
    let (target, th) = input.indexed_iter_mut().find(|(_c, el)| el == &&b'E').unwrap();
    *th = b'z';

    let p1 = part1(&input, start, target);
    println!("Part 1: {}", p1);

    let p2 = part2(&input, target);
    println!("Part 2: {}", p2);

    Ok(())
}
