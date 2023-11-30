use itertools::Itertools;
use ndarray::{s, Array2, Array1};
use std::io;

type Coord = (usize, usize);

const POUR_POINT: Coord = (0, 500);

fn part2(mut input: Array2<u8>) -> usize {
    let (h, mut w) = input.dim();
    input.push_row(Array1::zeros(w).view()).unwrap();
    let h = h +1;

    for i in 0.. {
        let mut sand = POUR_POINT;

        loop {
            if sand.1 + 1 >= w {
                input.push_column(Array1::zeros(h).view()).unwrap();
                w += 1;
            }

            if sand.0 + 1 == h {
                input[sand] = 2;
                break;
            } else if input[(sand.0 + 1, sand.1)] == 0 {
                sand = (sand.0 + 1, sand.1);
            } else if input[(sand.0 + 1, sand.1 - 1)] == 0 {
                sand = (sand.0 + 1, sand.1 - 1);
            } else if input[(sand.0 + 1, sand.1 + 1)] == 0 {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                input[sand] = 2;
                if sand == POUR_POINT {
                    //print_map(&input);
                    return i + 1;
                }
                break;
            }
        }
    }

    unreachable!()
}

fn part1(mut input: Array2<u8>) -> usize {
    let (h, _) = input.dim();

    for i in 0.. {
        let mut sand = POUR_POINT;

        loop {
            if sand.0 + 1 >= h {
                return i;
            } else if input[(sand.0 + 1, sand.1)] == 0 {
                sand = (sand.0 + 1, sand.1);
            } else if input[(sand.0 + 1, sand.1 - 1)] == 0 {
                sand = (sand.0 + 1, sand.1 - 1);
            } else if input[(sand.0 + 1, sand.1 + 1)] == 0 {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                input[sand] = 2;
                break;
            }
        }
    }

    unreachable!()
}

fn parse_input(lines: Vec<String>) -> Array2<u8> {
    let coords: Vec<Vec<Coord>> = lines
        .into_iter()
        .map(|l| {
            l.split(" -> ")
                .map(|c| {
                    let (x, y) = c.split_once(',').unwrap();
                    (y.parse().unwrap(), x.parse().unwrap())
                })
                .collect()
        })
        .collect();

    let max_y = coords
        .iter()
        .flat_map(|l| l.iter())
        .map(|c| c.0)
        .max()
        .unwrap();
    let max_x = coords
        .iter()
        .flat_map(|l| l.iter())
        .map(|c| c.1)
        .max()
        .unwrap();

    let mut arr = Array2::zeros((max_y + 1, max_x + 1));

    for l in coords {
        for ((sy, sx), (ey, ex)) in l.into_iter().tuple_windows() {
            if sy == ey {
                arr.row_mut(sy)
                    .slice_mut(s![sx.min(ex)..=sx.max(ex)])
                    .fill(1);
            } else {
                arr.column_mut(sx)
                    .slice_mut(s![sy.min(ey)..=sy.max(ey)])
                    .fill(1);
            }
        }
    }

    arr
}

fn main() -> io::Result<()> {
    let input = aoc2022::read_input_lines()?;
    let input = parse_input(input);

    //print_map(&input);

    let p1 = part1(input.clone());
    println!("Part 1: {}", p1);

    let p2 = part2(input);
    println!("Part 2: {}", p2);

    Ok(())
}

pub fn print_map(mtx: &Array2<u8>) {
    let minx = mtx.columns().into_iter().enumerate()
        .find(|(_, c)| c.sum() > 0).unwrap().0;
    let view = mtx.slice(s![.., minx..]);

    for r in view.rows() {
        println!(
            "{}",
            r.iter()
                .map(|c| match c {
                    1 => '█',
                    2 => 'o',
                    _ => '.'
                })
                .collect::<String>()
        );
    }
}