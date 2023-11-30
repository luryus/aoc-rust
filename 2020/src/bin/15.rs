use std::{num::NonZeroUsize, io};

fn run(input: &Vec<usize>, iters: usize) -> usize {
    let mut mem: Vec<Option<NonZeroUsize>> = vec![None; iters]; 

    for (i, &n) in input.iter().enumerate() {
        mem[n] = NonZeroUsize::new(i+1);
    }

    let mut next = 0;
    for i in input.len()+1..iters {
        let n = next;
        next = match mem[n] {
            Some(li) => i - li.get(),
            None => 0
        };
        mem[n] = NonZeroUsize::new(i);
    }

    next
}

fn main() -> io::Result<()> {
    let input = aoc2020::read_ints_from_stdin()?;

    let p1 = run(&input, 2020);
    println!("Part 1: {}", p1);

    let p2 = run(&input, 30_000_000);
    println!("Part 2: {}", p2);

    Ok(())
}