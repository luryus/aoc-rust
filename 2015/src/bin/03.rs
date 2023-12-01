use std::io;
use std::collections::BTreeMap;

fn part1(input: &String) {
    let mut pos = (0, 0);
    let mut houses: BTreeMap<(i32, i32), u32> = BTreeMap::new();

    houses.insert(pos, 1);

    for c in input.chars() {
        pos = match c {
            '>' => (pos.0 + 1, pos.1),
            '<' => (pos.0 - 1, pos.1),
            '^' => (pos.0, pos.1 - 1),
            'v' => (pos.0, pos.1 + 1),
            _ => pos
        };
        let h_entry = houses.entry(pos).or_insert(0);
        *h_entry += 1;
    }

    println!("{}", houses.len());
}

fn part2(input: String) {
    let mut pos = (0, 0);
    let mut houses: BTreeMap<(i32, i32), u32> = BTreeMap::new();

    houses.insert(pos, 1);

    for c in input.chars().step_by(2) {
        pos = match c {
            '>' => (pos.0 + 1, pos.1),
            '<' => (pos.0 - 1, pos.1),
            '^' => (pos.0, pos.1 - 1),
            'v' => (pos.0, pos.1 + 1),
            _ => pos
        };
        let h_entry = houses.entry(pos).or_insert(0);
        *h_entry += 1;
    }

    pos = (0, 0);

    for c in input.chars().skip(1).step_by(2) {
        pos = match c {
            '>' => (pos.0 + 1, pos.1),
            '<' => (pos.0 - 1, pos.1),
            '^' => (pos.0, pos.1 - 1),
            'v' => (pos.0, pos.1 + 1),
            _ => pos
        };
        let h_entry = houses.entry(pos).or_insert(0);
        *h_entry += 1;
    }

    println!("{}", houses.len());
}

fn main() -> io::Result<()> {
    let input = aoc2015::read_stdin_to_string()?;


    part1(&input);
    part2(input);

    Ok(())
}