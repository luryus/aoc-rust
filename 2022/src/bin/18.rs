use itertools::Itertools;
use ndarray::Array3;
use std::{
    collections::{HashSet, VecDeque},
    io,
};

type Coord = (usize, usize, usize);

fn neighbours_3d(c: Coord, dims: Coord) -> Vec<Coord> {
    let (z, y, x) = c;
    let mut res = vec![];
    if z > 0 {
        res.push((z - 1, y, x));
    }
    if z < dims.0 - 1 {
        res.push((z + 1, y, x));
    }

    if y > 0 {
        res.push((z, y - 1, x));
    }
    if y < dims.1 - 1 {
        res.push((z, y + 1, x));
    }

    if x > 0 {
        res.push((z, y, x - 1));
    }
    if x < dims.2 - 1 {
        res.push((z, y, x + 1));
    }

    res
}

fn part1(input: &Vec<Coord>) -> usize {
    let max_x = input.iter().map(|(x, _, _)| x).max().unwrap();
    let max_y = input.iter().map(|(_, y, _)| y).max().unwrap();
    let max_z = input.iter().map(|(_, _, z)| z).max().unwrap();

    let mut grid: Array3<u8> = Array3::zeros((max_z + 1, max_y + 1, max_x + 1));

    for (x, y, z) in input {
        grid[(*z, *y, *x)] = 1;
    }

    grid.indexed_iter()
        .filter(|(_, v)| **v > 0)
        .map(|(c, _)| {
            let pop_neighs = neighbours_3d(c, grid.dim())
                .into_iter()
                .filter(|n| grid[*n] > 0)
                .count();
            6 - pop_neighs
        })
        .sum()
}

fn part2(input: &Vec<Coord>) -> usize {
    let max_x = input.iter().map(|(x, _, _)| x).max().unwrap();
    let max_y = input.iter().map(|(_, y, _)| y).max().unwrap();
    let max_z = input.iter().map(|(_, _, z)| z).max().unwrap();

    let mut grid: Array3<u8> = Array3::zeros((max_z + 3, max_y + 3, max_x + 3));

    for (x, y, z) in input {
        grid[(*z + 1, *y + 1, *x + 1)] = 1;
    }

    let dims = grid.dim();
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    q.push_back((0, 0, 0));

    while let Some(c) = q.pop_front() {
        visited.insert(c);
        if grid[c] == 0 {
            let neihgs = neighbours_3d(c, dims);
            for n in neihgs {
                if grid[n] == 1 {
                    grid[c] = 2;
                } else  if !visited.contains(&n) && !q.contains(&n) {
                    q.push_back(n);
                }
            }
        }
    }

    grid.indexed_iter()
        .filter(|(_, v)| **v == 2)
        .map(|(c, _)| {
            neighbours_3d(c, grid.dim())
                .into_iter()
                .filter(|n| grid[*n] == 1)
                .count()
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;
    let input: Vec<Coord> = input
        .into_iter()
        .map(|l| {
            l.split(',')
                .filter_map(|n| n.parse::<usize>().ok())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
