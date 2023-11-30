use itertools::Itertools;
use std::io;

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|g| g.chars().filter(|c| c.is_alphabetic()).unique().count())
        .sum()
}

fn part2(input: &str) -> usize {
    let groups = input.split("\n\n");

    groups
        .map(|g| {
            g.lines()
                .map(|l| l.chars().collect_vec())
                .fold1(|acc, x| {
                    acc.into_iter()
                        .filter(|c| x.contains(c))
                        .collect_vec()
                })
                .map_or(0, |s| s.len())
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input = aoc2020::read_stdin_to_string()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
