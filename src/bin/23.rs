
use std::io;
use std::collections::{BTreeMap, VecDeque};

use aoc2019::{read_stdin_to_string, IntCodeComputer};


fn run1(mem: &Vec<i64>) -> i64 {

    let mut comps: Vec<IntCodeComputer> = (0..50)
        .map(|i| IntCodeComputer::new_with_input(mem, vec![i]))
        .collect();

    let mut packet_queues = Vec::with_capacity(50);
    packet_queues.resize_with(50, || VecDeque::new());

    loop {
        for (i, c) in comps.iter_mut().enumerate() {
            if let Some((x, y)) = packet_queues[i].pop_front() {
                c.inputs.push_back(x);
                c.inputs.push_back(y);
            }
            else {
                c.inputs.push_back(-1);
            }
            c.run();

            if let Some(addr) = c.outputs.pop_front() {
                let x = c.outputs.pop_front().unwrap();
                let y = c.outputs.pop_front().unwrap();

                
                if addr == 255 {
                    return y;
                }
                if (0..50).contains(&addr) {
                    packet_queues[addr as usize].push_back((x, y));
                }
            }
        }
    }
}

fn run2(mem: &Vec<i64>) -> i64 {

    let mut comps: Vec<IntCodeComputer> = (0..50)
        .map(|i| IntCodeComputer::new_with_input(mem, vec![i]))
        .collect();

    let mut packet_queues = Vec::with_capacity(50);
    packet_queues.resize_with(50, || VecDeque::new());

    let mut nat_packet = None;
    let mut last_nat_y = None;
    
    for iter in 0.. {
        let mut all_idle = true;
        for (i, c) in comps.iter_mut().enumerate() {
            if let Some((x, y)) = packet_queues[i].pop_front() {
                c.inputs.push_back(x);
                c.inputs.push_back(y);
                all_idle = false;
            }
            else {
                c.inputs.push_back(-1);
            }
            c.run();

            if let Some(addr) = c.outputs.pop_front() {
                all_idle = false;
                let x = c.outputs.pop_front().unwrap();
                let y = c.outputs.pop_front().unwrap();

                
                if addr == 255 {
                    nat_packet = match nat_packet {
                        Some((nat_iter, _, _)) if nat_iter <= iter => {
                            Some((iter, x, y))
                        },
                        None => Some((iter, x, y)),
                        _ => nat_packet
                    }
                }
                if (0..50).contains(&addr) {
                    packet_queues[addr as usize].push_back((x, y));
                }
            }
        }

        if all_idle {
            if let Some((_, x, y)) = nat_packet {
                packet_queues[0].push_back((x, y));

                if let Some(last_y) = last_nat_y {
                    if last_y == y {
                        return y;
                    }
                }

                last_nat_y = Some(y);
            }
        }
    }

    0
}

fn main() -> io::Result<()> {

    let input = read_stdin_to_string()?;
    let mem: Vec<i64> = input.trim()
        .split(",")
        .map(|i| i.parse().unwrap())
        .collect();

    let res1 = run1(&mem);
    println!("Part 1: {}", res1);
    let res2 = run2(&mem);
    println!("Part 2: {}", res2);

    Ok(())
} 