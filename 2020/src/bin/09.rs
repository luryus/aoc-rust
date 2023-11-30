use std::io;
use itertools::{Itertools, MinMaxResult};

const PREAMBLE: usize = 25;

fn part1(input: &Vec<i64>) -> i64 {
    for i in PREAMBLE..input.len() {
        let valid = input.iter().copied()
            .skip(i - PREAMBLE).take(PREAMBLE)
            .tuple_combinations()
            .filter(|(l, r)| l + r == input[i])
            .next().is_some();
        if !valid {
            return input[i];
        }
    }

    unreachable!("No invalid row found")
}


fn part2(input: &Vec<i64>) -> i64 {
    let part1_res = part1(input);

    let minmaxres= (0..input.len())
        .filter_map(|lo| {
            (1..)
                .map(|len| (len, input.iter().skip(lo).take(len).sum::<i64>()))
                .take_while(|(_, s)| *s <= part1_res)
                .last()
                .filter(|(_, s)| *s == part1_res)
                .map(|(len, _)| (lo, len))
        })
        .next()
        .map(|(lo, len)| input.iter().skip(lo).take(len).minmax());

    match minmaxres {
        Some(MinMaxResult::MinMax(min, max)) => min + max,
        _ => unreachable!()
    }
}

fn main() -> io::Result<()> {
    let input = aoc2020::read_ints_from_stdin()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}