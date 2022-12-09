use itertools::Itertools;
use ndarray::{array, Array1, Array2, ArrayView1};
use std::io;
use aoc2022::iter::TakeUntilInclusiveExt;


fn part1(input: &Array2<u32>) -> usize {
    let mut visible = Array2::zeros(input.raw_dim());

    let (rows, cols) = visible.dim();

    visible.row_mut(0).fill(1);
    visible.row_mut(rows - 1).fill(1);
    visible.column_mut(0).fill(1);
    visible.column_mut(cols - 1).fill(1);

    for (ri, r) in input.rows().into_iter().enumerate() {
        // l to r
        let mut max_h = r.first().unwrap_or(&0);
        for (ci, h) in r.iter().enumerate().skip(1) {
            if h > max_h {
                visible[[ri, ci]] = 1usize;
                max_h = h;
            }
        }

        // r to l
        let mut max_h = r.last().unwrap_or(&0);
        for (ci, h) in r.iter().enumerate().rev().skip(1) {
            if h > max_h {
                visible[[ri, ci]] = 1;
                max_h = h;
            }
        }
    }

    for (ci, c) in input.columns().into_iter().enumerate() {
        // top to bottom
        let mut max_h = c.first().unwrap_or(&0);
        for (ri, h) in c.iter().enumerate().skip(1) {
            if h > max_h {
                visible[[ri, ci]] = 1;
                max_h = h;
            }
        }

        // r to l
        let mut max_h = c.last().unwrap_or(&0);
        for (ri, h) in c.iter().enumerate().rev().skip(1) {
            if h > max_h {
                visible[[ri, ci]] = 1;
                max_h = h;
            }
        }
    }

    visible.sum()
}

fn part2(input: &Array2<u32>) -> usize {
    input
        .indexed_iter()
        .map(|((row, col), &h)| {
            let left = get_score(Dir::Down, input.row(row), col, h);
            let right = get_score(Dir::Up, input.row(row), col, h);
            let up = get_score(Dir::Down, input.column(col), row, h);
            let down = get_score(Dir::Up, input.column(col), row, h);
            left * right * up * down
        })
        .max()
        .unwrap()
}

#[derive(PartialEq)]
enum Dir { Up, Down }

fn get_score(dir: Dir, arr: ArrayView1<u32>, start: usize, h: u32) -> usize {
    if dir == Dir::Up {
        arr.iter().skip(start+1).take_until_inclusive(|&&x| x >= h).count()
    } else {
        arr.iter().take(start).rev().take_until_inclusive(|&&x| x >= h).count()
    }
}

fn main() -> io::Result<()> {
    let input = aoc2022::read_input_int_matrix()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
