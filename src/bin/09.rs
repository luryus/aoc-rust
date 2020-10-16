use std::io;

fn main() -> io::Result<()> {
    let input = aoc2019::read_stdin_to_string()?;

    let mem = input.trim().split(",").map(|i| i.parse().unwrap()).collect();

    let mut c = aoc2019::IntCodeComputer::new_with_input(&mem, vec![1]);
    c.run();
    println!("{:?}", c.outputs);

    let mut c = aoc2019::IntCodeComputer::new_with_input(&mem, vec![2]);
    c.run();
    println!("{:?}", c.outputs);

    Ok(())
}