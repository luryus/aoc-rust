use itertools::Itertools;
use std::io;

fn part1(input: &[String]) -> usize {
    input
        .split(|l| l.trim().is_empty())
        .map(|g| g.iter().filter_map(|l| l.parse::<usize>().ok()).sum())
        .max()
        .unwrap()
}

fn part2(input: &[String]) -> usize {
    input
        .split(|l| l.trim().is_empty())
        .map(|g| g.iter().filter_map(|l| l.parse::<usize>().ok()).sum::<usize>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}

fn main() -> io::Result<()> {
    let input = aoc2022::read_input_lines()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
