use std::io;
use aoc2019::{IntCodeComputer, read_stdin_to_string};
use itertools::Itertools;

fn check_coord(x: usize, y: usize, mem: &Vec<i64>) -> bool {
    let mut cmp = IntCodeComputer::new_with_input(mem, vec![x as i64, y as i64]);
    cmp.run();
    let r = *cmp.outputs.front().expect("No output!");

    r == 1
}

fn run1(mem: &Vec<i64>) {
    let mut s = 0;

    let mut pic: [[char; 50]; 50] = [[' '; 50]; 50];

    for (x, y) in (0usize..50).cartesian_product(0usize..50) {
        let r = check_coord(x, y, mem);
        pic[y][x] = if r { '#' } else { '.' };
        if r {
            s += 1;
        }
    }

    for l in 0..50 {
        println!("{}", &pic[l].iter().collect::<String>());
    }

    println!("Part 1: {}", s);
}

fn row_last_x(row: usize, prev_row_last: usize, mem: &Vec<i64>) -> usize {
    let mut last = prev_row_last;
    while !check_coord(last, row, mem) {
        last += 1;
    }
    while check_coord(last, row, mem) {
        last += 1
    }

    last - 1
}

fn run2(mem: &Vec<i64>) {
    let mut row = 3;
    let mut row_last = 0;
    while row_last < 100 {
        row += 1;
        row_last = row_last_x(row, row_last, mem);
    }

    loop {
        if check_coord(row_last - 99, row, mem) && check_coord(row_last - 99, row + 99, mem) && check_coord(row_last, row+99, mem) {
            println!("{}", (row_last - 99) * 10000  + row);
            break;
        }

        row += 1;
        row_last = row_last_x(row, row_last, mem);
    }
}

fn main() -> io::Result<()>  {
    let input = read_stdin_to_string()?;

    let mem: Vec<i64> = input.trim().split(",").map(|i| i.parse().unwrap()).collect();

    run1(&mem);
    run2(&mem);

    Ok(())
}