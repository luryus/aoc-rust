use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    io,
};

const fn mix(val: usize, secret: usize) -> usize {
    val ^ secret
}

const fn prune(secret: usize) -> usize {
    secret % 16777216
}

fn next(secret: usize) -> usize {
    let secret = prune(mix(secret * 64, secret));
    let secret = prune(mix(secret / 32, secret));
    prune(mix(secret * 2048, secret))
}

fn seq(mut x: usize) -> (Vec<i8>, Vec<i8>) {
    let first = (x % 10) as i8;
    let mut res = Vec::with_capacity(200);
    for _ in 0..2000 {
        x = next(x);
        res.push((x % 10) as i8);
    }

    let changes = std::iter::once(first).chain(res
        .iter().copied())
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect();

    (res, changes)
}

fn part1(input: &[usize]) -> usize {
    input
        .iter()
        .copied()
        .map(|mut x| {
            for _i in 0..2000 {
                x = next(x);
            }
            x
        })
        .sum::<usize>()
}

fn part2(input: &[usize]) -> usize {
    let x: Vec<_> = input.iter().map(|n| seq(*n)).collect();

    let mut scores: HashMap<(i8, i8, i8, i8), usize> = Default::default();

    let mut vis = HashSet::new();
    for (p, d) in &x {
        assert!(p.len() == 2000);
        assert!(d.len() == 2000);
        vis.clear();
        for (i, w) in d.iter().copied().tuple_windows().enumerate() {
            if vis.insert(w) {
                let score = p[i + 3];
                *scores.entry(w).or_default() += score as usize;
            }
        }
    }

    let (_, v) = scores.into_iter().max_by_key(|(_, v)| *v).unwrap();

    v
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_ints(false)?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_ints_from_file(aoclib::get_test_input_file!(22), false).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, 19822877190);

        let p2 = part2(&input);
        assert_eq!(p2, 2277);
    }
}
