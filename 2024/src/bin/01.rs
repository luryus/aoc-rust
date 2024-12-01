use std::{collections::HashMap, io};

use itertools::Itertools;

fn part1(input: &(Vec<usize>, Vec<usize>)) -> usize {
    input
        .0
        .iter()
        .sorted()
        .zip(input.1.iter().sorted())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn part2(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let mut counts: HashMap<usize, usize> = Default::default();

    for r in &input.1 {
        *counts.entry(*r).or_default() += 1;
    }

    input
        .0
        .iter()
        .map(|a| a * counts.get(a).unwrap_or(&0))
        .sum()
}

fn main() -> io::Result<()> {
    let input: (Vec<usize>, Vec<usize>) = aoclib::read_input_ints(false)?
        .into_iter()
        .tuples()
        .fold((vec![], vec![]), |(mut lc, mut rc), (l, r)| {
            lc.push(l);
            rc.push(r);
            (lc, rc)
        });

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
        let input: (Vec<usize>, Vec<usize>) =
            aoclib::read_ints_from_file(aoclib::get_test_input_file!(1), false)
                .unwrap()
                .into_iter()
                .tuples()
                .fold((vec![], vec![]), |(mut lc, mut rc), (l, r)| {
                    lc.push(l);
                    rc.push(r);
                    (lc, rc)
                });

        let p1 = part1(&input);
        assert_eq!(p1, 1660292);

        let p2 = part2(&input);
        assert_eq!(p2, 22776016);
    }
}
