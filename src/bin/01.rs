use std::io;

fn part1(inp: &Vec<String>) {
    let s: u32 = inp.iter()
        .filter_map(|l| l.parse::<u32>().ok())
        .map(|w| w / 3 - 2)
        .sum();
    println!("Part 1: {}", s);
}

fn full_fuel(module: u32) -> u32 {
    let mut prev = module;
    let mut sum = 0;

    while prev > 0 {
        prev = (prev / 3).saturating_sub(2);
        sum += prev;
    }

    sum
}

fn part2(inp: &Vec<String>) {
    let s: u32 = inp.iter()
        .filter_map(|l| l.parse::<u32>().ok())
        .map(|w| full_fuel(w))
        .sum();
    println!("Part 2: {}", s);
}

fn main() -> io::Result<()> {
    let input = aoc2019::read_stdin_lines()?;

    part1(&input);
    part2(&input);

    Ok(())
}