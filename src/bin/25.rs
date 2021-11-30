use std::io;

fn part1(input: &Vec<usize>) -> usize {
    let pub_a = input[0];
    let pub_b = input[1];
    let base = 7usize;
    let modulus = 20201227usize; 

    let mut exp = 1usize;
    let mut acc = base;

    while acc != pub_a {
        exp += 1;
        acc = (acc * base) % modulus; 
    }

    let mut key = 1;
    for _ in 0..exp {
        key = (key * pub_b) % modulus;
    }

    key
}

fn main() -> io::Result<()> {
    let input = aoc2020::read_ints_from_stdin()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    Ok(())
}