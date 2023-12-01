use itertools::Itertools;
use std::collections::VecDeque;
use std::io;


fn part1(input: &VecDeque<u8>) -> usize {
    let mut input = input.clone();
    let (min, max) = (*input.iter().min().unwrap(), *input.iter().max().unwrap());
    for _ in 0..100 {
        let cur = *input.front().unwrap();
        input.rotate_left(1);

        let (a, b, c) = (
            input.pop_front().unwrap(),
            input.pop_front().unwrap(),
            input.pop_front().unwrap(),
        );

        let mut dest = if cur > min { cur - 1 } else { max };
        while dest == a || dest == b || dest == c {
            dest = if dest > min { dest - 1 } else { max };
        }

        let dest_idx = input.iter().position(|&x| x == dest).unwrap();
        input.insert(dest_idx + 1, a);
        input.insert(dest_idx + 2, b);
        input.insert(dest_idx + 3, c);
    }

    input.rotate_left(input.iter().position(|&x| x == 1).unwrap());
    input.into_iter().skip(1).join("").parse().unwrap()
}


fn part2(input: &VecDeque<u8>) -> usize {
    const SIZE: usize = 1_000_000;
    let input = input.into_iter().map(|x| *x as usize).collect_vec();
    let mut nexts = [0usize; SIZE+1];

    for (l, r) in input.iter().zip(input.iter().skip(1)) {
        nexts[*l] = *r;
    }
    let max = *input.iter().max().unwrap();
    nexts[input[input.len()-1]] = max + 1;
    for i in max+1..=SIZE {
        nexts[i] = i+1;
    }

    let mut cur = input[0];
    nexts[SIZE] = cur;
    
    for _ in 0..10_000_000 {
        let a = nexts[cur];
        let b = nexts[a];
        let c = nexts[b];

        let next_cur = nexts[c];

        let mut dest = if cur == 1 { SIZE } else { cur - 1 };
        while a == dest || b == dest || c == dest {
            dest = if dest == 1 { SIZE } else { dest - 1 };
        }

        nexts[c] = nexts[dest];
        nexts[dest] = a;

        nexts[cur] = next_cur;
        cur = next_cur;
    }

    let a = nexts[1];
    let b = nexts[a];
    a*b
}

fn main() -> io::Result<()> {
    let input = aoc2020::read_stdin_to_string()?;
    let input = input
        .trim()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as u8)
        .collect::<VecDeque<_>>();

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
