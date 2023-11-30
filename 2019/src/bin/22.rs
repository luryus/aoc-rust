use std::{io, convert::TryInto};
use std::collections::VecDeque;
use aoc2019::read_stdin_lines;
use num_bigint::{BigInt, ToBigInt};

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
enum Op {
    Deal(usize),
    Cut(i32),
    Reverse,
}

impl Op {
    fn run(&self, mut deck: VecDeque<usize>) -> VecDeque<usize> {
        match self {
            Op::Deal(incr) => {
                let mut new = VecDeque::new();
                new.resize(deck.len(), 0);
                let mut t = 0;
                while let Some(c) = deck.pop_front() {
                    new[t] = c;
                    t = (t + *incr) % new.len();
                }
                new
            },
            Op::Cut(offset) => {
                if *offset > 0 {
                    deck.rotate_left(*offset as usize);
                } else {
                    deck.rotate_right(offset.abs() as usize);
                }
                deck
            },
            Op::Reverse => {
                for i in 0..(deck.len() / 2) {
                    let j = deck.len() - 1 - i;
                    if i != j {
                        deck.swap(i, j);
                    }
                }
                deck
            },
        }
    }
}

fn run1(ops: &Vec<Op>) {
    let mut deck = (0..10007).collect::<VecDeque<_>>();

    for op in ops {
        deck = op.run(deck);
    }
    
    let pos = deck.iter().position(|c| *c == 2019).unwrap();
    println!("Part 1: {}", pos);
}

fn run2(ops: &Vec<Op>) -> i128 {
    // This is directly adopted from reddit comments

    let deck_size = 119315717514047i128;
    let iters = 101741582076661i128;

    let mut first = 0i128;
    let mut incr = 1i128;

    // Run one iteration to check how the things change
    for op in ops {
        match op {
            Op::Reverse => {
                incr = incr * -1;
                first = (first + incr) % deck_size;
            },
            Op::Cut(x) => {
                first = (first + *x as i128 * incr) % deck_size;
            },
            Op::Deal(x) => {
                // This is the fun part
                // We have to find the new "second item" in the deck
                // The first item keeps constant
                
                // Each card i goes into the (x*i % deck_size) position
                // Use fermat's little theorem to find
                // x * 1 % deck_size == 1
                // This is the modular inverse problem. Because
                // deck_size is prime, Fermat can be used
                let x = x.to_bigint().unwrap();
                let inv = x.modpow(
                    &(deck_size - 2).to_bigint().unwrap(), 
                    &deck_size.to_bigint().unwrap());
                let inv: i128 = inv.try_into().unwrap();
                incr = (incr * inv) % deck_size;
            }
        };
    }

    // We now have the changes for one iteration
    // Now to do it a hundred billion times more

    // The first value is difficult: it depends on the current value of incr
    // However, it's always changed in the form first += n * incr
    // Because incr is only multiplied by constants, every iteration
    // can be reduced to first += x * incr, even if incr changes 
    // during the execution
    // (how do people figure these out?)
    // When this is repeated x times, combined with the incr changes below,
    // a geometric series is formed
    let first = first.to_bigint().unwrap();
    let incr = incr.to_bigint().unwrap();
    let iters = iters.to_bigint().unwrap();
    let deck_size = deck_size.to_bigint().unwrap();

    let first = 
        first 
        * (BigInt::from(1i32) - incr.modpow(&iters, &deck_size)) 
        * (BigInt::from(1i32) - &incr).modpow(&(&deck_size - BigInt::from(2)), &deck_size);

    // Incr only changes by constant values => it multiplies
    // by the same value every iteration. We can just exponentiate
    // the change
    let incr = incr.modpow(&iters, &deck_size);
    let incr: i128 = incr.try_into().unwrap();

    let res = ((first + 2020 * incr) % deck_size).try_into().unwrap();
    println!("Part 2: {}", res);
    res
}

fn parse_ops(input: Vec<String>) -> Vec<Op> {
    input.into_iter().filter_map(|s| {
        if s.starts_with("deal into") {
            Some(Op::Reverse)
        } else if s.starts_with("deal with") {
            let incr = s[20..].trim().parse().unwrap();
            Some(Op::Deal(incr))
        } else if s.starts_with("cut") {
            let offset = s[4..].trim().parse().unwrap();
            Some(Op::Cut(offset))
        } else {
            dbg!(s);
            None
        }
    }).collect()
}

fn main() -> io::Result<()> {

    let input = read_stdin_lines()?;

    let ops = parse_ops(input);

    run1(&ops);
    run2(&ops);

    Ok(())
}