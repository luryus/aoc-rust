use std::io;

const ADD: usize = 1usize;
const MUL: usize = 2usize;
const HALT: usize = 99usize;

fn run(inp: &Vec<usize>, a: usize, b: usize) -> Option<usize> {
    let mut ins = inp.clone();

    // Initialize
    ins[1] = a;
    ins[2] = b;

    let mut pos = 0;
    loop {
        match ins[pos] {
            ADD => {
                let op1 = ins[pos + 1];
                let op2 = ins[pos + 2];
                let target = ins[pos + 3];

                ins[target] = ins[op1] + ins[op2];
                pos += 4;
            }
            MUL => {
                let op1 = ins[pos + 1];
                let op2 = ins[pos + 2];
                let target = ins[pos + 3];

                ins[target] = ins[op1] * ins[op2];
                pos += 4;
            }
            HALT => return Some(ins[0]),
            _ => return None,
        }
    }
}

fn part1(inp: &Vec<usize>) -> Option<usize> {
    run(inp, 12, 2)
}

fn part2(inp: &Vec<usize>) -> Option<usize> {
    for i in 0..=99 {
        for j in 0..=99 {
            let r = run(inp, i, j);
            if Some(19690720) == r {
                return Some(100 * i + j);
            }
        }
    }

    None
}

fn main() -> io::Result<()> {
    let input = aoc2019::read_stdin_to_string().map(|l| {
        l.split(",")
            .map(|i| i.parse::<usize>().expect("Non-numeric input"))
            .collect::<Vec<usize>>()
    })?;

    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));

    Ok(())
}
