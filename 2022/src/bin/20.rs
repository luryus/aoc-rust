use std::{collections::VecDeque, io};

fn run(input: &Vec<i64>, rounds: usize, key: i64) -> i64 {
    let len = input.len() as i64;
    let mut ll: VecDeque<_> = input.iter().copied().map(|n| n * key).enumerate().collect();

    for _ in 0..rounds {
        for i in 0..input.len() {
            let c = ll.iter().position(|(ii, _)| *ii == i).unwrap();
            let n = ll[c].1;

            let new_pos = (c as i64 + n).rem_euclid(len - 1) as usize;

            let node = ll.remove(c).unwrap();
            ll.insert(new_pos, node);
        }
    }

    let zero_idx = ll.iter().position(|(_ii, nn)| *nn == 0).unwrap();
    ll[(zero_idx + 1000) % input.len()].1
        + ll[(zero_idx + 2000) % input.len()].1
        + ll[(zero_idx + 3000) % input.len()].1
}

fn main() -> io::Result<()> {
    let input = aoc2022::read_input_ints(true)?;

    let p1 = run(&input, 1, 1);
    println!("Part 1: {}", p1);

    let p2 = run(&input, 10, 811589153);
    println!("Part 2: {}", p2);

    Ok(())
}
