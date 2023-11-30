use std::io;

fn part1(inp: &Vec<String>) {
    let s: u32 = inp.iter()
        .filter_map(|l| l.parse::<u32>().ok())
        .map(|w| w / 3 - 2)
        .sum();
    println!("Part 1: {}", s);
}

fn full_fuel2(module: u32) -> u32 {
    let f = (module / 3).saturating_sub(2);
    if f > 0 { 
        f + full_fuel2(f) 
    } else { 
        f
    } 
}

fn part2(inp: &Vec<String>) {
    let s: u32 = inp.iter()
        .filter_map(|l| l.parse::<u32>().ok())
        .map(|w| full_fuel2(w))
        .sum();
    println!("Part 2: {}", s);
}

fn main() -> io::Result<()> {
    let input = aoc2019::read_stdin_lines()?;

    part1(&input);
    part2(&input);

    Ok(())
}