use itertools::Itertools;
use std::{collections::HashMap, io};

fn part1(input: &Vec<Instr>) -> usize {
    let mut mask_bits = 0usize;
    let mut mask_values = 0usize;
    let mut mem: HashMap<usize, usize> = HashMap::new();
    for i in input {
        match i {
            Instr::Mask(mask) => {
                mask_values = usize::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
                mask_bits =
                    usize::from_str_radix(&mask.replace("0", "1").replace("X", "0"), 2).unwrap();
            }
            Instr::Store(addr, val) => {
                // Add the 1 bits in the mask
                let masked_val = val | mask_bits;
                // Turn all zero bits in the mask low in the val
                let masked_val = (!mask_bits | mask_values) & masked_val;

                mem.insert(*addr, masked_val);
            }
        };
    }

    mem.values().sum()
}

fn part2(input: &Vec<Instr>) -> usize {
    let mut mask_bits = 0usize;
    let mut mask_or_value = 0usize;
    let mut mem: HashMap<usize, usize> = HashMap::new();
    for i in input {
        match i {
            Instr::Mask(mask) => {
                mask_or_value = usize::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
                mask_bits =
                    usize::from_str_radix(&mask.replace("0", "1").replace("X", "0"), 2).unwrap();
            }
            Instr::Store(addr, val) => {
                // Add the 1 bits in the mask
                let masked_addr = addr | mask_or_value;
                // Zero all X bits
                let masked_addr = masked_addr & mask_bits;

                // Handle the floating bits
                // find all bit indices where there was an X
                let idxs = (0..36)
                    .filter(|i| (mask_bits >> i) & 0b1 == 0)
                    .collect_vec();
                // Generate all combinations
                (0..=idxs.len())
                    .flat_map(|i| idxs.iter().combinations(i))
                    .map(|ii| ii.into_iter().fold(0, |acc, i| acc | (1 << i)))
                    .for_each(|m| {
                        mem.insert(m | masked_addr, *val);
                    });
            }
        };
    }

    mem.values().sum()
}

enum Instr {
    Mask(String),
    Store(usize, usize),
}

fn parse_input(inp: &Vec<String>) -> Vec<Instr> {
    inp.into_iter()
        .map(|l| {
            if l.starts_with("mask") {
                Instr::Mask(l.split_at(7).1.to_string())
            } else {
                let (addr, val) = aoc2020::read_ints_from_string(l)
                    .into_iter()
                    .collect_tuple()
                    .unwrap();
                Instr::Store(addr, val)
            }
        })
        .collect_vec()
}

fn main() -> io::Result<()> {
    let input = aoc2020::read_stdin_lines()?;
    let input = parse_input(&input);

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
