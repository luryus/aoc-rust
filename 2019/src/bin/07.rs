use std::io;
use itertools::Itertools;

fn run(mem: &Vec<i64>, phase_offset: usize) -> i64 {
    const AMPS: usize = 5;
    
    let mut max_out = 0;

    for phase_settings in (phase_offset..(phase_offset+AMPS)).permutations(AMPS) {
        let mut next_input = 0;
        let mut amps = phase_settings
            .into_iter()
            .map(|p| aoc2019::IntCodeComputer::new_with_input(mem, vec![p as i64]))
            .collect::<Vec<_>>();
        let mut run = true;
        while run {
            for a in amps.iter_mut() {
                a.inputs.push_back(next_input);
                let waiting_for_input = a.run();
                if !waiting_for_input {
                    run = false;
                }
                next_input = a.outputs.pop_front().unwrap();
            }
        }
        
        if next_input > max_out {
            max_out = next_input;
        }
    }

    max_out
}

fn main() -> io::Result<()> {
    let input = aoc2019::read_stdin_to_string().map(|l| {
        l.split(",")
            .map(|i| i.trim().parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    })?;

    println!("Part 1: {}", run(&input, 0));
    println!("Part 2: {}", run(&input, 5));

    Ok(())
}