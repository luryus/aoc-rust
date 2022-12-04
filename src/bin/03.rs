use std::{collections::HashSet, io};

fn part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(l, r)| {
            let ls: HashSet<u8> = l.as_bytes().iter().copied().collect();
            let rs: HashSet<u8> = r.as_bytes().iter().copied().collect();
            ls.intersection(&rs).copied().next().unwrap()
        })
        .map(|b| {
            (if b.is_ascii_uppercase() {
                b - b'A' + 27
            } else {
                b - b'a' + 1
            }) as usize
        })
        .sum()
}

fn part2(input: &[String]) -> usize {
    input
        .chunks(3)
        .map(|c| {
            c.iter()
                .map(|s| s.as_bytes().iter().copied().collect::<HashSet<_>>())
                .reduce(|mut a, b| {
                    a.retain(|e| b.contains(e));
                    a 
                })
                .unwrap().drain().next().unwrap()
        })
        .map(|b| {
            (if b.is_ascii_uppercase() {
                b - b'A' + 27
            } else {
                b - b'a' + 1
            }) as usize
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input = aoc2021::read_input_lines()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
