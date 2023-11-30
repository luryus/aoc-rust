use aoc2019::{IntCodeComputer};
use std::io::{Result, stdin, BufRead};
use std::fs;
use std::env::args;

fn part1(mem: &Vec<i64>) {
    let mut comp = IntCodeComputer::new(mem);

    while comp.run() {
        println!("{}", comp.outputs.drain(..).map(|x| x as u8 as char).collect::<String>());
        let line = stdin().lock().lines().next().unwrap().unwrap();
        for x in line.chars() {
            comp.inputs.push_back(x as i64);
        }
        comp.inputs.push_back('\n' as i64);
    }

    println!("{}", comp.outputs.drain(..).map(|x| x as u8 as char).collect::<String>());
}


fn main() -> Result<()> {
    let path = args().skip(1).next().unwrap();
    let input = fs::read_to_string(path)?;
    let mem: Vec<i64> = input.trim()
        .split(",")
        .map(|i| i.parse().unwrap())
        .collect();
    
    part1(&mem);

    Ok(())
}