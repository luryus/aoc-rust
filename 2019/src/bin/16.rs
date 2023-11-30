use std::io;
use itertools::Itertools;
use std::iter::repeat;
use num_integer::lcm;
use std::num::Wrapping;

fn pattern<'a>(i: &'a usize) -> impl Iterator<Item=i8> + 'a {
    let base = vec![0i8, 1, 0, -1];
    let p = base.into_iter()
        .flat_map(move |x| repeat(x).take(*i))
        .cycle()
        .skip(1);

    p
}

fn run1(input: &Vec<i8>) -> Vec<i8> {

    let mut last_round = input.to_vec();
    for _ in 0..100 {
        let mut results = Vec::with_capacity(100);
        for j in 0..input.len() {
            let res = (last_round.iter().zip(pattern(&(j+1)))
                .filter_map(|(a, b)| match b {
                    1 => Some(*a), -1 => Some(-(*a)), _ => None
                })
                .fold(Wrapping(0i32), |x, y| x + Wrapping(y.into()))
                .0.abs() % 10) as i8;
            results.push(res);
        }
        last_round = results;
    }

    last_round
}


fn run2(input: &Vec<i8>, offset: usize) -> Vec<i32> {
    let mut last_round = input.iter().cycle().take(10000*input.len()).skip(offset).copied().map(|x| x as i32).collect::<Vec<i32>>();

    // when the offset is greater than (input.len() / 2), the following is true:
    // When calculating the result for any index (i) greater than or equal to the offset
    // The "multiplier sequence" starts with all zeros, and has i zeros at the beginning
    // Then the multiplier sequence is all ones until the end of the sequence. The multiplier at position i is the first
    // one. 

    // To calculate the result for i, we can just sum all the values from index i.
    
    for _ in 0..100 {
        let res = last_round.iter().rev().scan(0, |state, x| {
            *state = (*state + x).abs() % 10;
            Some(*state)
        }).collect::<Vec<i32>>();

        last_round = res;
        last_round.reverse();
    }

    last_round
}

fn main() -> io::Result<()> {

    let input = aoc2019::read_stdin_to_string()?;
    let input = input.trim()
        .bytes()
        .map(|c| (c - b'0') as i8)
        .collect::<Vec<_>>();

    let p1 = run1(&input);
    println!("Part 1: {}", p1.iter().map(|n| n.to_string()).take(8).join(""));


    //let p2 = run2(&input);
    let offset = input.iter().take(7).map(|n| n.to_string()).join("").parse::<usize>().unwrap();
    println!("Part 2 offset: {}", offset);

    let p2_total_size = 10000 * input.len();
    println!("Part 2 total size: {}", p2_total_size);

    if offset < p2_total_size / 2 {
        panic!();
    }

    let p2 = run2(&input, offset);

    println!("Part 2 result: {}", p2.iter().take(8).map(|n| n.to_string()).join(""));

    Ok(())
}