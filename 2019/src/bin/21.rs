use std::io;
use aoc2019::{read_stdin_to_string, IntCodeComputer};


fn run1(mem: &Vec<i64>) {

    let mut comp = IntCodeComputer::new(mem);
    comp.run();

    while let Some(c) = comp.outputs.pop_front() {
        print!("{}", (c as u8) as char);
    }

    let input = "NOT A J\nNOT B T\nOR T J\nNOT C T\nOR T J\nAND D J\nWALK\n";

    input.bytes().map(|b| b as i64).for_each(|b| comp.inputs.push_back(b));

    comp.run();

    while let Some(c) = comp.outputs.pop_front() {
        if c > 127 || c < 0 {
            println!("Part 1: {}", c);
            return;
        }
        print!("{}", (c as u8) as char);
    }
}

fn run2(mem: &Vec<i64>) {

    let mut comp = IntCodeComputer::new(mem);
    comp.run();

    while let Some(c) = comp.outputs.pop_front() {
        print!("{}", (c as u8) as char);
    }

    let input = "NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
OR A T
OR B T
OR C T
OR D T
AND E T
OR H T
AND T J
RUN\n";

    input.bytes().map(|b| b as i64).for_each(|b| comp.inputs.push_back(b));

    comp.run();

    while let Some(c) = comp.outputs.pop_front() {
        if c > 127 || c < 0 {
            println!("Part 2: {}", c);
            return;
        }
        print!("{}", (c as u8) as char);
    }
}



fn main() -> io::Result<()> {
    let input = read_stdin_to_string()?;
    let mem: Vec<i64> = input.trim().split(",").map(|i| i.parse().unwrap()).collect();

    run1(&mem);
    run2(&mem);

    Ok(())
}