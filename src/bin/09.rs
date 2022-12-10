use std::{collections::HashSet, io};

fn tail_moves(hx: i32, hy: i32, tx: i32, ty: i32) -> bool {
    let dx = (hx - tx).abs();
    let dy = (hy - ty).abs();

    dx >= 2 || dy >= 2
}

fn part1(input: &Vec<(char, u8)>) -> usize {
    let mut hx = 0;
    let mut hy = 0;
    let mut tx = 0;
    let mut ty = 0;

    let mut tail_visited = HashSet::new();
    tail_visited.insert((tx, ty));

    for (dir, d) in input {
        for _ in 0..*d {
            match *dir {
                'U' => hy -= 1,
                'D' => hy += 1,
                'L' => hx -= 1,
                'R' => hx += 1,
                _ => panic!("Unknown dir {dir}"),
            };

            if tail_moves(hx, hy, tx, ty) {
                tx += (hx - tx).signum();
                ty += (hy - ty).signum();

                tail_visited.insert((tx, ty));
            }
        }
    }

    tail_visited.len()
}

fn part2(input: &Vec<(char, u8)>) -> usize {
    let mut xs = vec![0; 10];
    let mut ys = vec![0; 10];

    let mut tail_visited = HashSet::new();
    tail_visited.insert((0, 0));

    for (dir, d) in input {
        for _ in 0..*d {
            match *dir {
                'U' => xs[0] -= 1,
                'D' => xs[0] += 1,
                'L' => ys[0] -= 1,
                'R' => ys[0] += 1,
                _ => panic!("Unknown dir {dir}"),
            };

            for ti in 1..xs.len() {
                if tail_moves(xs[ti-1], ys[ti-1], xs[ti], ys[ti]) {
                    xs[ti] += (xs[ti-1] - xs[ti]).signum();
                    ys[ti] += (ys[ti-1] - ys[ti]).signum();
                }
            }
            tail_visited.insert((*xs.last().unwrap(), *ys.last().unwrap()));
        }
    }

    tail_visited.len()
}

fn main() -> io::Result<()> {
    let input = aoc2022::read_input_lines()?;
    let input = parse_input(input);

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}

fn parse_input(input: Vec<String>) -> Vec<(char, u8)> {
    input
        .into_iter()
        .map(|l| {
            let (l, r) = l.split_once(' ').expect("Line not in expected format");
            (l.chars().next().unwrap(), r.parse().unwrap())
        })
        .collect()
}
