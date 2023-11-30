use std::{io, str::FromStr};

struct Range(usize, usize);

impl Range {
    fn contains(&self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }
    fn overlap(&self, other: &Self) -> bool {
        self.0 <= other.1 && other.0 <= self.1
    }
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((l, r)) = s.split_once('-') else {
            return Err(());
        };

        let l = l.parse().map_err(|_| ())?;
        let r = r.parse().map_err(|_| ())?;

        Ok(Range(l, r))
    }
}

fn part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|l| l.split_once(',').unwrap())
        .map(|(l, r)| (l.parse::<Range>().unwrap(), r.parse::<Range>().unwrap()))
        .map(|(l, r)| (l.contains(&r) || r.contains(&l)) as usize)
        .sum()
}

fn part2(input: &[String]) -> usize {
    input
        .iter()
        .map(|l| l.split_once(',').unwrap())
        .map(|(l, r)| (l.parse::<Range>().unwrap(), r.parse::<Range>().unwrap()))
        .map(|(l, r)| (l.overlap(&r)) as usize)
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
