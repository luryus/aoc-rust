use std::{collections::HashSet, io};
use std::collections::VecDeque;

fn score(deck: &VecDeque<usize>) -> usize {
    deck.iter().rev().enumerate().map(|(i, c)| (i+1)*c).sum()
}

fn part1(p1: &VecDeque<usize>, p2: &VecDeque<usize>) -> usize {
    let mut p1 = p1.clone();
    let mut p2 = p2.clone();
    while !p1.is_empty() && !p2.is_empty() {
        let p1c = p1.pop_front().unwrap();
        let p2c = p2.pop_front().unwrap();

        if p1c > p2c {
            p1.push_back(p1c);
            p1.push_back(p2c);
        } else {
            p2.push_back(p2c);
            p2.push_back(p1c);
        }
    }

    let win = if p1.is_empty() { p2 } else { p1 };
    score(&win)
}

fn part2_game(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>) -> (bool, VecDeque<usize>) {
    let mut seen = HashSet::new();

    loop {
        if seen.contains(&(p1.clone(), p2.clone())) {
            return (true, p1);
        }
        seen.insert((p1.clone(), p2.clone()));
        if p1.is_empty() {
            return (false, p2);
        }
        if p2.is_empty() {
            return (true, p1);
        }

        let p1c = p1.pop_front().unwrap();
        let p2c = p2.pop_front().unwrap();

        if p1.len() >= p1c && p2.len() >= p2c {
            let new_p1 = p1.iter().copied().take(p1c).collect();
            let new_p2 = p2.iter().copied().take(p2c).collect();
            if part2_game(new_p1, new_p2).0 {
                p1.push_back(p1c);
                p1.push_back(p2c);
            } else {
                p2.push_back(p2c);
                p2.push_back(p1c);
            }
        } else if p1c > p2c {
            p1.push_back(p1c);
            p1.push_back(p2c);
        } else {
            p2.push_back(p2c);
            p2.push_back(p1c);
        }
    }
}

fn part2(p1: VecDeque<usize>, p2: VecDeque<usize>) -> usize {
    let (_, winning_deck) = part2_game(p1, p2);
    score(&winning_deck)
}

fn main() -> io::Result<()> {
    let input = aoc2020::read_stdin_to_string()?;
    let (pl1, pl2) = aoc2020::split_to_tuple2(&input, "\n\n").unwrap();
    let pl1 = aoc2020::read_ints_from_string(pl1.strip_prefix("Player 1:\n").unwrap()).into_iter().collect();
    let pl2 = aoc2020::read_ints_from_string(pl2.strip_prefix("Player 2:\n").unwrap()).into_iter().collect();

    let p1 = part1(&pl1, &pl2);
    println!("Part 1: {}", p1);

    let p2 = part2(pl1, pl2);
    println!("Part 2: {}", p2);

    Ok(())
}