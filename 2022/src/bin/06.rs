use itertools::Itertools;
use std::io;

fn part1(input: &str) -> usize {
    let chars: Vec<_> = input.chars().collect();
    chars.windows(4)
        .enumerate()
        .find(|(_, w)| w.iter().all_unique())
        .unwrap()
        .0 + 4
}

fn part2(input: &str) -> usize {
    let chars: Vec<_> = input.chars().collect();
    chars
        .windows(14)
        .enumerate()
        .find(|(_, w)| w.iter().all_unique())
        .unwrap()
        .0 + 14
}

fn main() -> io::Result<()> {
    let input = aoc2022::read_input_string()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
