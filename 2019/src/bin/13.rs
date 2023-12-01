use std::io;
use itertools::Itertools;
use std::collections::HashMap;
use aoc2019::OrderingExt;

fn part1(mem: &Vec<i64>) -> Option<usize> {
    let mut comp = aoc2019::IntCodeComputer::new(mem);

    comp.run();

    let mut framebuf: HashMap<(i64, i64), i64> = HashMap::new();

    for (x, y, id) in comp.outputs.iter().tuples() {
        framebuf.insert((*x, *y), *id);
    }

    Some(framebuf.values()
        .filter(|v| **v == 2)
        .count())
}


fn part2(mem: &Vec<i64>) -> Option<i64> {
    let mut mem = mem.to_vec();
    mem[0] = 2;

    let mut comp = aoc2019::IntCodeComputer::new(&mem);

    let mut score = 0;
    let mut ball_pos = 0;
    let mut paddle_pos = 0;
    while comp.run() {
        for (x, y, id) in comp.outputs.iter().tuples() {
            if *x == -1 && *y == 0 {
                score = *id;
            } else if *id == 4 {
                ball_pos = *x;
            } else if *id == 3 {
                paddle_pos = *x;
            }        
        }
        comp.outputs.clear();

        comp.inputs.push_back(-paddle_pos.cmp(&ball_pos).as_number() as i64);
    }

    for (x, y, id) in comp.outputs.iter().tuples() {
        if *x == -1 && *y == 0 {
            score = *id;
        }
    }

    Some(score)
}

fn main() -> io::Result<()> {
    let input = aoc2019::read_stdin_to_string()?;

    let mem = input.trim().split(",").map(|i| i.parse().unwrap()).collect();

    println!("Part 1: {:?}", part1(&mem));
    println!("Part 2: {:?}", part2(&mem));

    Ok(())
}