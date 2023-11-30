use std::io;
use itertools::Itertools;

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
enum Dir {
    Up, Down, Left, Right,
}

fn turn_left(dir: Dir) -> Dir {
    match dir {
        Dir::Up => Dir::Left,
        Dir::Left => Dir::Down,
        Dir::Down => Dir::Right,
        Dir::Right => Dir::Up,
    }
}

fn turn_right(dir: Dir) -> Dir {
    match dir {
        Dir::Up => Dir::Right,
        Dir::Left => Dir::Up,
        Dir::Down => Dir::Left,
        Dir::Right => Dir::Down
    }
}

fn next_coord(x: usize, y: usize, w: usize, h: usize, dir: Dir) -> Option<(usize, usize)> {
    match dir {
        Dir::Up => if y > 0 { Some((x, y-1)) } else { None },
        Dir::Down => if y < h-1 { Some((x, y+1)) } else { None },
        Dir::Left => if x > 0 { Some((x-1, y)) } else { None },
        Dir::Right => if x < w-1 { Some((x+1, y)) } else { None },
    }
}

fn run1(mem: &Vec<i64>) {
    let mut cmp = aoc2019::IntCodeComputer::new(mem);
    cmp.run();

    let output_str = cmp.outputs.iter().map(|b| *b as u8 as char).collect::<String>();
    let output = output_str.trim().split("\n").collect::<Vec<_>>();
    for l in &output {
        println!("{}", l);
    }

    let (w, h) = (output[0].len(), output.len());
    let mut checksum = 0;
    let mut start_point = (0, 0);
    for (x, y) in (1..w-1).cartesian_product(1..h-1) {
        if output[y].as_bytes()[x] == b'#' && output[y-1].as_bytes()[x] == b'#' && output[y+1].as_bytes()[x] == b'#' &&
            output[y].as_bytes()[x-1] == b'#' && output[y].as_bytes()[x+1] == b'#' {

            checksum += x*y;
        }
        if output[y].as_bytes()[x] == b'^' {
            start_point = (x, y);
        }
    }

    println!("{}", checksum);

    let mut dir = Dir::Up;
    let (mut x, mut y) = start_point;
    let mut d = 0;
    let mut instructions: Vec<String> = Vec::new();
    loop {
        if let Some((next_x, next_y)) = next_coord(x, y, w, h, dir) {
            if output[next_y].as_bytes()[next_x] == b'#' {
                x = next_x;
                y = next_y;
                d += 1;
                continue;
            }
        }
        if d == 0 && instructions.last() == Some(&"R".to_owned()) {
            instructions.pop();
            instructions.push("L".to_owned());
            dir = turn_right(turn_right(dir));
        }
        else if d == 0 && instructions.last() == Some(&"L".to_owned()) {
            break;
        }
        else {
            if d > 0 {
                instructions.push(d.to_string());
                d = 0;
            }
            instructions.push("R".to_owned());
            dir = turn_right(dir);
        }
    }
    println!("{:?}", instructions);
}

fn run2(mem: &Vec<i64>) {
    let mut mem = mem.to_vec();
    mem[0] = 2;
    
    // TODO analyze route and find groups automagically
    let move_routine = "A,B,A,C,A,B,C,A,B,C\n".as_bytes();
    let a = "R,12,R,4,R,10,R,12\n".as_bytes();
    let b = "R,6,L,8,R,10\n".as_bytes();
    let c = "L,8,R,4,R,4,R,6\n".as_bytes();
    let no_output = "n\n".as_bytes();

    let mut comp = aoc2019::IntCodeComputer::new(&mem);
    for b in [move_routine, a, b, c, no_output].concat() {
        comp.inputs.push_back(b as i64);
    }

    comp.run();
    println!("{:?}", comp.outputs.back());
}


fn main() -> io::Result<()> {
    let input = aoc2019::read_stdin_to_string()?;

    let mem: Vec<i64> = input.trim().split(",").map(|i| i.parse().unwrap()).collect();

    run1(&mem);
    run2(&mem);

    Ok(())
}