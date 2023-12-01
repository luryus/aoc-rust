use std::io;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref NUMBER_REGEX: Regex = Regex::new(r"\d+").unwrap();
}


fn run1(input: &Vec<Operation>) {
    let mut grid: [[bool; 1000]; 1000] = [[false; 1000]; 1000];

    for op in input {
        for i in op.start.0..=op.end.0 {
            for j in op.start.1..=op.end.1 {
                grid[j][i] = match op.op_type {
                    OperationType::Off => false,
                    OperationType::On => true,
                    OperationType::Toggle => !grid[j][i],
                };
            }
        }
    }

    let c = grid.iter().flat_map(|x| x.iter()).filter(|x| **x).count();
    println!("Part 1: {}", c);
}

fn run2(input: &Vec<Operation>) {
    let mut grid: [[u64; 1000]; 1000] = [[0; 1000]; 1000];

    for op in input {
        for i in op.start.0..=op.end.0 {
            for j in op.start.1..=op.end.1 {
                grid[j][i] = match op.op_type {
                    OperationType::Off => grid[j][i].saturating_sub(1),
                    OperationType::On => grid[j][i]+1,
                    OperationType::Toggle => grid[j][i]+2,
                };
            }
        }
    }

    let c: u64 = grid.iter().flat_map(|x| x.iter()).sum();
    println!("Part 2: {}", c);
}


enum OperationType {
    On,
    Off,
    Toggle,
}

struct Operation {
    op_type: OperationType,
    start: (usize, usize),
    end: (usize, usize),
}

fn parse_input_line(line: &str) -> Operation {
    let op_type = if line.starts_with("toggle") {
        OperationType::Toggle
    } else if line.starts_with("turn on") {
        OperationType::On
    } else {
        OperationType::Off
    };

    let r: &Regex = &NUMBER_REGEX;
    let mut cs = r.captures_iter(line)
        .map(|c| c[0].parse().unwrap());

    let sx = cs.next().unwrap();
    let sy = cs.next().unwrap();
    let ex = cs.next().unwrap();
    let ey = cs.next().unwrap();

    Operation {
        op_type: op_type,
        start: (sx, sy),
        end: (ex, ey),
    }
}

fn main() -> io::Result<()> {
    let input = aoc2015::read_stdin_lines()
        .map(|x| x.iter().map(|l| parse_input_line(l)).collect::<Vec<Operation>>())?;
    run1(&input);
    run2(&input);

    Ok(())
}