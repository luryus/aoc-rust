use std::io;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE: Regex =  Regex::new(r"(\d+)\-(\d+) (\w): (\w+)").unwrap();
}

fn parse(input: &str) -> Option<(usize, usize, char, &str)> {
    let caps = RE.captures(input)?;

    let lo = caps.get(1)?.as_str().parse().ok()?;
    let hi = caps.get(2)?.as_str().parse().ok()?;
    let c = caps.get(3)?.as_str().chars().next()?;
    let pass = caps.get(4)?.as_str();

    Some((lo, hi, c, pass))
}

fn valid(lo: usize, hi: usize, c: char, pass: &str) -> bool {
    let co = pass.chars().filter(|&sc| sc == c).count();
    lo <= co && hi >= co
}

fn part1(input: &Vec<(usize, usize, char, &str)>) -> usize {
    input.iter()
        .filter(|(lo, hi, c, pass)| valid(*lo, *hi, *c, *pass))
        .count()
}


fn valid2(lo: usize, hi: usize, c: char, pass: &str) -> bool {
   let loc = pass.chars().nth(lo - 1).unwrap();
   let hic = pass.chars().nth(hi - 1).unwrap();
   (loc == c) ^ (hic == c)
}


fn part2(input: &Vec<(usize, usize, char, &str)>) -> usize {
    input.iter()
        .filter(|(lo, hi, c, pass)| valid2(*lo, *hi, *c, *pass))
        .count()
}

fn main() -> io::Result<()> {
    let input = aoc2020::read_stdin_lines()?;
    let parsed = input.iter()
        .map(|l| parse(l).unwrap())
        .collect::<Vec<_>>();

    let p1 = part1(&parsed);
    println!("Part 1: {}", p1);

    let p2 = part2(&parsed);
    println!("Part 2: {}", p2);

    Ok(())
}