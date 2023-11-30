use std::collections::BTreeSet;
use std::io;

fn seat_id2(s: &str) -> usize {
    s.chars().fold(0, |acc, c| match c {
        'B' | 'R' => (acc << 1) | 1,
        _ => acc << 1,
    })
}

fn part1(input: &Vec<String>) -> usize {
    input.iter().map(|s| seat_id2(s)).max().unwrap()
}

fn part2(input: &Vec<String>) -> usize {
    let seats = input.iter().map(|s| seat_id2(s)).collect::<BTreeSet<_>>();

    seats
        .iter()
        .zip(seats.iter().skip(1))
        .filter(|(&l, &r)| r - l > 1)
        .map(|(&l, _)| l + 1)
        .next()
        .unwrap()
}

fn main() -> io::Result<()> {
    let input = aoc2020::read_stdin_lines()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
