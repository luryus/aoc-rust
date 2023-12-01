use std::io;

fn main() -> io::Result<()> {
    let input = aoc2019::read_stdin_to_string().map(|l| {
        l.trim().split(",")
            .map(|i| i.parse::<i64>().expect("Non-number input"))
            .collect::<Vec<_>>()
    })?;

    let mut comp1 = aoc2019::IntCodeComputer::new_with_input(&input, vec![1]);
    comp1.run();
    println!("Part 1: {:?}", comp1.outputs.back().expect("Empty output"));
    
    let mut comp2 = aoc2019::IntCodeComputer::new_with_input(&input, vec![5]);
    comp2.run();
    println!("Part 2: {:?}", comp2.outputs);

    Ok(())
}