use std::io;

fn is_increasing(s: &str) -> bool {
    s.chars().zip(s.chars().skip(1))
        .all(|(c1, c2)| c1 <= c2)
}

fn has_pair(s: &str) -> bool {
    s.chars().zip(s.chars().skip(1))
        .any(|(c1, c2)| c1 == c2)
}

fn has_pair_strict(s: &str) -> bool {
    let (_, _, found) = s.chars()
        .chain(std::iter::once(' '))
        .fold((' ', 0, false), |(prev, count, found), c| {
            if prev == c {
                (c, count + 1, found)
            } else {
                (c, 1, found || count == 2)
            }
        });
    found
}

fn is_valid(n: u32) -> (bool, bool) {
    let ns = n.to_string();
    (is_increasing(&ns) && has_pair(&ns), is_increasing(&ns) && has_pair_strict(&ns))
}

fn run(start: u32, end: u32) -> (u32, u32) {
    (start..=end)
        .map(is_valid)
        .map(|(a, b)| (a as u32, b as u32))
        .fold((0, 0), |(acc_a, acc_b), (a, b)| (acc_a + a, acc_b + b))
}

fn main() -> io::Result<()> {
    let input = aoc2019::read_stdin_to_string()?;

    let mut parsed = input.split("-")
        .filter_map(|s| s.trim().parse::<u32>().ok());
    let start = parsed.next().unwrap();
    let end = parsed.next().unwrap();

    let (part1, part2) = run(start, end);

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
    
    Ok(())
}